//! Rune host. Canonical scripting language for the engine (James, 2026-10-03).
//!
//! Binding is two directions. `bind_script` stores a named Rune body on the
//! world. The world API binds Rust functions into that script under `world::`.
//! A script can read the date. It cannot marry anyone unless Marriage is on.

use std::cell::RefCell;
use std::sync::{Arc, OnceLock};

use rune::{Context, Diagnostics, Module, Source, Sources, Vm};

use crate::error::{Error, Result};
static RUNTIME: OnceLock<Arc<rune::runtime::RuntimeContext>> = OnceLock::new();

thread_local! {
    static VIEW: RefCell<Option<ScriptView>> = const { RefCell::new(None) };
    static COMMAND: RefCell<ScriptCommand> = const { RefCell::new(ScriptCommand { marry: None }) };
}

/// What a script may see while `on_fire` runs. Not the whole world.
#[derive(Clone, Debug)]
pub struct ScriptView {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub marriage_enabled: bool,
    pub person_count: u32,
}

/// What a script asked the world to do. The world applies this after the call.
#[derive(Clone, Debug, Default)]
pub struct ScriptCommand {
    pub marry: Option<(u32, u32)>,
}

fn date_text() -> String {
    VIEW.with(|slot| match slot.borrow().as_ref() {
        Some(view) => format!("{}-{}-{}", view.year, view.month, view.day),
        None => "no-world".to_string(),
    })
}

fn contract_marriage(a: i64, b: i64) -> String {
    let allowed = VIEW.with(|slot| {
        let slot = slot.borrow();
        let Some(view) = slot.as_ref() else {
            return false;
        };
        view.marriage_enabled
            && a >= 0
            && b >= 0
            && (a as u32) < view.person_count
            && (b as u32) < view.person_count
            && a != b
    });
    if !allowed {
        return "refused".to_string();
    }
    COMMAND.with(|slot| slot.borrow_mut().marry = Some((a as u32, b as u32)));
    "ok".to_string()
}

fn context() -> Result<Context> {
    let mut context = Context::with_config(false)
        .map_err(|_| Error::Script("Rune context failed".to_string()))?;
    let mut module = Module::with_item(["world"])
        .map_err(|_| Error::Script("Rune world module failed".to_string()))?;
    module
        .function("date_text", date_text)
        .build()
        .map_err(|_| Error::Script("Rune world binding failed".to_string()))?;
    module
        .function("contract_marriage", contract_marriage)
        .build()
        .map_err(|_| Error::Script("Rune world binding failed".to_string()))?;
    context
        .install(module)
        .map_err(|_| Error::Script("Rune world module failed".to_string()))?;
    Ok(context)
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

    /// Call `on_fire`. The script sees `view` through `world::` and may ask for one marriage.
    pub fn call_on_fire(&self, view: ScriptView) -> Result<(String, ScriptCommand)> {
        let year = view.year as i64;
        let month = view.month as i64;
        let day = view.day as i64;
        VIEW.with(|slot| *slot.borrow_mut() = Some(view));
        COMMAND.with(|slot| *slot.borrow_mut() = ScriptCommand { marry: None });
        let mut vm = Vm::new(runtime()?, Arc::clone(&self.unit));
        let called = vm.call(["on_fire"], (year, month, day));
        let command = COMMAND.with(|slot| slot.borrow().clone());
        VIEW.with(|slot| *slot.borrow_mut() = None);
        let value = called.map_err(|_| Error::Script("Rune on_fire failed".to_string()))?;
        let text = rune::from_value(value)
            .map_err(|_| Error::Script("Rune on_fire must return a string".to_string()))?;
        Ok((text, command))
    }
}
