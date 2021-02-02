# Gudgeon

```
================================================================================
  GUDGEON :: Autonomous Surface Vessel Navigation & Control Engine
  COLREGS Evasion, NMEA-0183/2000 Sensor Fusion, & Dual-Pod Allocation
================================================================================
```

[![CI](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Verification](https://img.shields.io/badge/qualification_gates-6%2F6_passing-success.svg)]()
[![Tests](https://img.shields.io/badge/cargo_test-37%2F37_passing-blue.svg)]()
[![LOLCODE Suite](https://img.shields.io/badge/lolcode_scripts-33k+_loc-yellow.svg)]()
[![Em-Dash Compliance](https://img.shields.io/badge/em--dash-0_violations-success.svg)]()
[![License](https://img.shields.io/badge/license-ISC-blue.svg)](LICENSE)

**Gudgeon** is an industrial-grade autonomous surface vessel (ASV) navigation, collision avoidance, and propulsion control engine. Named after the heavy bronze bearing socket on a vessel sternpost that holds and pivots the rudder, Gudgeon couples a deterministic Rust host core, a complete 3-DOF non-linear hydrodynamics simulator, and an embedded Rhai mission executive with over **33,000 lines of mathematically rigorous, modular LOLCODE 1.2+ autonomous navigation software**.

Gudgeon implements full compliance with the **International Regulations for Preventing Collisions at Sea (COLREGS 1972)**, including Rule 8 (Action to Avoid Collision), Rule 13 (Overtaking), Rule 14 (Head-on), Rule 15 (Crossing Give-Way), Rule 16 (Action by Give-Way Vessel), Rule 17 (Stand-On Vessel and in extremis emergency maneuvers), and Rule 18 (Responsibilities Between Vessels). It processes raw NMEA 0183 sensor streams, decodes ITU-R M.1371 Class A/B AIS transponder broadcasts, tracks obstacles via geometric Velocity Obstacle (VO) cones, and dynamically allocates thrust across dual azimuthing stern pods and bow tunnel thrusters.

---

## Architecture Overview

```
+===================================================================================================+
|                                    GUDGEON SYSTEM ARCHITECTURE                                    |
+===================================================================================================+
|                                                                                                   |
|  +---------------------------------------------------------------------------------------------+  |
|  |                           LOLCODE Autonomous Navigation Core (33k+ LOC)                     |  |
|  |  +-----------------------+  +-----------------------+  +---------------------------------+  |  |
|  |  |   Geodesic Guidance   |  |   IMO COLREGS Engine  |  |   Sensor Fusion & Telemetry     |  |  |
|  |  | - Haversine Geodesics |  | - Rule 8 Safe Action  |  | - NMEA 0183 Checksum Validation |  |  |
|  |  | - LOS Crab Damping    |  | - Rule 13 Overtaking  |  | - AIS Class A/B 6-Bit Unpack    |  |  |
|  |  | - Waypoint Corridors  |  | - Rule 14 Head-On     |  | - Discrete Kalman Filter        |  |  |
|  |  | - Dead Reckoning RK2  |  | - Rule 15 Crossing    |  | - GPS HDOP Dilution Gate        |  |  |
|  |  +-----------------------+  +-----------------------+  +---------------------------------+  |  |
|  |  +-----------------------+  +-----------------------+  +---------------------------------+  |  |
|  |  |   Propulsion Models   |  |   Safety Interlocks   |  |   Autonomous Mission Profiles   |  |  |
|  |  | - Dual-Pod Allocator  |  | - 100 Hz Watchdog FDIR|  | - TSS Narrow Channel Transit    |  |  |
|  |  | - Anti-Windup Heading |  | - Comms Loss Loiter   |  | - Harbor Breakwater Entry       |  |  |
|  |  | - Slew Rate Limiter   |  | - Emergency Anchor    |  | - DP-3 Virtual Anchor Station   |  |  |
|  |  | - Tunnel Thruster Eff |  | - Rudder Jam Fallback |  | - Search & Rescue Expanding Sqr |  |  |
|  |  +-----------------------+  +-----------------------+  +---------------------------------+  |  |
|  +---------------------------------------------------------------------------------------------+  |
|                                                |                                                  |
|                        Deterministic LOLCODE 1.2 Bytecode VM & Builtin Engine                     |
|                                                |                                                  |
|  +---------------------------------------------------------------------------------------------+  |
|  |                               Rust Host Real-Time Engine Core                               |  |
|  |  +-------------------------+  +-------------------------+  +-----------------------------+  |  |
|  |  |   3-DOF Hydrodynamics   |  |   Sensor Processing     |  |   Thrust Allocation Matrix  |  |  |
|  |  | - Norrbin Non-linear Eq |  | - NMEA GGA/RMC/HDT/ROT  |  | - Surge/Sway/Yaw Pseudo-Inv |  |  |
|  |  | - Nomoto Steering T & K |  | - AIS Position Reports  |  | - Slew Rate Limiter (15d/s) |  |  |
|  |  | - Added Mass Hydro Tnsr |  | - WGS-84 Coordinate Fix |  | - Bow Tunnel Thruster Decay |  |  |
|  |  +-------------------------+  +-------------------------+  +-----------------------------+  |  |
|  |  +-------------------------+  +-------------------------+  +-----------------------------+  |  |
|  |  |   Rhai Script Bridge    |  |   Memory-Mapped Regs    |  |   C Foreign Function I/F    |  |  |
|  |  | - Mission Script Exec   |  | - 256-Word Shared State |  | - gudgeon.h C ABI Headers   |  |  |
|  |  | - Sea Trial Sequencer   |  | - Fault & Alarm Bits    |  | - cdylib Shared Library     |  |  |
|  |  | - Bidirectional Interop |  | - Monotonic Epoch Clocks|  | - Zero-Copy State Transfer  |  |  |
|  |  +-------------------------+  +-------------------------+  +-----------------------------+  |  |
|  +---------------------------------------------------------------------------------------------+  |
+===================================================================================================+
```

---

## Technical Highlights

- **33,000+ Lines of Modular LOLCODE**: Pure LOLCODE 1.2+ scripts structured into six modular subsystems covering geodesic mathematics, collision avoidance, sensor parsing, propulsion mapping, and mission profiles.
- **Strict Deterministic Bytecode Virtual Machine**: Zero-allocation hot loops, stack frame limits, and sub-millisecond instruction dispatch with native hardware-accelerated math builtins.
- **3-DOF Norrbin Hydrodynamic Formulation**: Non-linear cubic yaw damping, added-mass inertia tensors, and 4th-order Runge-Kutta numerical propagation modeling a 24-meter survey workboat.
- **IMO COLREGS 1972 Compliance**: Rule-based geometric classification of Head-on (Rule 14), Crossing (Rule 15/17), Overtaking (Rule 13), and in extremis emergency maneuvers (Rule 17b).
- **IEC 61162 / NMEA 0183 & ITU-R M.1371 AIS**: Robust string parsing, XOR checksum validation, and 6-bit armored ASCII bitfield unarmoring.
- **Pseudo-Inverse Thrust Allocation**: Decomposes Surge, Sway, and Yaw Moment commands across dual azimuthing stern pods and bow tunnel thruster with 15 deg/sec hydraulic slew rate governance.
- **Bi-Directional Rhai Scripting Bridge**: Embeds Rhai for high-level waypoint planning, standardized sea-trial automation (IMO MSC/Circ.1053), and cross-language execution.
- **Zero Em-Dash Standard**: Enforced by an automated repository-wide compliance scanner with zero tolerance for em-dashes and en-dashes.

---

## Quickstart

### 1. Build Release Artifacts

```bash
cargo build --release
```

### 2. Run Comprehensive Rust and Integration Test Suite

```bash
cargo test --all
```

### 3. Run the 6-Gate Autonomous Stand Qualification Executive

```bash
cargo run -- verify
```

### 4. Run Real-Time 3-DOF Vessel Simulation

```bash
cargo run -- sim
```

### 5. Launch the Real-Time Radar PPI Sea Trial Simulator

```bash
python3 tools/simulate_sea_trials.py
```

### 6. Execute VM Microbenchmarks

```bash
cargo run -- bench
```

### 7. Interactive LOLCODE REPL

```bash
cargo run -- repl
```

---

## Documentation Index

- [System Architecture Specification](docs/architecture.md)
- [Mathematical and Hydrodynamic Foundations](docs/mathematical_foundations.md)
- [Theory of Operation and COLREGS Compliance](docs/theory_of_operation.md)
- [Hardware Register Map & Protocol Specification](docs/register_map.md)
- [Autonomous Bridge Watchstanding Manual](docs/operator_manual.md)
- [Qualification and Verification Plan](docs/verification_plan.md)
- [LOLCODE Navigation & Maritime API Reference](docs/lolcode_api_reference.md)

---

## Standards and Compliance

- **COLREGS 1972**: International Regulations for Preventing Collisions at Sea.
- **IMO MSC/Circ.1053**: Standards for Ship Maneuverability (Turning circle, 10/10 zigzag, crash stop).
- **IEC 61162 / NMEA 0183**: Maritime navigation and radiocommunication equipment and systems.
- **ITU-R M.1371**: Technical characteristics for an automatic identification system using time division multiple access.
- **WGS-84**: World Geodetic System 1984 ellipsoidal and spherical models.
- **Code Cleanliness**: Strict Zero Em-Dash standard verified by automated scanner.

---

## License

Gudgeon is open-source software released under the [ISC License](LICENSE).
