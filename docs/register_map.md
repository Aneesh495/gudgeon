# Hardware Register Map and Protocol Specification

Gudgeon manages an addressable 256-word double-precision (64-bit IEEE 754) memory-mapped telemetry register bank. Registers are organized into functional sectors.

## 1. Register Address Map

| Offset Range | Subsystem Category | Access | Description |
|---|---|---|---|
| **0x00 - 0x0F** | System Status & Watchdogs | Read/Write | System mode, heartbeat, fault flags, epoch timers |
| **0x10 - 0x1F** | Kinematics & Hydrodynamics | Read-Only | WGS-84 coordinates, speed, heading, velocities |
| **0x20 - 0x2F** | Environment & Wind/Drift | Read-Only | Apparent wind, true wind, sea state, current velocity |
| **0x30 - 0x3F** | Waypoint & LOS Guidance | Read/Write | Target waypoint, cross-track error, course command |
| **0x40 - 0x4F** | COLREGS & Target Matrix | Read-Only | Encounter status, min CPA, min TCPA, threat count |
| **0x50 - 0x5F** | Propulsion Actuators | Read/Write | Pod thrusts, azimuth angles, bow thruster force |
| **0x60 - 0x7F** | Sensor Health & Diagnostics | Read-Only | GPS HDOP, satellite count, IMU variance |
| **0x80 - 0xFF** | User & Mission Scratchpad | Read/Write | Custom mission variables and telemetry logging |

---

## 2. Specific Register Definitions

### Register 0x00: System Status (`REG_SYS_STATUS`)
- `0.0`: Uninitialized
- `1.0`: Standby / Moored
- `2.0`: Autonomous Navigation Active
- `3.0`: Evasive COLREGS Maneuver Active
- `4.0`: Emergency In Extremis Mode
- `5.0`: Fail-Safe Comms-Loss Loiter Mode

### Register 0x01: System Operating Mode (`REG_SYS_MODE`)
- `1.0`: Autonomous Line-of-Sight Mode
- `2.0`: Manual Helm Remote Override Mode
- `3.0`: Dynamic Positioning Virtual Anchor Mode

### Register 0x03: System Fault Register (`REG_FAULT_FLAGS`)
Bitfield packed as float:
- `Bit 0 (0x01)`: Watchdog heartbeat timeout
- `Bit 1 (0x02)`: GPS lock lost or HDOP > 2.5
- `Bit 2 (0x04)`: IMU gyro rate drift fault
- `Bit 3 (0x08)`: Left pod hydraulic pressure warning
- `Bit 4 (0x10)`: Right pod hydraulic pressure warning
- `Bit 5 (0x20)`: Bilge level alarm

### Kinematic Registers (0x10 to 0x18)
- `0x10 (16)`: Latitude (degrees, positive North, negative South)
- `0x11 (17)`: Longitude (degrees, positive East, negative West)
- `0x13 (19)`: Speed Over Ground (SOG) in knots
- `0x14 (20)`: Course Over Ground (COG) in degrees [0, 360)
- `0x15 (21)`: True Heading in degrees [0, 360)
- `0x16 (22)`: Rate of Turn in degrees per second
- `0x17 (23)`: Surge Velocity \(u\) in meters per second
- `0x18 (24)`: Sway Velocity \(v\) in meters per second

### Propulsion Command Registers (0x50 to 0x54)
- `0x50 (80)`: Left Stern Pod Thrust Command (Newtons)
- `0x51 (81)`: Left Stern Pod Azimuth Angle (degrees, [-35, +35])
- `0x52 (82)`: Right Stern Pod Thrust Command (Newtons)
- `0x53 (83)`: Right Stern Pod Azimuth Angle (degrees, [-35, +35])
- `0x54 (84)`: Bow Tunnel Thruster Command (Newtons, [-15000, +15000])
