/**
 * Gudgeon: Autonomous Surface Vessel Navigation and Control Engine
 * C Foreign Function Interface (FFI) Header Specification
 *
 * Copyright (c) 2026 Aneesh Krishna Parthasarathy
 * Licensed under the ISC License.
 */

#ifndef GUDGEON_H
#define GUDGEON_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

#define GUDGEON_MAX_REGISTERS 256

/**
 * 3-DOF Vessel Kinematic and Hydrodynamic State
 */
typedef struct {
    double lat_deg;
    double lon_deg;
    double u_surge_mps;
    double v_sway_mps;
    double r_yaw_rate_radps;
    double heading_deg;
    double sog_knots;
    double cog_deg;
    double pod_left_thrust_n;
    double pod_left_angle_deg;
    double pod_right_thrust_n;
    double pod_right_angle_deg;
    double bow_thruster_thrust_n;
} gudgeon_vessel_state_t;

/**
 * COLREGS Encounter Classification Metrics
 */
typedef struct {
    uint32_t target_id;
    double range_nm;
    double bearing_deg;
    double cpa_nm;
    double tcpa_sec;
    int32_t situation_code; // 0=Clear, 13=Overtaking, 14=HeadOn, 15=GiveWay, 16=StandOn
} gudgeon_colregs_target_t;

/**
 * Initializes the Gudgeon navigation runtime.
 * Returns 0 on success, non-zero error code on failure.
 */
int32_t gudgeon_runtime_init(void);

/**
 * Executes a raw LOLCODE 1.2 source string.
 * Output is written into out_buf up to max_len bytes.
 */
int32_t gudgeon_eval_lolcode(const char* code_str, char* out_buf, size_t max_len);

/**
 * Executes a LOLCODE script file from disk.
 */
int32_t gudgeon_exec_lolcode_file(const char* filepath);

/**
 * Reads a 64-bit float from the telemetry register bank.
 */
double gudgeon_reg_read(uint32_t offset);

/**
 * Writes a 64-bit float into the telemetry register bank.
 */
void gudgeon_reg_write(uint32_t offset, double value);

/**
 * Steps the 3-DOF hydrodynamic vessel simulation by dt seconds.
 */
void gudgeon_sim_step(double fx_cmd_n, double fy_cmd_n, double mz_cmd_nm, double dt_sec);

/**
 * Retrieves the current vessel state into the caller-provided struct.
 */
void gudgeon_get_state(gudgeon_vessel_state_t* out_state);

/**
 * Shuts down the runtime and frees global state.
 */
void gudgeon_runtime_shutdown(void);

#ifdef __cplusplus
}
#endif

#endif /* GUDGEON_H */
