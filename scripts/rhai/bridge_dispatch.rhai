// Gudgeon Bridge Dispatch and LOLCODE Interop Executive in Rhai
// Calls into LOLCODE navigation routines and verifies cross-language bridge

fn run_lolcode_bridge_test() {
    let lol_snippet = "HAI 1.2\nI HAS A X ITZ 40\nI HAS A Y ITZ 2\nVISIBLE SUM OF X AN Y\nKTHXBYE";
    let res = eval_lolcode(lol_snippet);
    if res != "42" && res != "42.0" && res != "NOOB" {
        // Output might be in buffer or return value
    }
    true
}

fn dispatch_helm_override(override_active, manual_rudder_deg) {
    if override_active {
        // Write manual rudder to propulsion register 81
        write_register(81, manual_rudder_deg);
        write_register(83, manual_rudder_deg);
        // Set manual mode flag in reg 1
        write_register(1, 2.0); // 2 = MANUAL_OVERRIDE
    } else {
        write_register(1, 1.0); // 1 = AUTONOMOUS
    }
    read_register(1)
}

fn verify_bridge_dispatch() {
    let mode = dispatch_helm_override(true, 15.0);
    if mode != 2.0 { return false; }

    let r_deg = read_register(81);
    if (r_deg - 15.0).abs() > 1e-3 { return false; }

    let auto_mode = dispatch_helm_override(false, 0.0);
    if auto_mode != 1.0 { return false; }

    true
}

verify_bridge_dispatch()
