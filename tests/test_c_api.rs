//! Integration Tests for C Foreign Function Interface (FFI)

use gudgeon::c_api::*;
use std::ffi::CString;

#[test]
fn test_c_api_init_and_registers() {
    assert_eq!(gudgeon_runtime_init(), 0);
    gudgeon_reg_write(19, 18.2);
    let val = gudgeon_reg_read(19);
    assert!((val - 18.2).abs() < 1e-4);
}

#[test]
fn test_c_api_eval_lolcode() {
    assert_eq!(gudgeon_runtime_init(), 0);
    let code = CString::new("HAI 1.2\nI HAS A RES ITZ 42\nVISIBLE RES\nKTHXBYE").unwrap();
    let mut buf = vec![0u8; 128];
    let ret = unsafe { gudgeon_eval_lolcode(code.as_ptr(), buf.as_mut_ptr() as *mut _, buf.len()) };
    assert_eq!(ret, 0);
    let out_str = String::from_utf8_lossy(&buf);
    assert!(out_str.starts_with("42"));
}

#[test]
fn test_c_api_sim_step() {
    assert_eq!(gudgeon_runtime_init(), 0);
    // Apply 25kN surge thrust
    gudgeon_sim_step(25_000.0, 0.0, 0.0, 0.1);
    let mut state = CGudgeonVesselState {
        lat_deg: 0.0,
        lon_deg: 0.0,
        u_surge_mps: 0.0,
        v_sway_mps: 0.0,
        r_yaw_rate_radps: 0.0,
        heading_deg: 0.0,
        sog_knots: 0.0,
        cog_deg: 0.0,
        pod_left_thrust_n: 0.0,
        pod_left_angle_deg: 0.0,
        pod_right_thrust_n: 0.0,
        pod_right_angle_deg: 0.0,
        bow_thruster_thrust_n: 0.0,
    };
    unsafe { gudgeon_get_state(&mut state) };
    assert!(state.pod_left_thrust_n > 0.0);
    assert!(state.pod_right_thrust_n > 0.0);
}
