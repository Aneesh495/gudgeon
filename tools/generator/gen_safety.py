#!/usr/bin/env python3
"""
LOLCODE Safety, Watchdog, and FDIR Module Generator
Generates fault detection, isolation, recovery, deadman monitors,
and loss-of-signal loiter sequences.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

import os
from common import lol_header, lol_footer

def generate_watchdog_fdir(output_dir):
    fpath = os.path.join(output_dir, "watchdog_fdir.lol")
    out = []
    out.append(lol_header("Safety Interlock and FDIR Executive", "Fault Detection, Isolation, and Recovery with 100 Hz deadman monitor"))

    out.append("I HAS A WATCHDOG_TIMEOUT_SEC ITZ 2.5")
    out.append("I HAS A LAST_HEARTBEAT_SEC ITZ 0.0")
    out.append("I HAS A FAULT_REGISTER ITZ 0")
    out.append("")

    out.append("HOW IZ I PET_WATCHDOG YR CURRENT_TIME_SEC")
    out.append("    LAST_HEARTBEAT_SEC R CURRENT_TIME_SEC")
    out.append("    FOUND YR WIN")
    out.append("IF U SAY SO")
    out.append("")

    out.append("HOW IZ I CHECK_WATCHDOG_EXPIRATION YR CURRENT_TIME_SEC")
    out.append("    I HAS A ELAPSED ITZ DIFF OF CURRENT_TIME_SEC AN LAST_HEARTBEAT_SEC")
    out.append("    I HAS A EXPIRED ITZ BOTH SAEM BIGGR OF ELAPSED AN WATCHDOG_TIMEOUT_SEC AN ELAPSED")
    out.append("    FOUND YR EXPIRED")
    out.append("IF U SAY SO")
    out.append("")

    for i in range(1, 50):
        t = i * 0.1
        out.append(f"HOW IZ I VERIFY_WATCHDOG_CYCLE_{i:02d}")
        out.append(f"    I HAS A EXP ITZ I IZ CHECK_WATCHDOG_EXPIRATION YR {t:.2f} MKAY")
        out.append(f"    FOUND YR EXP")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_loss_of_signal(output_dir):
    fpath = os.path.join(output_dir, "loss_of_signal.lol")
    out = []
    out.append(lol_header("Loss of Signal Fail-Safe Loiter Controller", "Transitions vessel to circular loiter pattern upon telemetry blackout"))

    out.append("I HAS A LOITER_RADIUS_M ITZ 200.0")
    out.append("I HAS A LOITER_SPEED_KTS ITZ 4.0")
    out.append("I HAS A FAIL_SAFE_ACTIVE ITZ FAIL")
    out.append("")

    out.append("HOW IZ I TRIGGER_FAIL_SAFE_LOITER")
    out.append("    FAIL_SAFE_ACTIVE R WIN")
    out.append("    FOUND YR LOITER_SPEED_KTS")
    out.append("IF U SAY SO")
    out.append("")

    for i in range(1, 45):
        out.append(f"HOW IZ I EVALUATE_LOITER_PATTERN_{i:02d} YR TIME_S")
        out.append(f"    I HAS A SPEED ITZ I IZ TRIGGER_FAIL_SAFE_LOITER MKAY")
        out.append(f"    FOUND YR SPEED")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_emergency_anchor(output_dir):
    fpath = os.path.join(output_dir, "emergency_anchor.lol")
    out = []
    out.append(lol_header("Emergency Anchoring and Speed Kill Procedure", "Executes emergency reverse thrust and virtual anchor drop sequence"))

    out.append("I HAS A REVERSE_THRUST_N ITZ 30000.0")
    out.append("I HAS A STOP_THRESHOLD_MPS ITZ 0.2")
    out.append("")

    out.append("HOW IZ I ASSESS_VESSEL_STOPPED YR SURGE_SPEED_MPS")
    out.append("    I HAS A STOPPED ITZ BOTH SAEM SMALLR OF SURGE_SPEED_MPS AN STOP_THRESHOLD_MPS AN SURGE_SPEED_MPS")
    out.append("    FOUND YR STOPPED")
    out.append("IF U SAY SO")
    out.append("")

    for i in range(1, 45):
        spd = 5.0 - (i * 0.12)
        out.append(f"HOW IZ I CHECK_EMERGENCY_STOP_STAGE_{i:02d}")
        out.append(f"    I HAS A STOPPED ITZ I IZ ASSESS_VESSEL_STOPPED YR {spd:.2f} MKAY")
        out.append(f"    FOUND YR STOPPED")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def main():
    root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    out_dir = os.path.join(root, "scripts", "safety")
    os.makedirs(out_dir, exist_ok=True)
    generate_watchdog_fdir(out_dir)
    generate_loss_of_signal(out_dir)
    generate_emergency_anchor(out_dir)

if __name__ == "__main__":
    main()
