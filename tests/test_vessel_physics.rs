//! Tests for 3-DOF Vessel Hydrodynamics and Steering Physics

use gudgeon::vessel::{ControlCommand, HydrodynamicsSimulator, VesselParams, VesselState};

#[test]
fn test_straight_line_acceleration() {
    let params = VesselParams::default();
    let initial_state = VesselState {
        lat_deg: 37.7749,
        lon_deg: -122.4194,
        ..Default::default()
    };
    let mut sim = HydrodynamicsSimulator::new(params, initial_state);

    let cmd = ControlCommand {
        pod_left_thrust_n: 30_000.0,
        pod_left_angle_deg: 0.0,
        pod_right_thrust_n: 30_000.0,
        pod_right_angle_deg: 0.0,
        bow_thruster_thrust_n: 0.0,
    };

    // Step for 10 seconds
    for _ in 0..100 {
        sim.step(&cmd, 0.1);
    }

    assert!(sim.state.u_surge_mps > 1.0, "Vessel should accelerate forward");
    assert!(sim.state.sog_knots > 2.0, "SOG should exceed 2 knots");
    assert!(sim.state.v_sway_mps.abs() < 0.1, "Sway should be near zero on straight course");
    assert!(sim.state.r_yaw_rate_radps.abs() < 0.01, "Yaw rate should be near zero");
}

#[test]
fn test_turning_circle_maneuver() {
    let params = VesselParams::default();
    let initial_state = VesselState {
        lat_deg: 37.7749,
        lon_deg: -122.4194,
        u_surge_mps: 5.0, // 5 m/s initial speed
        ..Default::default()
    };
    let mut sim = HydrodynamicsSimulator::new(params, initial_state);

    // Apply 20 degree starboard rudder
    let cmd = ControlCommand {
        pod_left_thrust_n: 30_000.0,
        pod_left_angle_deg: 20.0,
        pod_right_thrust_n: 30_000.0,
        pod_right_angle_deg: 20.0,
        bow_thruster_thrust_n: 0.0,
    };

    let initial_heading = sim.state.heading_rad;
    for _ in 0..200 {
        sim.step(&cmd, 0.1);
    }

    // Heading should change significantly
    let heading_delta = (sim.state.heading_rad - initial_heading).abs();
    assert!(heading_delta > 0.5, "Starboard rudder should induce significant heading turn");
    assert!(sim.state.r_yaw_rate_radps.abs() > 0.01, "Yaw rate should be non-zero");
}
