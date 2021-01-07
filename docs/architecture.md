# Gudgeon System Architecture Specification

Gudgeon is a high-performance, deterministic autonomous surface vessel navigation, COLREGS collision avoidance, and propulsion control executive. It couples a mission-critical 33,000+ line modular LOLCODE 1.2+ navigation codebase with a native Rust host execution engine, 3-DOF hydrodynamics simulator, and an embedded Rhai mission executive.

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

## Layered Architecture Decomposition

The system is organized into four primary architectural tiers:

### 1. LOLCODE Autonomous Navigation Core (`scripts/`)
The primary operational intelligence of the autonomous vessel is written in over 33,000 lines of modular LOLCODE 1.2+ scripts:
- **`scripts/navigation/`**: Spherical geodesic navigation, WGS-84 Great Circle computation, Line-of-Sight (LOS) path-following with integral crab-angle drift compensation, and dead reckoning position integration.
- **`scripts/colregs/`**: International Regulations for Preventing Collisions at Sea (COLREGS 1972) state machines for Rules 8, 13, 14, 15, 16, 17, and 18, along with Velocity Obstacle (VO) collision cones and multi-contact CPA/TCPA threat matrices.
- **`scripts/sensors/`**: NMEA 0183 XOR checksum accumulators, AIS 6-bit unarmoring filters, discrete Kalman filter state estimators, and GPS horizontal dilution of precision (HDOP) integrity gates.
- **`scripts/propulsion/`**: Dual-azimuth stern pod thrust allocators, heading autopilot anti-windup PID regulators, hydraulic steering rate limiters, and transverse bow tunnel thruster efficiency decay models.
- **`scripts/safety/`**: 100 Hz deadman watchdogs, communications loss fail-safe loiter patterns, and emergency reverse thrust anchoring procedures.
- **`scripts/missions/`**: Narrow channel transit, Traffic Separation Scheme (TSS) adherence, Dynamic Positioning (DP-3) virtual anchor station keeping, and search-and-rescue expanding square patterns.

### 2. Deterministic Virtual Machine (`src/lol/`)
A custom, high-throughput execution runtime that compiles and evaluates LOLCODE source programs with strict deterministic latency:
- **Lexical Analyzer (`lexer.rs`)**: Tokenizes LOLCODE source text with line and column span tracking.
- **Recursive Descent Parser (`parser.rs`)**: Builds strongly-typed Abstract Syntax Trees for Polish prefix expressions, loops, and conditional constructs.
- **Virtual Machine (`vm.rs`)**: Executes AST nodes and bytecode chunks with native hardware-accelerated bindings for double-precision trigonometry, Great Circle geodesics, and telemetry register access.

### 3. Rust Host Real-Time Engine Core (`src/vessel/`, `src/sensors/`, `src/colregs/`, `src/propulsion/`)
Provides hard physics integration and protocol handling:
- **3-DOF Hydrodynamics (`vessel/hydrodynamics.rs`)**: Integrates Norrbin non-linear steering equations and Nomoto added-mass models using 4th-order Runge-Kutta numerical integration.
- **Sensors (`sensors/nmea.rs`, `sensors/ais.rs`)**: Decodes and verifies NMEA 0183 (GGA, RMC, HDT, ROT, VTG) sentences and ITU-R M.1371 Class A/B AIS telemetry frames.
- **COLREGS Classifier (`colregs/engine.rs`)**: Evaluates geometric encounter aspects and calculates Closest Point of Approach (CPA) distance and Time to Closest Point of Approach (TCPA).
- **Thrust Allocator (`propulsion/allocation.rs`)**: Resolves generalized forces (Surge Fx, Sway Fy, Yaw Moment Mz) into dual azimuthing stern pod angles and bow thruster commands.

### 4. Interoperability & Scripting Interfaces (`src/rhai_bridge/`, `src/c_api.rs`, `c_api/include/`)
- **Rhai Bridge (`rhai_bridge/`)**: Embeds the Rhai scripting engine to allow dynamic high-level mission planning, standardized sea-trial automation (IMO MSC/Circ.1053), and bidirectional execution of LOLCODE routines.
- **C Foreign Function Interface (`c_api.rs`, `gudgeon.h`)**: Exposes C ABI symbols and shared libraries for integration with C, C++, and Python simulation environments.
