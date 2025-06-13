use std::path::Path;
use rhai::Engine;
use crate::chimp::{ChimpConnection, Key};
use crate::osc::OscConnection;

pub struct MacroExecutor {
    engine: Engine,
}

impl MacroExecutor {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        
        engine.register_fn("press_key", |key: Key| {
            
        });
        engine.register_fn("hold_key", |key: Key| {
            
        });
        
        Self {
            engine,
        }
    }
    
    pub fn execute_macro(&self, path: &Path) -> color_eyre::Result<()> {
        self.engine.run_file(path.into())
            .map_err(|err| color_eyre::eyre::eyre!("Unable to execute macro: {err:?}"))?;
        
        Ok(())
    }
}

pub fn execute_macro(path: &Path, conn: &dyn ChimpConnection) -> color_eyre::Result<()> {
    let mut engine = Engine::new();

    engine.register_fn("press_key", move |key: Key| {
        conn.press_key(key);
    });
    engine.register_fn("hold_key", move |key: Key| {
        conn.hold_key(key);
    });

    engine.run_file(path.into())
        .map_err(|err| color_eyre::eyre::eyre!("Unable to execute macro: {err:?}"))?;

    Ok(())
}