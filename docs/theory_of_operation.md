# Theory of Operation and COLREGS Compliance

Gudgeon governs autonomous surface vessel navigation by enforcing the International Regulations for Preventing Collisions at Sea (COLREGS 1972) through a deterministic decision-tree hierarchy.

```
+===================================================================================================+
|                                    COLREGS DECISION HIERARCHY                                     |
+===================================================================================================+
|                                                                                                   |
|                                       Radar / AIS Target Track                                    |
|                                                   |                                               |
|                                     Compute Range, CPA, and TCPA                                  |
|                                                   |                                               |
|                              Is Range < 6.0 NM and TCPA in Horizon?                               |
|                                                   |                                               |
|                         +-------------------------+-------------------------+                     |
|                         |                                                   |                     |
|                        YES                                                 NO                     |
|                         |                                                   |                     |
|             Is CPA < 0.25 NM & TCPA < 180s?                           Maintain Nominal            |
|                         |                                             Waypoint Track              |
|             +-----------+-----------+                                                             |
|             |                       |                                                             |
|            YES                      NO                                                            |
|             |                       |                                                             |
|      RULE 17(b) EMERGENCY   Evaluate Relative Bearing                                            |
|      IN EXTREMIS ACTION:    and Target Aspect Angle                                               |
|      Hard Starboard Turn            |                                                             |
|      (+60 deg alteration)           |                                                             |
|                                     v                                                             |
|          +--------------------------+--------------------------+                                  |
|          |                          |                          |                                  |
|   |Rel Brg| > 112.5 deg       |Rel Brg| <= 15 deg        5 deg < Rel Brg <= 112.5                 |
|          |                  & Course Diff >= 165 deg           |                                  |
|          v                          v                          v                                  |
|       RULE 13                    RULE 14                    RULE 15                               |
|     OVERTAKING                   HEAD-ON                    CROSSING                              |
|   (Stand-on vessel;         (Both alter course         (Give-way vessel;                          |
|   Maintain course           to starboard to pass       Substantial alteration                     |
|   and speed)                port-to-port)              to pass astern)                            |
|                                                                                                   |
+===================================================================================================+
```

## 1. Encounter Geometries and Rule Classifications

### 1.1 Rule 14: Head-On Situation
- **Trigger Condition**: Two power-driven vessels meeting on reciprocal or nearly reciprocal courses so as to involve risk of collision. Geometrically defined by a relative bearing within 15 degrees of the bow and a reciprocal heading difference between 165 and 195 degrees.
- **Autonomous Mandate**: Both vessels must alter course to starboard so that each shall pass on the port side of the other.
- **Gudgeon Action**: Commands a deliberate course change of at least +35 degrees to starboard, executed in ample time (TCPA > 300 seconds), ensuring the CPA expands beyond the 0.8 nautical mile safety radius.

### 1.2 Rule 15: Crossing Situation
- **Trigger Condition**: Two power-driven vessels crossing so as to involve risk of collision. The vessel which has the other on her own starboard side (relative bearing between 5 and 112.5 degrees) is the Give-Way vessel.
- **Autonomous Mandate**: The Give-Way vessel must keep out of the way and avoid crossing ahead of the other vessel.
- **Gudgeon Action**: Applies an immediate course alteration to starboard (+30 to +45 degrees) to pass cleanly astern of the stand-on vessel.

### 1.3 Rule 17: Action by Stand-On Vessel
- **Trigger Condition**: Vessel has the other on her own port side (relative bearing between -112.5 and -5 degrees).
- **Autonomous Mandate**: The stand-on vessel must maintain her course and speed.
- **Rule 17(a)(ii) Permitted Action**: If the give-way vessel does not appear to be taking appropriate action, the stand-on vessel may take action by her maneuver alone to avoid collision.
- **Rule 17(b) In Extremis Obligation**: When collision cannot be avoided by the give-way vessel alone, the stand-on vessel is legally obligated to take whatever action will best avert collision. Gudgeon executes an emergency hard starboard turn (+60 degrees) and sound signal trigger.

### 1.4 Rule 13: Overtaking
- **Trigger Condition**: A vessel coming up with another vessel from a direction more than 22.5 degrees abaft her beam (relative bearing greater than 112.5 degrees or less than -112.5 degrees).
- **Autonomous Mandate**: The overtaking vessel must keep out of the way of the vessel being overtaken.

---

## 2. Safety Corridors and Time Horizons

Gudgeon partitions the airspace and waterspace around the hull into concentric defensive zones:

| Defense Zone | Radius | Time Horizon | Action Level |
|---|---|---|---|
| **Monitoring Sector** | 6.0 NM | 900 seconds | Target tracking and early trajectory prediction |
| **Assessment Sector** | 3.0 NM | 600 seconds | COLREGS situation classification and planning |
| **Action Sector** | 1.5 NM | 400 seconds | Execution of substantial course and speed alterations |
| **In Extremis Boundary** | 0.25 NM | 180 seconds | Emergency avoidance maneuvers to avert immediate impact |
