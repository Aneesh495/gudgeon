#!/usr/bin/env python3
"""
LOLCODE Propulsion and Thrust Allocation Subsystem Generator
Generates dual-pod steering, tunnel thruster mapping, anti-windup PID,
and rudder rate limiting models.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

import os
from common import lol_header, lol_footer

def generate_pod_allocator(output_dir):
    fpath = os.path.join(output_dir, "pod_allocator.lol")
    out = []
    out.append(lol_header("Dual-Pod Stern Thrust Allocation Subsystem", "Decomposes surge, sway, and yaw commands into pod thrust and azimuth angles"))

    out.append("I HAS A MAX_POD_THRUST_N ITZ 45000.0")
    out.append("I HAS A MAX_POD_ANGLE_DEG ITZ 35.0")
    out.append("I HAS A POD_SEPARATION_M ITZ 3.2")
    out.append("")

    out.append("HOW IZ I COMPUTE_POD_SPLIT YR SURGE_N AN YR YAW_MOMENT_NM")
    out.append("    I HAS A BASE_THRUST ITZ QUOSHUNT OF SURGE_N AN 2.0")
    out.append("    I HAS A DIFF_THRUST ITZ QUOSHUNT OF YAW_MOMENT_NM AN POD_SEPARATION_M")
    out.append("    I HAS A LEFT_THRUST ITZ DIFF OF BASE_THRUST AN DIFF_THRUST")
    out.append("    I HAS A RIGHT_THRUST ITZ SUM OF BASE_THRUST AN DIFF_THRUST")
    out.append("    FOUND YR LEFT_THRUST")
    out.append("IF U SAY SO")
    out.append("")

    for i in range(1, 50):
        surge = 5000.0 + i * 800.0
        moment = (i % 10 - 5) * 1200.0
        out.append(f"HOW IZ I ALLOCATE_SETPOINT_{i:02d}")
        out.append(f"    I HAS A T_LEFT ITZ I IZ COMPUTE_POD_SPLIT YR {surge:.1f} AN YR {moment:.1f} MKAY")
        out.append(f"    FOUND YR T_LEFT")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_anti_windup_pid(output_dir):
    fpath = os.path.join(output_dir, "anti_windup_pid.lol")
    out = []
    out.append(lol_header("Heading Autopilot Anti-Windup PID Controller", "Closed-loop heading regulation with integrator anti-windup clamping"))

    out.append("I HAS A KP_HEADING ITZ 1.8")
    out.append("I HAS A KI_HEADING ITZ 0.05")
    out.append("I HAS A KD_HEADING ITZ 3.2")
    out.append("I HAS A INTEGRATOR_STATE ITZ 0.0")
    out.append("I HAS A INTEGRATOR_LIMIT ITZ 10.0")
    out.append("")

    out.append("HOW IZ I STEP_HEADING_PID YR HEADING_ERROR_DEG AN YR YAW_RATE_DEG_S AN YR DT_SEC")
    out.append("    I HAS A P_TERM ITZ PRODUKT OF KP_HEADING AN HEADING_ERROR_DEG")
    out.append("    I HAS A D_TERM ITZ PRODUKT OF KD_HEADING AN YAW_RATE_DEG_S")
    out.append("    I HAS A DELTA_INT ITZ PRODUKT OF HEADING_ERROR_DEG AN DT_SEC")
    out.append("    INTEGRATOR_STATE R SUM OF INTEGRATOR_STATE AN PRODUKT OF KI_HEADING AN DELTA_INT")
    out.append("    INTEGRATOR_STATE R I IZ MATH_CLAMP YR INTEGRATOR_STATE AN YR -10.0 AN YR 10.0 MKAY")
    out.append("    I HAS A RAW_RUDDER ITZ DIFF OF SUM OF P_TERM AN INTEGRATOR_STATE AN D_TERM")
    out.append("    I HAS A CLAMPED_RUDDER ITZ I IZ MATH_CLAMP YR RAW_RUDDER AN YR -35.0 AN YR 35.0 MKAY")
    out.append("    FOUND YR CLAMPED_RUDDER")
    out.append("IF U SAY SO")
    out.append("")

    for step in range(1, 50):
        err = (step % 20 - 10) * 1.5
        rate = (step % 10 - 5) * 0.2
        out.append(f"HOW IZ I TEST_PID_CYCLE_{step:02d}")
        out.append(f"    I HAS A CMD ITZ I IZ STEP_HEADING_PID YR {err:.2f} AN YR {rate:.2f} AN YR 0.1 MKAY")
        out.append(f"    FOUND YR CMD")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_rudder_rate_limiter(output_dir):
    fpath = os.path.join(output_dir, "rudder_rate_limiter.lol")
    out = []
    out.append(lol_header("Hydraulic Steering Rudder Slew Rate Governor", "Restricts angular velocity of steering gear to 15 degrees per second"))

    out.append("I HAS A MAX_SLEW_RATE_DEG_S ITZ 15.0")
    out.append("I HAS A CURRENT_ANGLE_DEG ITZ 0.0")
    out.append("")

    out.append("HOW IZ I GOVERN_RUDDER_SLEW YR TARGET_ANGLE_DEG AN YR DT_SEC")
    out.append("    I HAS A MAX_STEP ITZ PRODUKT OF MAX_SLEW_RATE_DEG_S AN DT_SEC")
    out.append("    I HAS A DELTA ITZ DIFF OF TARGET_ANGLE_DEG AN CURRENT_ANGLE_DEG")
    out.append("    I HAS A MINUS_STEP ITZ PRODUKT OF -1.0 AN MAX_STEP")
    out.append("    I HAS A CLAMPED_DELTA ITZ I IZ MATH_CLAMP YR DELTA AN YR MINUS_STEP AN YR MAX_STEP MKAY")
    out.append("    CURRENT_ANGLE_DEG R SUM OF CURRENT_ANGLE_DEG AN CLAMPED_DELTA")
    out.append("    FOUND YR CURRENT_ANGLE_DEG")
    out.append("IF U SAY SO")
    out.append("")

    for i in range(1, 45):
        target = (i % 14 - 7) * 4.0
        out.append(f"HOW IZ I TEST_SLEW_STEP_{i:02d}")
        out.append(f"    I HAS A ANGLE ITZ I IZ GOVERN_RUDDER_SLEW YR {target:.1f} AN YR 0.1 MKAY")
        out.append(f"    FOUND YR ANGLE")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def main():
    root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    out_dir = os.path.join(root, "scripts", "propulsion")
    os.makedirs(out_dir, exist_ok=True)
    generate_pod_allocator(out_dir)
    generate_anti_windup_pid(out_dir)
    generate_rudder_rate_limiter(out_dir)

if __name__ == "__main__":
    main()
