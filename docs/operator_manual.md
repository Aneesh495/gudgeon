# Autonomous Bridge Watchstanding and Operator Manual

This document provides operational procedures for shore-based operators and onboard watchstanders monitoring autonomous operations driven by Gudgeon.

## 1. Operating Modes and Helm Handover Procedures

Gudgeon supports three primary operating states: Autonomous Track, Manual Helm Remote Override, and Emergency Stop.

### 1.1 Engaging Autonomous Navigation
1. Verify that GPS HDOP is below 1.5 and satellite count is 8 or greater.
2. Confirm that dual stern pods and bow tunnel thruster have hydraulic pressure in green zone.
3. Upload and validate the waypoint route file using the mission executive.
4. Set Register 0x01 (`REG_SYS_MODE`) to `1.0` (Autonomous).
5. Verify on the bridge display that the vessel transitions smoothly from station keeping to the initial path intercept heading.

### 1.2 Taking Manual Remote Helm Override
Under IMO guidelines, an operator must be capable of immediately disengaging the autonomous controller:
1. Actuate the physical bridge helm or remote joystick.
2. The host hardware supervisor immediately detects manual control activity and asserts Register 0x01 to `2.0` (Manual Override).
3. Gudgeon suspends autonomous pod commands and transfers direct azimuth and throttle control to the manual operator within 20 milliseconds.
4. Audible tone confirms manual takeover on the console.

### 1.3 Emergency Stop Sequence
In the event of an unavoidable obstacle, navigation hazard, or catastrophic system fault:
1. Depress the emergency stop palm button on the console or send command `EMERGENCY_STOP` via telemetry.
2. Both stern pods slew to neutral (0 degrees).
3. Reverse thrust of 30,000 N per pod is commanded until surge speed drops below 0.25 m/s.
4. Pods shut down and virtual dynamic anchor mode is asserted.

---

## 2. Standard Sea Trial Qualification Maneuvers

Prior to uncrewed deployment, the vessel must execute standardized sea-trial maneuvers compliant with IMO MSC/Circ.1053:

### 2.1 Turning Circle Test
- Conducted at 14 knots with hard 35-degree starboard rudder.
- Advance must not exceed 4.5 vessel lengths (108 meters for a 24-meter hull).
- Tactical diameter must not exceed 5.0 vessel lengths (120 meters).

### 2.2 10/10 Zigzag Maneuver
- Vessel approaches at base speed (12 knots).
- Rudder is deflected 10 degrees to starboard until heading changes by 10 degrees.
- Rudder is reversed to 10 degrees port until heading changes by 10 degrees port.
- First overshoot angle must remain below 10 degrees.
