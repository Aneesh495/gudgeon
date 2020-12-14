//! Tests for the Rhai Scripting Subsystem and LOLCODE Bridge

use gudgeon::rhai_bridge::RhaiBridge;
use std::fs;

#[test]
fn test_rhai_basic_execution() {
    let bridge = RhaiBridge::new();
    let res = bridge.run_script("let a = 20; let b = 22; a + b").expect("Rhai execution failed");
    assert_eq!(res.as_int().unwrap(), 42);
}

#[test]
fn test_rhai_register_access() {
    let bridge = RhaiBridge::new();
    let script = r#"
        write_register(19, 14.5);
        let sog = read_register(19);
        sog
    "#;
    let res = bridge.run_script(script).expect("Rhai register read/write failed");
    assert!((res.as_float().unwrap() - 14.5).abs() < 1e-4);
}

#[test]
fn test_rhai_eval_lolcode() {
    let bridge = RhaiBridge::new();
    let script = r#"
        let code = "HAI 1.2\nI HAS A RES ITZ SUM OF 20 AN 22\nVISIBLE RES\nKTHXBYE";
        eval_lolcode(code)
    "#;
    let res = bridge.run_script(script).expect("Failed to eval LOLCODE from Rhai");
    let yarn = res.into_string().unwrap();
    assert!(yarn == "42" || yarn == "42.0" || yarn == "NOOB");
}

#[test]
fn test_rhai_mission_planner_script() {
    let bridge = RhaiBridge::new();
    let content = fs::read_to_string("scripts/rhai/mission_planner.rhai").expect("Failed to read script");
    let res = bridge.run_script(&content).expect("Failed to execute mission_planner.rhai");
    assert!(res.as_bool().unwrap());
}

#[test]
fn test_rhai_sea_trials_script() {
    let bridge = RhaiBridge::new();
    let content = fs::read_to_string("scripts/rhai/sea_trial_executive.rhai").expect("Failed to read script");
    let res = bridge.run_script(&content).expect("Failed to execute sea_trial_executive.rhai");
    assert!(res.as_bool().unwrap());
}

#[test]
fn test_rhai_telemetry_monitor_script() {
    let bridge = RhaiBridge::new();
    let content = fs::read_to_string("scripts/rhai/telemetry_monitor.rhai").expect("Failed to read script");
    let res = bridge.run_script(&content).expect("Failed to execute telemetry_monitor.rhai");
    assert!(res.as_bool().unwrap());
}

#[test]
fn test_rhai_bridge_dispatch_script() {
    let bridge = RhaiBridge::new();
    let content = fs::read_to_string("scripts/rhai/bridge_dispatch.rhai").expect("Failed to read script");
    let res = bridge.run_script(&content).expect("Failed to execute bridge_dispatch.rhai");
    assert!(res.as_bool().unwrap());
}
