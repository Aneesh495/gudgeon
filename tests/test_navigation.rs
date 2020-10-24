//! Tests for Line-of-Sight (LOS) Navigation and Path Tracking

use gudgeon::navigation::{LosGuidance, Waypoint};

#[test]
fn test_great_circle_distance() {
    // SF to Oakland is approx 13 to 14 km
    let dist = LosGuidance::distance_meters(37.7749, -122.4194, 37.8044, -122.2712);
    assert!(dist > 12_000.0 && dist < 16_000.0, "Distance should be ~14 km, got {}", dist);
}

#[test]
fn test_los_guidance_converges_to_path() {
    let wps = vec![
        Waypoint {
            lat_deg: 37.0,
            lon_deg: -122.0,
            radius_m: 50.0,
            target_speed_knots: 12.0,
        },
        Waypoint {
            lat_deg: 37.1,
            lon_deg: -122.0,
            radius_m: 50.0,
            target_speed_knots: 12.0,
        },
    ];
    let mut guidance = LosGuidance::new(wps, 150.0);

    // Vessel is to the right (east) of the path at lon = -121.99
    // Expected desired heading should steer to port (west) of 000
    let (desired_heading, speed, done) = guidance.update(37.05, -121.99, 1.0);
    assert!(!done);
    assert_eq!(speed, 12.0);
    assert!(desired_heading > 300.0 && desired_heading < 360.0, "Desired heading must steer left to correct rightward cross track error");
}
