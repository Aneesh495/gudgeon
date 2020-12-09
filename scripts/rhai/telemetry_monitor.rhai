// Gudgeon Telemetry Monitor and Register Poller in Rhai
// Monitors register bank, verifies sensor validity, and detects alarm states

fn check_telemetry_health() {
    let status_reg = read_register(0);
    let fault_reg = read_register(3);
    let sog_knots = read_register(19);
    let heading_deg = read_register(21);

    let health = #{
        "system_status": status_reg,
        "fault_flags": fault_reg,
        "sog_knots": sog_knots,
        "heading_deg": heading_deg,
        "alarms_present": fault_reg > 0.0,
        "healthy": fault_reg == 0.0
    };

    health
}

fn simulate_fault_recovery() {
    // Write test fault flag
    write_register(3, 1.0);
    let h1 = check_telemetry_health();
    if !h1.alarms_present { return false; }

    // Clear fault register
    write_register(3, 0.0);
    let h2 = check_telemetry_health();
    if h2.alarms_present { return false; }

    true
}

simulate_fault_recovery()
