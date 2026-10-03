//! Rune host. Canonical scripting language for the engine (James, 2026-10-03).
//!
//! Binding is two directions. `bind_script` stores a named Rune body on the
//! world. The world API binds Rust functions into that script under `world::`.
//! Writes are asks. The world applies them after `on_fire` returns, and only
//! if that game enabled the feature.

use std::cell::RefCell;
use std::sync::{Arc, OnceLock};

use rune::{Context, Diagnostics, Module, Source, Sources, Vm};

use crate::error::{Error, Result};

static RUNTIME: OnceLock<Arc<rune::runtime::RuntimeContext>> = OnceLock::new();

thread_local! {
    static VIEW: RefCell<Option<ScriptView>> = const { RefCell::new(None) };
    static ASKS: RefCell<Vec<ScriptAsk>> = const { RefCell::new(Vec::new()) };
    static NEXT_TITLE: RefCell<u32> = const { RefCell::new(0) };
}

/// What a script may see while `on_fire` runs. Not the whole world.
#[derive(Clone, Debug)]
pub struct ScriptView {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub marriage_enabled: bool,
    pub titles_enabled: bool,
    pub inheritance_enabled: bool,
    pub person_count: u32,
    pub location_count: u32,
    pub polity_count: u32,
    pub title_count: u32,
    pub names: Vec<String>,
    pub alive: Vec<bool>,
}

/// One ask a script made. Applied in order after `on_fire` returns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScriptAsk {
    Marry(u32, u32),
    GrantTitle { name: String, holder: u32 },
    DesignateHeir { title_id: u32, heir: u32 },
    MovePerson { person_id: u32, location_id: u32 },
    Kill(u32),
    SetAllegiance { person_id: u32, polity_id: u32 },
}

fn view_ok(check: impl FnOnce(&ScriptView) -> bool) -> bool {
    VIEW.with(|slot| slot.borrow().as_ref().is_some_and(check))
}

fn in_range(id: i64, len: u32) -> Option<u32> {
    if id >= 0 && (id as u32) < len {
        Some(id as u32)
    } else {
        None
    }
}

fn date_text() -> String {
    VIEW.with(|slot| match slot.borrow().as_ref() {
        Some(view) => format!("{}-{}-{}", view.year, view.month, view.day),
        None => "no-world".to_string(),
    })
}

fn person_name(id: i64) -> String {
    VIEW.with(|slot| {
        let slot = slot.borrow();
        let Some(view) = slot.as_ref() else {
            return "none".to_string();
        };
        in_range(id, view.person_count)
            .and_then(|index| view.names.get(index as usize))
            .cloned()
            .unwrap_or_else(|| "none".to_string())
    })
}

fn is_alive(id: i64) -> String {
    VIEW.with(|slot| {
        let slot = slot.borrow();
        let Some(view) = slot.as_ref() else {
            return "none".to_string();
        };
        match in_range(id, view.person_count).and_then(|index| view.alive.get(index as usize)) {
            Some(true) => "yes".to_string(),
            Some(false) => "no".to_string(),
            None => "none".to_string(),
        }
    })
}

fn contract_marriage(a: i64, b: i64) -> String {
    let pair = VIEW.with(|slot| {
        let slot = slot.borrow();
        let Some(view) = slot.as_ref() else {
            return None;
        };
        if !view.marriage_enabled {
            return None;
        }
        Some((in_range(a, view.person_count)?, in_range(b, view.person_count)?))
    });
    let Some((a, b)) = pair.filter(|(a, b)| a != b) else {
        return "refused".to_string();
    };
    ASKS.with(|slot| slot.borrow_mut().push(ScriptAsk::Marry(a, b)));
    "ok".to_string()
}

