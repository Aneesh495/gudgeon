//! End-to-End Execution Tests for the LOLCODE Autonomous Navigation Suite

use gudgeon::lol::{run_file, LolVM};
use std::path::Path;

#[test]
fn test_execute_haversine_script() {
    let mut vm = LolVM::new();
    let path = Path::new("scripts/navigation/haversine.lol");
    assert!(path.exists(), "haversine.lol must exist");
    let res = run_file(path, &mut vm).expect("Failed to execute haversine.lol");
    assert!(!res.is_noob(), "Script should execute cleanly");
}

#[test]
fn test_execute_los_script() {
    let mut vm = LolVM::new();
    let path = Path::new("scripts/navigation/line_of_sight.lol");
    assert!(path.exists());
    let res = run_file(path, &mut vm).expect("Failed to execute line_of_sight.lol");
    assert!(!res.is_noob());
}

#[test]
fn test_execute_rule14_head_on_script() {
    let mut vm = LolVM::new();
    let path = Path::new("scripts/colregs/rule14_head_on.lol");
    assert!(path.exists());
    let res = run_file(path, &mut vm).expect("Failed to execute rule14_head_on.lol");
    assert!(!res.is_noob());
}

#[test]
fn test_execute_pod_allocator_script() {
    let mut vm = LolVM::new();
    let path = Path::new("scripts/propulsion/pod_allocator.lol");
    assert!(path.exists());
    let res = run_file(path, &mut vm).expect("Failed to execute pod_allocator.lol");
    assert!(!res.is_noob());
}

#[test]
fn test_execute_watchdog_fdir_script() {
    let mut vm = LolVM::new();
    let path = Path::new("scripts/safety/watchdog_fdir.lol");
    assert!(path.exists());
    let res = run_file(path, &mut vm).expect("Failed to execute watchdog_fdir.lol");
    assert!(!res.is_noob());
}

#[test]
fn test_execute_channel_transit_script() {
    let mut vm = LolVM::new();
    let path = Path::new("scripts/missions/channel_transit.lol");
    assert!(path.exists());
    let res = run_file(path, &mut vm).expect("Failed to execute channel_transit.lol");
    assert!(!res.is_noob());
}
