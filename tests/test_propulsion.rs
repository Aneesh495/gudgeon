//! Tests for Dual-Pod and Bow Thruster Allocation

use gudgeon::propulsion::ThrustAllocator;

#[test]
fn test_thrust_allocation_pure_surge() {
    let allocator = ThrustAllocator::default();
    let cmd = allocator.allocate(40_000.0, 0.0, 0.0);

    // 40 kN surge should split into 20 kN left and 20 kN right pod
    assert!((cmd.pod_left_thrust_n - 20_000.0).abs() < 1e-3);
    assert!((cmd.pod_right_thrust_n - 20_000.0).abs() < 1e-3);
    assert_eq!(cmd.pod_left_angle_deg, 0.0);
    assert_eq!(cmd.pod_right_angle_deg, 0.0);
    assert_eq!(cmd.bow_thruster_thrust_n, 0.0);
}

#[test]
fn test_thrust_allocation_yaw_moment() {
    let allocator = ThrustAllocator::default();
    // Demand 16,000 Nm yaw moment (clockwise / starboard)
    let cmd = allocator.allocate(20_000.0, 0.0, 16_000.0);

    // Right pod should produce more thrust than left pod
    assert!(cmd.pod_right_thrust_n > cmd.pod_left_thrust_n);
}