fn grant_title(holder: i64, name: String) -> i64 {
    if name.is_empty() || !view_ok(|view| view.titles_enabled) {
        return -1;
    }
    let holder = VIEW.with(|slot| {
        slot.borrow()
            .as_ref()
            .and_then(|view| in_range(holder, view.person_count))
    });
    let Some(holder) = holder else {
        return -1;
    };
    let title_id = NEXT_TITLE.with(|slot| {
        let id = *slot.borrow();
        *slot.borrow_mut() += 1;
        id
    });
    ASKS.with(|slot| {
        slot.borrow_mut().push(ScriptAsk::GrantTitle {
            name,
            holder,
        })
    });
    title_id as i64
}

fn designate_heir(title_id: i64, heir: i64) -> String {
    let pair = VIEW.with(|slot| {
        let slot = slot.borrow();
        let Some(view) = slot.as_ref() else {
            return None;
        };
        if !view.inheritance_enabled {
            return None;
        }
        let heir = in_range(heir, view.person_count)?;
        if title_id < 0 {
            return None;
        }
        Some((title_id as u32, heir))
    });
    let Some((title_id, heir)) = pair else {
        return "refused".to_string();
    };
    ASKS.with(|slot| {
        slot.borrow_mut()
            .push(ScriptAsk::DesignateHeir { title_id, heir })
    });
    "ok".to_string()
}

fn move_person(person_id: i64, location_id: i64) -> String {
    let pair = VIEW.with(|slot| {
        let slot = slot.borrow();
        let Some(view) = slot.as_ref() else {
            return None;
        };
        Some((
            in_range(person_id, view.person_count)?,
            in_range(location_id, view.location_count)?,
        ))
    });
    let Some((person_id, location_id)) = pair else {
        return "refused".to_string();
    };
    ASKS.with(|slot| {
        slot.borrow_mut()
            .push(ScriptAsk::MovePerson { person_id, location_id })
    });
    "ok".to_string()
}

fn kill_person(person_id: i64) -> String {
    let person_id = VIEW.with(|slot| {
        slot.borrow()
            .as_ref()
            .and_then(|view| in_range(person_id, view.person_count))
    });
    let Some(person_id) = person_id else {
        return "refused".to_string();
    };
    ASKS.with(|slot| slot.borrow_mut().push(ScriptAsk::Kill(person_id)));
    "ok".to_string()
}

fn set_allegiance(person_id: i64, polity_id: i64) -> String {
    let pair = VIEW.with(|slot| {
        let slot = slot.borrow();
        let Some(view) = slot.as_ref() else {
            return None;
        };
        Some((
            in_range(person_id, view.person_count)?,
            in_range(polity_id, view.polity_count)?,
        ))
    });
    let Some((person_id, polity_id)) = pair else {
        return "refused".to_string();
    };
    ASKS.with(|slot| {
        slot.borrow_mut()
            .push(ScriptAsk::SetAllegiance { person_id, polity_id })
    });
    "ok".to_string()
}

fn context() -> Result<Context> {
    let mut context = Context::with_config(false)
        .map_err(|_| Error::Script("Rune context failed".to_string()))?;
    let mut module = Module::with_item(["world"])
        .map_err(|_| Error::Script("Rune world module failed".to_string()))?;
    for (name, install) in [
        ("date_text", install_date as fn(&mut Module) -> Result<()>),
        ("person_name", install_person_name),
        ("is_alive", install_is_alive),
        ("contract_marriage", install_marriage),
        ("grant_title", install_grant),
        ("designate_heir", install_heir),
        ("move_person", install_move),
        ("kill_person", install_kill),
        ("set_allegiance", install_allegiance),
    ] {
        let _ = name;
        install(&mut module)?;
    }
    context
        .install(module)
        .map_err(|_| Error::Script("Rune world module failed".to_string()))?;
    Ok(context)
}

