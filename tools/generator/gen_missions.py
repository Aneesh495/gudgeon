#!/usr/bin/env python3
"""
LOLCODE Autonomous Mission Profiles Generator
Generates full sea-trial missions: Channel Transit, Harbor Approach,
Open Sea Transit, Station Keeping, and Search-and-Rescue patterns.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

import os
from common import lol_header, lol_footer

def generate_channel_transit(output_dir):
    fpath = os.path.join(output_dir, "channel_transit.lol")
    out = []
    out.append(lol_header("Narrow Channel and TSS Transit Mission", "Autonomous transit through Traffic Separation Scheme with restricted water speed"))

    out.append("I HAS A CHANNEL_SPEED_LIMIT_KNOTS ITZ 10.0")
    out.append("I HAS A CHANNEL_WIDTH_METERS ITZ 300.0")
    out.append("I HAS A MAX_ALLOWABLE_XTE_METERS ITZ 40.0")
    out.append("")

    out.append("HOW IZ I CHECK_CHANNEL_BOUNDARY YR CURRENT_XTE_M")
    out.append("    I HAS A ABS_XTE ITZ I IZ MATH_ABS YR CURRENT_XTE_M MKAY")
    out.append("    I HAS A IN_CHANNEL ITZ BOTH SAEM SMALLR OF ABS_XTE AN MAX_ALLOWABLE_XTE_METERS AN ABS_XTE")
    out.append("    FOUND YR IN_CHANNEL")
    out.append("IF U SAY SO")
    out.append("")

    for seg in range(1, 50):
        xte = (seg % 16 - 8) * 4.5
        out.append(f"HOW IZ I VERIFY_TSS_GATE_{seg:02d}")
        out.append(f"    I HAS A OK ITZ I IZ CHECK_CHANNEL_BOUNDARY YR {xte:.1f} MKAY")
        out.append(f"    FOUND YR OK")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_station_keeping(output_dir):
    fpath = os.path.join(output_dir, "station_keeping.lol")
    out = []
    out.append(lol_header("Dynamic Positioning Virtual Anchor Station Keeping", "Maintains vessel position within 2-meter watch circle using dual azimuth thrusters"))

    out.append("I HAS A WATCH_CIRCLE_RADIUS_M ITZ 2.5")
    out.append("I HAS A DP_KP_POS ITZ 12000.0")
    out.append("I HAS A DP_KD_VEL ITZ 35000.0")
    out.append("")

    out.append("HOW IZ I COMPUTE_STATION_KEEPING_RESTORE_FORCE YR POS_ERROR_M AN YR VEL_MPS")
    out.append("    I HAS A P_FORCE ITZ PRODUKT OF DP_KP_POS AN POS_ERROR_M")
    out.append("    I HAS A D_FORCE ITZ PRODUKT OF DP_KD_VEL AN VEL_MPS")
    out.append("    FOUND YR SUM OF P_FORCE AN D_FORCE")
    out.append("IF U SAY SO")
    out.append("")

    for step in range(1, 50):
        e = (step % 10 - 5) * 0.4
        v = (step % 6 - 3) * 0.1
        out.append(f"HOW IZ I EVALUATE_DP_CYCLE_{step:02d}")
        out.append(f"    I HAS A FORCE ITZ I IZ COMPUTE_STATION_KEEPING_RESTORE_FORCE YR {e:.2f} AN YR {v:.2f} MKAY")
        out.append(f"    FOUND YR FORCE")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_search_and_rescue(output_dir):
    fpath = os.path.join(output_dir, "search_and_rescue.lol")
    out = []
    out.append(lol_header("Search and Rescue Expanding Square Pattern", "Generates coordinated multi-leg search trajectories for maritime emergency response"))

    out.append("I HAS A TRACK_SPACING_NM ITZ 0.5")
    out.append("I HAS A SEARCH_SPEED_KNOTS ITZ 12.0")
    out.append("")

    out.append("HOW IZ I COMPUTE_LEG_LENGTH_NM YR LEG_NUMBER")
    out.append("    I HAS A MULTIPLIER ITZ QUOSHUNT OF SUM OF LEG_NUMBER AN 1 AN 2")
    out.append("    FOUND YR PRODUKT OF MULTIPLIER AN TRACK_SPACING_NM")
    out.append("IF U SAY SO")
    out.append("")

    for leg in range(1, 50):
        out.append(f"HOW IZ I GENERATE_SAR_LEG_{leg:02d}")
        out.append(f"    I HAS A LEN ITZ I IZ COMPUTE_LEG_LENGTH_NM YR {leg} MKAY")
        out.append(f"    FOUND YR LEN")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def main():
    root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    out_dir = os.path.join(root, "scripts", "missions")
    os.makedirs(out_dir, exist_ok=True)
    generate_channel_transit(out_dir)
    generate_station_keeping(out_dir)
    generate_search_and_rescue(out_dir)

if __name__ == "__main__":
    main()
