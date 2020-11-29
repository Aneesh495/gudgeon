//! Rhai Scripting Engine and Bi-Directional LOLCODE Interop Bridge
//! Embeds Rhai to provide dynamic high-level mission scripting, sea-trial qualification,
//! and bidirectional interop with the LOLCODE navigation subsystem.

use crate::lol::run_source;
use crate::telemetry::registers::RegisterBank;
use rhai::{Dynamic, Engine, Scope, AST};
use std::sync::{Arc, Mutex};

pub struct RhaiBridge {
    pub engine: Engine,
    pub registers: Arc<Mutex<RegisterBank>>,
}

impl Default for RhaiBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl RhaiBridge {
    pub fn new() -> Self {
        let registers = Arc::new(Mutex::new(RegisterBank::new()));
        let mut engine = Engine::new();

        // Register accessors for Rhai
        let reg_read = Arc::clone(&registers);
        engine.register_fn("read_register", move |offset: i64| -> f64 {
            let bank = reg_read.lock().unwrap();
            bank.read(offset as usize)
        });

        let reg_write = Arc::clone(&registers);
        engine.register_fn("write_register", move |offset: i64, val: f64| {
            let mut bank = reg_write.lock().unwrap();
            bank.write(offset as usize, val);
        });

        // LOLCODE evaluation from Rhai
        engine.register_fn("eval_lolcode", |code: &str| -> String {
            match run_source(code) {
                Ok((val, _)) => val.to_yarn(),
                Err(e) => format!("ERROR: {}", e),
            }
        });

        // Maritime navigation math in Rhai
        engine.register_fn("haversine_nm", |lat1: f64, lon1: f64, lat2: f64, lon2: f64| -> f64 {
            let dlat = (lat2 - lat1).to_radians();
            let dlon = (lon2 - lon1).to_radians();
            let a = (dlat / 2.0).sin().powi(2)
                + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
            let c = 2.0 * a.sqrt().atan2((1.0 - a).max(0.0).sqrt());
            3440.065 * c
        });

        engine.register_fn("bearing_deg", |lat1: f64, lon1: f64, lat2: f64, lon2: f64| -> f64 {
            let l1 = lat1.to_radians();
            let l2 = lat2.to_radians();
            let dlon = (lon2 - lon1).to_radians();
            let y = dlon.sin() * l2.cos();
            let x = l1.cos() * l2.sin() - l1.sin() * l2.cos() * dlon.cos();
            let mut b = y.atan2(x).to_degrees();
            if b < 0.0 {
                b += 360.0;
            }
            b
        });

        Self { engine, registers }
    }

    pub fn compile(&self, script: &str) -> Result<AST, String> {
        self.engine.compile(script).map_err(|e| format!("Rhai compilation error: {}", e))
    }

    pub fn run_script(&self, script: &str) -> Result<Dynamic, String> {
        let mut scope = Scope::new();
        self.engine.eval_with_scope::<Dynamic>(&mut scope, script).map_err(|e| format!("Rhai execution error: {}", e))
    }
}