fn install_date(module: &mut Module) -> Result<()> {
    module
        .function("date_text", date_text)
        .build()
        .map_err(|_| Error::Script("Rune world binding failed".to_string()))?;
    Ok(())
}
fn install_person_name(module: &mut Module) -> Result<()> {
    module
        .function("person_name", person_name)
        .build()
        .map_err(|_| Error::Script("Rune world binding failed".to_string()))?;
    Ok(())
}
fn install_is_alive(module: &mut Module) -> Result<()> {
    module
        .function("is_alive", is_alive)
        .build()
        .map_err(|_| Error::Script("Rune world binding failed".to_string()))?;
    Ok(())
}
fn install_marriage(module: &mut Module) -> Result<()> {
    module
        .function("contract_marriage", contract_marriage)
        .build()
        .map_err(|_| Error::Script("Rune world binding failed".to_string()))?;
    Ok(())
}
fn install_grant(module: &mut Module) -> Result<()> {
    module
        .function("grant_title", grant_title)
        .build()
        .map_err(|_| Error::Script("Rune world binding failed".to_string()))?;
    Ok(())
}
fn install_heir(module: &mut Module) -> Result<()> {
    module
        .function("designate_heir", designate_heir)
        .build()
        .map_err(|_| Error::Script("Rune world binding failed".to_string()))?;
    Ok(())
}
fn install_move(module: &mut Module) -> Result<()> {
    module
        .function("move_person", move_person)
        .build()
        .map_err(|_| Error::Script("Rune world binding failed".to_string()))?;
    Ok(())
}
fn install_kill(module: &mut Module) -> Result<()> {
    module
        .function("kill_person", kill_person)
        .build()
        .map_err(|_| Error::Script("Rune world binding failed".to_string()))?;
    Ok(())
}
fn install_allegiance(module: &mut Module) -> Result<()> {
    module
        .function("set_allegiance", set_allegiance)
        .build()
        .map_err(|_| Error::Script("Rune world binding failed".to_string()))?;
    Ok(())
}

fn runtime() -> Result<Arc<rune::runtime::RuntimeContext>> {
    if let Some(existing) = RUNTIME.get() {
        return Ok(Arc::clone(existing));
    }
    let built = context()?
        .runtime()
        .map_err(|_| Error::Script("Rune runtime failed".to_string()))?;
    let _ = RUNTIME.set(Arc::new(built));
    Ok(Arc::clone(RUNTIME.get().expect("runtime just set")))
}

pub struct CompiledScript {
    body: String,
    unit: Arc<rune::Unit>,
}

impl CompiledScript {
    pub fn compile(name: &str, body: &str) -> Result<Self> {
        let context = context()?;
        let mut sources = Sources::new();
        sources
            .insert(
                Source::memory(body)
                    .map_err(|_| Error::Script(format!("Rune source rejected for {name}")))?,
            )
            .map_err(|_| Error::Script(format!("Rune source rejected for {name}")))?;
        let mut diagnostics = Diagnostics::new();
        let built = rune::prepare(&mut sources)
            .with_context(&context)
            .with_diagnostics(&mut diagnostics)
            .build();
        if !diagnostics.is_empty() {
            return Err(Error::Script(format!("Rune rejected {name}")));
        }
        let unit = built.map_err(|_| Error::Script(format!("Rune rejected {name}")))?;
        Ok(Self {
            body: body.to_string(),
            unit: Arc::new(unit),
        })
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    /// Call `on_fire`. Asks are collected and returned; the world applies them.
    pub fn call_on_fire(&self, view: ScriptView) -> Result<(String, Vec<ScriptAsk>)> {
        let year = view.year as i64;
        let month = view.month as i64;
        let day = view.day as i64;
        NEXT_TITLE.with(|slot| *slot.borrow_mut() = view.title_count);
        VIEW.with(|slot| *slot.borrow_mut() = Some(view));
        ASKS.with(|slot| slot.borrow_mut().clear());
        let mut vm = Vm::new(runtime()?, Arc::clone(&self.unit));
        let called = vm.call(["on_fire"], (year, month, day));
        let asks = ASKS.with(|slot| slot.borrow().clone());
        VIEW.with(|slot| *slot.borrow_mut() = None);
        let value = called.map_err(|_| Error::Script("Rune on_fire failed".to_string()))?;
        let text = rune::from_value(value)
            .map_err(|_| Error::Script("Rune on_fire must return a string".to_string()))?;
        Ok((text, asks))
    }
}
