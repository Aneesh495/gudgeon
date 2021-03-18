//! Tests for Autonomous Trials, Bilge Safety, and DVL Subsystems

use gudgeon::lol::{run_file, LolVM};
use std::path::Path;

#[test]
fn test_execute_turning_trial_script() {
    let mut vm = LolVM::new();
    let res = run_file(Path::new("scripts/missions/turning_trial.lol"), &mut vm).expect("Failed turning_trial");
    assert!(!res.is_noob());
}

#[test]
fn test_execute_zigzag_trial_script() {
    let mut vm = LolVM::new();
    let res = run_file(Path::new("scripts/missions/zigzag_trial.lol"), &mut vm).expect("Failed zigzag_trial");
    assert!(!res.is_noob());
}

#[test]
fn test_execute_crash_stop_trial_script() {
    let mut vm = LolVM::new();
    let res = run_file(Path::new("scripts/missions/crash_stop_trial.lol"), &mut vm).expect("Failed crash_stop");
    assert!(!res.is_noob());
}

#[test]
fn test_execute_bilge_monitor_script() {
    let mut vm = LolVM::new();
    let res = run_file(Path::new("scripts/safety/bilge_monitor.lol"), &mut vm).expect("Failed bilge_monitor");
    assert!(!res.is_noob());
}

#[test]
fn test_execute_fire_suppression_script() {
    let mut vm = LolVM::new();
    let res = run_file(Path::new("scripts/safety/fire_suppression.lol"), &mut vm).expect("Failed fire_suppression");
    assert!(!res.is_noob());
}

#[test]
fn test_execute_dvl_script() {
    let mut vm = LolVM::new();
    let res = run_file(Path::new("scripts/sensors/doppler_velocity_log.lol"), &mut vm).expect("Failed dvl");
    assert!(!res.is_noob());
}

#[test]
fn test_execute_geofence_script() {
    let mut vm = LolVM::new();
    let res = run_file(Path::new("scripts/navigation/geofence_polygon.lol"), &mut vm).expect("Failed geofence");
    assert!(!res.is_noob());
}
