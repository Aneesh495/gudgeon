//! Tests for LOLCODE Language Engine Subsystem

use gudgeon::lol::run_source;

#[test]
fn test_basic_arithmetic() {
    let code = r#"
    HAI 1.2
    I HAS A X ITZ 10
    I HAS A Y ITZ 20
    I HAS A Z ITZ SUM OF X AN Y
    VISIBLE Z
    KTHXBYE
    "#;
    let (_val, vm) = run_source(code).expect("Execution failed");
    assert_eq!(vm.output_buffer, vec!["30"]);
    assert_eq!(vm.globals.get("Z"), None); // local inside main block
}

#[test]
fn test_nested_arithmetic() {
    let code = r#"
    HAI 1.2
    I HAS A RESULT ITZ PRODUKT OF SUM OF 5 AN 5 AN DIFF OF 10 AN 6
    VISIBLE RESULT
    KTHXBYE
    "#;
    let (_, vm) = run_source(code).expect("Execution failed");
    assert_eq!(vm.output_buffer, vec!["40"]);
}

#[test]
fn test_functions() {
    let code = r#"
    HAI 1.2
    HOW IZ I COMPUTE_KINETIC YR MASS AN YR VEL
        I HAS A V_SQ ITZ PRODUKT OF VEL AN VEL
        I HAS A HALF_M ITZ QUOSHUNT OF MASS AN 2.0
        FOUND YR PRODUKT OF HALF_M AN V_SQ
    IF U SAY SO

    I HAS A KE ITZ I IZ COMPUTE_KINETIC YR 10.0 AN YR 4.0 MKAY
    VISIBLE KE
    KTHXBYE
    "#;
    let (_, vm) = run_source(code).expect("Execution failed");
    assert_eq!(vm.output_buffer, vec!["80.0"]);
}

#[test]
fn test_conditional() {
    let code = r#"
    HAI 1.2
    I HAS A FLAG ITZ WIN
    FLAG
    O RLY?
        YA RLY
            VISIBLE "CONDITION_MET"
        NO WAI
            VISIBLE "CONDITION_FAILED"
    OIC
    KTHXBYE
    "#;
    let (_, vm) = run_source(code).expect("Execution failed");
    assert_eq!(vm.output_buffer, vec!["CONDITION_MET"]);
}

#[test]
fn test_loop_counter() {
    let code = r#"
    HAI 1.2
    I HAS A SUM ITZ 0
    I HAS A I ITZ 1
    IM IN YR LOOP UPPIN YR I TIL BOTH SAEM I AN 6
        SUM R SUM OF SUM AN I
    IM OUTTA YR LOOP
    VISIBLE SUM
    KTHXBYE
    "#;
    let (_, vm) = run_source(code).expect("Execution failed");
    assert_eq!(vm.output_buffer, vec!["15"]);
}

#[test]
fn test_haversine_and_bearing_builtins() {
    let code = r#"
    HAI 1.2
    I HAS A DIST ITZ I IZ NAV_HAVERSINE YR 37.7749 AN YR -122.4194 AN YR 34.0522 AN YR -118.2437 MKAY
    I HAS A BRG ITZ I IZ NAV_BEARING YR 37.7749 AN YR -122.4194 AN YR 34.0522 AN YR -118.2437 MKAY
    VISIBLE DIST
    VISIBLE BRG
    KTHXBYE
    "#;
    let (_, vm) = run_source(code).expect("Execution failed");
    assert_eq!(vm.output_buffer.len(), 2);
    let dist: f64 = vm.output_buffer[0].parse().unwrap();
    let brg: f64 = vm.output_buffer[1].parse().unwrap();
    // Distance SF to LA is approximately 300 to 320 nautical miles
    assert!(dist > 290.0 && dist < 330.0, "Expected dist in [290, 330], got {}", dist);
    // Bearing is south-southeast, roughly 135 to 145 degrees
    assert!(brg > 130.0 && brg < 150.0, "Expected bearing in [130, 150], got {}", brg);
}

#[test]
fn test_colregs_classify_builtins() {
    let code = r#"
    HAI 1.2
    I HAS A SITUATION_HEADON ITZ I IZ COLREGS_CLASSIFY YR 0.0 AN YR 180.0 AN YR 2.0 MKAY
    I HAS A SITUATION_CROSSING ITZ I IZ COLREGS_CLASSIFY YR 45.0 AN YR -90.0 AN YR 2.0 MKAY
    I HAS A SITUATION_STANDON ITZ I IZ COLREGS_CLASSIFY YR -45.0 AN YR 90.0 AN YR 2.0 MKAY
    VISIBLE SITUATION_HEADON
    VISIBLE SITUATION_CROSSING
    VISIBLE SITUATION_STANDON
    KTHXBYE
    "#;
    let (_, vm) = run_source(code).expect("Execution failed");
    assert_eq!(vm.output_buffer, vec!["14", "15", "16"]);
}
