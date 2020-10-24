//! Tests for COLREGS Situation Classification and Collision Calculations

use gudgeon::colregs::{ColregsEngine, ColregsSituation};

#[test]
fn test_cpa_tcpa_head_on() {
    // Two vessels approaching each other head on along the north-south axis
    // Own vessel at (0, 0), moving north at 10 knots (cog = 0)
    // Target at (0.5, 0), moving south at 10 knots (cog = 180)
    let (cpa, tcpa, range, bearing) = ColregsEngine::evaluate_cpa_tcpa(
        37.0, -122.0, 10.0, 0.0,
        37.5, -122.0, 10.0, 180.0,
    );
    assert!((range - 30.0).abs() < 1.0, "Range should be approx 30 nm");
    assert!(cpa < 0.01, "CPA should be zero for direct collision course");
    assert!(tcpa > 0.0, "TCPA should be positive in the future");
    assert!((bearing - 0.0).abs() < 1.0 || (bearing - 360.0).abs() < 1.0);
}

#[test]
fn test_classify_head_on_encounter() {
    let situation = ColregsEngine::classify_encounter(
        0.0,   // own heading north
        0.0,   // target directly ahead bearing 000
        180.0, // target heading south 180
        2.5,   // range 2.5 nm
        0.05,  // CPA 0.05 nm
        300.0, // TCPA 300s
        0.8,
        900.0,
    );
    assert_eq!(situation, ColregsSituation::HeadOn);

    let evasion_heading = ColregsEngine::compute_evasion_heading(0.0, situation, 0.05, 0.8);
    // Head on requires starboard turn (+35 deg)
    assert!((evasion_heading - 35.0).abs() < 1e-4);
}

#[test]
fn test_classify_crossing_give_way() {
    let situation = ColregsEngine::classify_encounter(
        0.0,   // own heading north
        45.0,  // target bearing 045 (starboard bow)
        270.0, // target heading west 270
        2.0,   // range 2.0 nm
        0.1,   // CPA 0.1 nm
        400.0, // TCPA 400s
        0.8,
        900.0,
    );
    assert_eq!(situation, ColregsSituation::CrossingGiveWay);

    let evasion_heading = ColregsEngine::compute_evasion_heading(0.0, situation, 0.1, 0.8);
    // Give way turns to starboard to pass astern
    assert!(evasion_heading > 20.0 && evasion_heading <= 50.0);
}

#[test]
fn test_classify_crossing_stand_on() {
    let situation = ColregsEngine::classify_encounter(
        0.0,    // own heading north
        315.0,  // target bearing 315 (-45 deg port bow)
        90.0,   // target heading east 090
        2.0,
        0.1,
        400.0,
        0.8,
        900.0,
    );
    assert_eq!(situation, ColregsSituation::CrossingStandOn);

    let evasion_heading = ColregsEngine::compute_evasion_heading(0.0, situation, 0.1, 0.8);
    // Stand on maintains course
    assert_eq!(evasion_heading, 0.0);
}
