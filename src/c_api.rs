//! C Foreign Function Interface (FFI) Implementation for Gudgeon

use crate::lol::{run_file, run_source, LolVM};
use crate::propulsion::ThrustAllocator;
use crate::telemetry::registers::RegisterBank;
use crate::vessel::{HydrodynamicsSimulator, VesselParams, VesselState};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::sync::{Mutex, OnceLock};

static GLOBAL_REGISTERS: OnceLock<Mutex<RegisterBank>> = OnceLock::new();
static GLOBAL_SIM: OnceLock<Mutex<HydrodynamicsSimulator>> = OnceLock::new();
static GLOBAL_ALLOCATOR: OnceLock<ThrustAllocator> = OnceLock::new();

fn get_registers() -> &'static Mutex<RegisterBank> {
    GLOBAL_REGISTERS.get_or_init(|| Mutex::new(RegisterBank::new()))
}

fn get_simulator() -> &'static Mutex<HydrodynamicsSimulator> {
    GLOBAL_SIM.get_or_init(|| {
        let params = VesselParams::default();
        let state = VesselState {
            lat_deg: 37.7749,
            lon_deg: -122.4194,
            u_surge_mps: 0.0,
            v_sway_mps: 0.0,
            r_yaw_rate_radps: 0.0,
            heading_rad: 0.0,
            sog_knots: 0.0,
            cog_deg: 0.0,
            pod_left_thrust_n: 0.0,
            pod_left_angle_deg: 0.0,
            pod_right_thrust_n: 0.0,
            pod_right_angle_deg: 0.0,
            bow_thruster_thrust_n: 0.0,
        };
        Mutex::new(HydrodynamicsSimulator::new(params, state))
    })
}

fn get_allocator() -> &'static ThrustAllocator {
    GLOBAL_ALLOCATOR.get_or_init(ThrustAllocator::default)
}

#[repr(C)]
pub struct CGudgeonVesselState {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub u_surge_mps: f64,
    pub v_sway_mps: f64,
    pub r_yaw_rate_radps: f64,
    pub heading_deg: f64,
    pub sog_knots: f64,
    pub cog_deg: f64,
    pub pod_left_thrust_n: f64,
    pub pod_left_angle_deg: f64,
    pub pod_right_thrust_n: f64,
    pub pod_right_angle_deg: f64,
    pub bow_thruster_thrust_n: f64,
}

#[no_mangle]
pub extern "C" fn gudgeon_runtime_init() -> i32 {
    let _ = get_registers();
    let _ = get_simulator();
    let _ = get_allocator();
    0
}

#[no_mangle]
pub unsafe extern "C" fn gudgeon_eval_lolcode(
    code_str: *const c_char,
    out_buf: *mut c_char,
    max_len: usize,
) -> i32 {
    if code_str.is_null() {
        return -1;
    }
    let c_str = match CStr::from_ptr(code_str).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };

    match run_source(c_str) {
        Ok((val, vm)) => {
            let output_str = if !vm.output_buffer.is_empty() {
                vm.output_buffer.join("\n")
            } else {
                val.to_yarn()
            };
            if !out_buf.is_null() && max_len > 0 {
                let bytes = output_str.as_bytes();
                let copy_len = bytes.len().min(max_len - 1);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf as *mut u8, copy_len);
                *out_buf.add(copy_len) = 0;
            }
            0
        }
        Err(_) => -3,
    }
}

#[no_mangle]
pub unsafe extern "C" fn gudgeon_exec_lolcode_file(filepath: *const c_char) -> i32 {
    if filepath.is_null() {
        return -1;
    }
    let path_str = match CStr::from_ptr(filepath).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };
    let mut vm = LolVM::new();
    match run_file(Path::new(path_str), &mut vm) {
        Ok(_) => 0,
        Err(_) => -3,
    }
}

#[no_mangle]
pub extern "C" fn gudgeon_reg_read(offset: u32) -> f64 {
    get_registers().lock().unwrap().read(offset as usize)
}

#[no_mangle]
pub extern "C" fn gudgeon_reg_write(offset: u32, value: f64) {
    get_registers().lock().unwrap().write(offset as usize, value);
}

#[no_mangle]
pub extern "C" fn gudgeon_sim_step(fx_cmd_n: f64, fy_cmd_n: f64, mz_cmd_nm: f64, dt_sec: f64) {
    let cmd = get_allocator().allocate(fx_cmd_n, fy_cmd_n, mz_cmd_nm);
    let mut sim = get_simulator().lock().unwrap();
    sim.step(&cmd, dt_sec);

    // Sync to registers
    let mut bank = get_registers().lock().unwrap();
    bank.write(16, sim.state.lat_deg);
    bank.write(17, sim.state.lon_deg);
    bank.write(19, sim.state.sog_knots);
    bank.write(20, sim.state.cog_deg);
    bank.write(21, sim.state.heading_rad.to_degrees());
    bank.write(22, sim.state.r_yaw_rate_radps.to_degrees());
    bank.write(23, sim.state.u_surge_mps);
    bank.write(24, sim.state.v_sway_mps);
    bank.write(80, cmd.pod_left_thrust_n);
    bank.write(81, cmd.pod_left_angle_deg);
    bank.write(82, cmd.pod_right_thrust_n);
    bank.write(83, cmd.pod_right_angle_deg);
    bank.write(84, cmd.bow_thruster_thrust_n);
}

#[no_mangle]
pub unsafe extern "C" fn gudgeon_get_state(out_state: *mut CGudgeonVesselState) {
    if out_state.is_null() {
        return;
    }
    let sim = get_simulator().lock().unwrap();
    (*out_state).lat_deg = sim.state.lat_deg;
    (*out_state).lon_deg = sim.state.lon_deg;
    (*out_state).u_surge_mps = sim.state.u_surge_mps;
    (*out_state).v_sway_mps = sim.state.v_sway_mps;
    (*out_state).r_yaw_rate_radps = sim.state.r_yaw_rate_radps;
    (*out_state).heading_deg = sim.state.heading_rad.to_degrees();
    (*out_state).sog_knots = sim.state.sog_knots;
    (*out_state).cog_deg = sim.state.cog_deg;
    (*out_state).pod_left_thrust_n = sim.state.pod_left_thrust_n;
    (*out_state).pod_left_angle_deg = sim.state.pod_left_angle_deg;
    (*out_state).pod_right_thrust_n = sim.state.pod_right_thrust_n;
    (*out_state).pod_right_angle_deg = sim.state.pod_right_angle_deg;
    (*out_state).bow_thruster_thrust_n = sim.state.bow_thruster_thrust_n;
}

#[no_mangle]
pub extern "C" fn gudgeon_runtime_shutdown() {
    // Graceful release
}
