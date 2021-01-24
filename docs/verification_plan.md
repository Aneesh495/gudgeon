# Qualification and Verification Plan

This document establishes the acceptance gates, testing criteria, and verification procedures for the Gudgeon autonomous navigation system.

## 1. 6-Gate Qualification Matrix

Qualification is verified on every build via `gudgeon verify` (and `make verify`), which executes six sequential automated gates:

```
+===================================================================================================+
|                                  QUALIFICATION GATE ARCHITECTURE                                  |
+===================================================================================================+
|                                                                                                   |
|  [GATE 1] LOLCODE Syntax & Virtual Machine Evaluation                                              |
|           - Lexical token stream verification                                                     |
|           - Recursive descent AST validation                                                      |
|           - Deterministic arithmetic and stack dispatch                                           |
|                                                                                                   |
|  [GATE 2] 3-DOF Hydrodynamics & Steering Dynamics                                                 |
|           - Norrbin non-linear steering rate response                                             |
|           - 4th-order Runge-Kutta numerical stability                                             |
|           - Straight-line surge acceleration and turning circle convergence                       |
|                                                                                                   |
|  [GATE 3] IMO COLREGS Rules 8/13/14/15/17 Classification                                          |
|           - Head-on reciprocal encounter classification                                           |
|           - Starboard crossing give-way classification                                            |
|           - Port crossing stand-on classification                                                 |
|           - In extremis emergency evasive recommendation                                          |
|                                                                                                   |
|  [GATE 4] NMEA 0183 & Sensor Protocol Integrity                                                   |
|           - GPRMC and GPGGA sentence decoding                                                     |
|           - XOR checksum verification and corrupted packet rejection                              |
|           - Coordinate hemisphere conversion precision                                            |
|                                                                                                   |
|  [GATE 5] Dual-Pod Thrust Allocation & Slew Governor                                              |
|           - Surge force split between stern pods                                                  |
|           - Yaw moment differential thrust allocation                                             |
|           - 15 degree/second hydraulic slew rate compliance                                       |
|                                                                                                   |
|  [GATE 6] Rhai High-Level Mission & Bridge Interop                                                |
|           - Shared register read/write synchronization                                            |
|           - Bidirectional LOLCODE evaluation from Rhai scripts                                    |
|           - Route planning and sea-trial maneuver executive passing                               |
|                                                                                                   |
+===================================================================================================+
```

## 2. Gate Verification Requirements

| Gate ID | Subsystem Tested | Acceptance Criterion | Tolerance |
|---|---|---|---|
| **GATE-01** | LOLCODE Engine | Arithmetic evaluation matches expected stack values | Exact match |
| **GATE-02** | 3-DOF Hydrodynamics | Speed > 0.5 m/s and non-zero yaw rate under rudder | > 0.5 m/s |
| **GATE-03** | COLREGS Engine | Head-on, crossing, and stand-on encounters correctly identified | Zero misclassifications |
| **GATE-04** | NMEA 0183 Parser | Valid packets accepted, corrupted checksums rejected | 100% rejection of corrupts |
| **GATE-05** | Thrust Allocator | Differential thrust correctly balances yaw demand | Pod right > Pod left |
| **GATE-06** | Rhai Interop | Bidirectional register synchronization succeeds | Float delta < 1e-4 |
