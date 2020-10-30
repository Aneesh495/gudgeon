#!/usr/bin/env python3
"""
LOLCODE IMO COLREGS 1972 Collision Avoidance Module Generator
Generates rule decision trees, velocity obstacles, and CPA/TCPA matrices.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

import os
from common import lol_header, lol_footer

def generate_colregs_rules(output_dir):
    rules = [
        ("rule08_action.lol", "COLREGS Rule 8: Action to Avoid Collision", 8, "Evaluates early and substantial maneuvers with good seamanship"),
        ("rule13_overtaking.lol", "COLREGS Rule 13: Overtaking Situation", 13, "Classifies vessel coming up with another vessel from direction >22.5 deg abaft beam"),
        ("rule14_head_on.lol", "COLREGS Rule 14: Head-on Situation", 14, "Governs reciprocal or nearly reciprocal power-driven vessel encounters"),
        ("rule15_crossing.lol", "COLREGS Rule 15: Crossing Situation", 15, "Establishes give-way obligations for vessel having the other on starboard side"),
        ("rule16_give_way.lol", "COLREGS Rule 16: Action by Give-Way Vessel", 16, "Enforces substantial action taken in ample time to keep well clear"),
        ("rule17_stand_on.lol", "COLREGS Rule 17: Action by Stand-On Vessel", 17, "Governs maintaining course and speed, warning maneuvers, and in extremis actions"),
        ("rule18_responsibilities.lol", "COLREGS Rule 18: Responsibilities Between Vessels", 18, "Enforces vessel hierarchy: NUC, RAM, CBD, Fishing, Sailing, Power-driven"),
    ]

    for fname, title, rule_num, desc in rules:
        fpath = os.path.join(output_dir, fname)
        out = []
        out.append(lol_header(title, desc))
        out.append(f"I HAS A RULE_IDENTIFIER ITZ {rule_num}")
        out.append("I HAS A SAFE_PASSING_DISTANCE_NM ITZ 0.8")
        out.append("I HAS A LOOKAHEAD_TIME_SEC ITZ 900.0")
        out.append("")

        out.append(f"HOW IZ I EVALUATE_RULE_{rule_num} YR REL_BEARING_DEG AN YR ASPECT_DEG AN YR RANGE_NM")
        out.append("    I HAS A SITUATION ITZ I IZ COLREGS_CLASSIFY YR REL_BEARING_DEG AN YR ASPECT_DEG AN YR RANGE_NM MKAY")
        out.append(f"    I HAS A MATCHED ITZ BOTH SAEM SITUATION AN {rule_num}")
        out.append("    FOUND YR MATCHED")
        out.append("IF U SAY SO")
        out.append("")

        out.append(f"HOW IZ I RECOMMENDED_ACTION_{rule_num} YR CURRENT_HEADING_DEG AN YR CPA_NM")
        if rule_num in (8, 14, 15, 16):
            out.append("    BTW Substantial alteration of course to starboard")
            out.append("    I HAS A ALTERATION ITZ 35.0")
            out.append("    I HAS A NEW_HEADING ITZ SUM OF CURRENT_HEADING_DEG AN ALTERATION")
            out.append("    FOUND YR NEW_HEADING")
        elif rule_num == 17:
            out.append("    BTW Stand-on: Maintain course and speed")
            out.append("    FOUND YR CURRENT_HEADING_DEG")
        else:
            out.append("    FOUND YR CURRENT_HEADING_DEG")
        out.append("IF U SAY SO")
        out.append("")

        # Add encounter variation scenarios
        for scen in range(1, 45):
            brg = (scen * 7.5) % 360.0 - 180.0
            asp = (scen * 11.25) % 360.0 - 180.0
            rng = 1.0 + (scen % 5) * 0.8
            out.append(f"HOW IZ I TEST_ENCOUNTER_SCENARIO_{scen:02d}")
            out.append(f"    I HAS A RES ITZ I IZ EVALUATE_RULE_{rule_num} YR {brg:.1f} AN YR {asp:.1f} AN YR {rng:.1f} MKAY")
            out.append(f"    FOUND YR RES")
            out.append(f"IF U SAY SO")
            out.append("")

        out.append(lol_footer())
        with open(fpath, "w", encoding="utf-8") as f:
            f.write("\n".join(out))
        print(f"Generated {fpath} ({len(out)} lines)")

def generate_velocity_obstacle(output_dir):
    fpath = os.path.join(output_dir, "velocity_obstacle.lol")
    out = []
    out.append(lol_header("Velocity Obstacle Collision Cone Evaluator", "Geometric collision cone boundaries and evasion vector calculation"))

    out.append("I HAS A SECURITY_RADIUS_NM ITZ 0.8")
    out.append("I HAS A TIME_HORIZON_SEC ITZ 900.0")
    out.append("")

    out.append("HOW IZ I CHECK_VELOCITY_OBSTACLE YR DX_NM AN YR DY_NM AN YR DVX_KTS AN YR DVY_KTS")
    out.append("    I HAS A CPA ITZ I IZ CPA_COMPUTE YR DX_NM AN YR DY_NM AN YR DVX_KTS AN YR DVY_KTS MKAY")
    out.append("    I HAS A TCPA ITZ I IZ TCPA_COMPUTE YR DX_NM AN YR DY_NM AN YR DVX_KTS AN YR DVY_KTS MKAY")
    out.append("    I HAS A IN_CONE ITZ FAIL")
    out.append("    I HAS A CPA_VIOLATION ITZ BOTH SAEM SMALLR OF CPA AN SECURITY_RADIUS_NM AN CPA")
    out.append("    I HAS A TCPA_POS ITZ BOTH SAEM BIGGR OF TCPA AN 0.0 AN TCPA")
    out.append("    I HAS A TCPA_HORIZON ITZ BOTH SAEM SMALLR OF TCPA AN TIME_HORIZON_SEC AN TCPA")
    out.append("    IN_CONE R BOTH OF CPA_VIOLATION AN BOTH OF TCPA_POS AN TCPA_HORIZON")
    out.append("    FOUND YR IN_CONE")
    out.append("IF U SAY SO")
    out.append("")

    for vo in range(1, 50):
        dx = -2.0 + (vo * 0.1)
        dy = 1.5 - (vo * 0.08)
        dvx = 5.0 - (vo * 0.2)
        dvy = -4.0 + (vo * 0.15)
        out.append(f"HOW IZ I EVALUATE_VO_TARGET_{vo:02d}")
        out.append(f"    I HAS A CONFLICT ITZ I IZ CHECK_VELOCITY_OBSTACLE YR {dx:.2f} AN YR {dy:.2f} AN YR {dvx:.2f} AN YR {dvy:.2f} MKAY")
        out.append(f"    FOUND YR CONFLICT")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_cpa_tcpa_matrix(output_dir):
    fpath = os.path.join(output_dir, "cpa_tcpa_matrix.lol")
    out = []
    out.append(lol_header("Multi-Target CPA/TCPA Threat Matrix", "Computes closest approach metrics and ranks target collision hazards"))

    out.append("I HAS A MAX_TRACKED_TARGETS ITZ 16")
    out.append("I HAS A CRITICAL_CPA_THRESHOLD_NM ITZ 0.5")
    out.append("I HAS A CRITICAL_TCPA_THRESHOLD_SEC ITZ 300.0")
    out.append("")

    out.append("HOW IZ I ASSESS_TARGET_THREAT YR CPA_NM AN YR TCPA_SEC")
    out.append("    I HAS A THREAT_SCORE ITZ 0.0")
    out.append("    I HAS A CPA_RISK ITZ QUOSHUNT OF 1.0 AN SUM OF CPA_NM AN 0.1")
    out.append("    I HAS A TCPA_RISK ITZ QUOSHUNT OF 100.0 AN SUM OF TCPA_SEC AN 10.0")
    out.append("    THREAT_SCORE R PRODUKT OF CPA_RISK AN TCPA_RISK")
    out.append("    FOUND YR THREAT_SCORE")
    out.append("IF U SAY SO")
    out.append("")

    for tgt in range(1, 45):
        cpa = 0.1 + (tgt * 0.05)
        tcpa = 120.0 + (tgt * 20.0)
        out.append(f"HOW IZ I EVALUATE_THREAT_TARGET_{tgt:02d}")
        out.append(f"    I HAS A SCORE ITZ I IZ ASSESS_TARGET_THREAT YR {cpa:.2f} AN YR {tcpa:.1f} MKAY")
        out.append(f"    FOUND YR SCORE")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def main():
    root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    out_dir = os.path.join(root, "scripts", "colregs")
    os.makedirs(out_dir, exist_ok=True)
    generate_colregs_rules(out_dir)
    generate_velocity_obstacle(out_dir)
    generate_cpa_tcpa_matrix(out_dir)

if __name__ == "__main__":
    main()
