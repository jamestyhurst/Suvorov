//! Rune host. Canonical scripting language for the engine (James, 2026-10-03).
//!
//! A bound script must define `pub fn on_fire(year, month, day)` and return a
//! string. The host compiles that body when it is bound and calls it when a
//! scheduled `Effect::RunScript` fires. Stdio is off: no `println`.

use std::sync::{Arc, OnceLock};

use rune::{Context, Diagnostics, Source, Sources, Vm};

use crate::error::{Error, Result};

static RUNTIME: OnceLock<Arc<rune::runtime::RuntimeContext>> = OnceLock::new();

fn runtime() -> Result<Arc<rune::runtime::RuntimeContext>> {
    if let Some(existing) = RUNTIME.get() {
        return Ok(Arc::clone(existing));
    }
    let context = Context::with_config(false)
        .map_err(|_| Error::Script("Rune context failed".to_string()))?;
    let built = context
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
        let context = Context::with_config(false)
            .map_err(|_| Error::Script("Rune context failed".to_string()))?;
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
        let unit = Arc::new(unit);
        let mut vm = Vm::new(runtime()?, Arc::clone(&unit));
        vm.call(["on_fire"], (0i64, 0i64, 0i64)).map_err(|_| {
            Error::Script(format!(
                "Rune script {name} must define on_fire(year, month, day) returning a string"
            ))
        })?;
        Ok(Self {
            body: body.to_string(),
            unit,
        })
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn call_on_fire(&self, year: i32, month: i32, day: i32) -> Result<String> {
        let mut vm = Vm::new(runtime()?, Arc::clone(&self.unit));
        let value = vm
            .call(["on_fire"], (year as i64, month as i64, day as i64))
            .map_err(|_| Error::Script("Rune on_fire failed".to_string()))?;
        rune::from_value(value).map_err(|_| Error::Script("Rune on_fire must return a string".to_string()))
    }
}
