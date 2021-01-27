# LOLCODE Navigation and Maritime API Reference

This reference documents the native built-in functions provided by the Gudgeon execution environment to LOLCODE scripts.

## 1. Trigonometry and Mathematical Builtins

All angles passed to trigonometric functions are expressed in radians, with helper functions provided for degree conversion.

### `MATH_SIN(rad)`
Returns the sine of the angle in radians.

### `MATH_COS(rad)`
Returns the cosine of the angle in radians.

### `MATH_TAN(rad)`
Returns the tangent of the angle in radians.

### `MATH_ATAN2(y, x)`
Computes the four-quadrant arctangent of \(y / x\) in radians [-PI, PI].

### `MATH_HYPOT(x, y)`
Computes the Euclidean norm \(\sqrt{x^2 + y^2}\).

### `MATH_CLAMP(val, min, max)`
Restricts `val` to the closed interval `[min, max]`.

### `MATH_DEG2RAD(deg)`
Converts decimal degrees to radians.

### `MATH_RAD2DEG(rad)`
Converts radians to decimal degrees.

---

## 2. Geodesic and Navigation Builtins

### `NAV_HAVERSINE(lat1, lon1, lat2, lon2)`
Computes the Great Circle distance in nautical miles between two coordinates on the WGS-84 sphere.

### `NAV_BEARING(lat1, lon1, lat2, lon2)`
Computes the true initial bearing in decimal degrees [0, 360) from point 1 to point 2.

### `NAV_CROSS_TRACK_ERR(dist_to_cur, bearing_to_cur_deg, path_bearing_deg)`
Computes the perpendicular cross-track error in nautical miles relative to the intended path.

### `NAV_LOS_HEADING(cross_track_err_m, lookahead_dist_m, path_course_deg)`
Computes the desired Line-of-Sight steering course in degrees [0, 360) incorporating crab angle correction.

---

## 3. COLREGS and Target Collision Builtins

### `COLREGS_CLASSIFY(rel_bearing_deg, aspect_deg, range_nm)`
Classifies the encounter geometry according to IMO COLREGS 1972 rules. Returns:
- `0`: Clear / no immediate encounter
- `13`: Rule 13 (Overtaking)
- `14`: Rule 14 (Head-on)
- `15`: Rule 15 (Crossing Give-Way)
- `16`: Rule 17 (Crossing Stand-On)

### `CPA_COMPUTE(dx_nm, dy_nm, dvx_kts, dvy_kts)`
Calculates the Closest Point of Approach distance in nautical miles based on relative position and velocity vectors.

### `TCPA_COMPUTE(dx_nm, dy_nm, dvx_kts, dvy_kts)`
Calculates the Time to Closest Point of Approach in seconds.

---

## 4. Hardware Telemetry Register Builtins

### `READ_REG(reg_offset)`
Reads a 64-bit floating point value from the hardware register bank at index `reg_offset`.

### `WRITE_REG(reg_offset, value)`
Writes a 64-bit floating point value into the hardware register bank at index `reg_offset`.
