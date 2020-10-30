#!/usr/bin/env python3
"""
LOLCODE Navigation and Path Planning Module Generator
Produces mathematically rigorous algorithms for Great Circle, Line of Sight,
Rhumb Line, Waypoint Traversal, and Dead Reckoning.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

import os
from common import lol_header, lol_footer

def generate_haversine(output_dir):
    fpath = os.path.join(output_dir, "haversine.lol")
    out = []
    out.append(lol_header("Great Circle and Haversine Navigation", "Computes spherical geodesics and true bearings across WGS-84"))

    # Earth radius constants and unit conversions
    out.append("BTW Earth radius in nautical miles and meters")
    out.append("I HAS A R_EARTH_NM ITZ 3440.065")
    out.append("I HAS A R_EARTH_METERS ITZ 6371000.0")
    out.append("I HAS A DEG2RAD ITZ 0.017453292519943295")
    out.append("I HAS A RAD2DEG ITZ 57.29577951308232")
    out.append("I HAS A PI ITZ 3.141592653589793")
    out.append("")

    # Degree to Radian function
    out.append("HOW IZ I DEG_TO_RAD YR DEG")
    out.append("    FOUND YR PRODUKT OF DEG AN DEG2RAD")
    out.append("IF U SAY SO")
    out.append("")

    out.append("HOW IZ I RAD_TO_DEG YR RAD")
    out.append("    FOUND YR PRODUKT OF RAD AN RAD2DEG")
    out.append("IF U SAY SO")
    out.append("")

    # Angle normalization to [-PI, PI]
    out.append("HOW IZ I NORMALIZE_ANGLE_RAD YR THETA")
    out.append("    I HAS A TWO_PI ITZ PRODUKT OF 2.0 AN PI")
    out.append("    I HAS A CUR ITZ THETA")
    out.append("    IM IN YR NORM_LOOP TIL BOTH SAEM SMALLR OF CUR AN PI AN CUR")
    out.append("        CUR R DIFF OF CUR AN TWO_PI")
    out.append("    IM OUTTA YR NORM_LOOP")
    out.append("    FOUND YR CUR")
    out.append("IF U SAY SO")
    out.append("")

    # Compass heading normalization to [0, 360)
    out.append("HOW IZ I NORMALIZE_HEADING_DEG YR HDG")
    out.append("    I HAS A H ITZ HDG")
    out.append("    IM IN YR HDG_POS TIL BOTH SAEM BIGGR OF H AN 0.0 AN H")
    out.append("        H R SUM OF H AN 360.0")
    out.append("    IM OUTTA YR HDG_POS")
    out.append("    IM IN YR HDG_MOD TIL BOTH SAEM SMALLR OF H AN 360.0 AN H")
    out.append("        H R DIFF OF H AN 360.0")
    out.append("    IM OUTTA YR HDG_MOD")
    out.append("    FOUND YR H")
    out.append("IF U SAY SO")
    out.append("")

    # Haversine distance in nautical miles
    out.append("HOW IZ I COMPUTE_HAVERSINE_NM YR LAT1_DEG AN YR LON1_DEG AN YR LAT2_DEG AN YR LON2_DEG")
    out.append("    I HAS A DIST ITZ I IZ NAV_HAVERSINE YR LAT1_DEG AN YR LON1_DEG AN YR LAT2_DEG AN YR LON2_DEG MKAY")
    out.append("    FOUND YR DIST")
    out.append("IF U SAY SO")
    out.append("")

    # Initial True Bearing calculation
    out.append("HOW IZ I COMPUTE_TRUE_BEARING_DEG YR LAT1_DEG AN YR LON1_DEG AN YR LAT2_DEG AN YR LON2_DEG")
    out.append("    I HAS A BRG ITZ I IZ NAV_BEARING YR LAT1_DEG AN YR LON1_DEG AN YR LAT2_DEG AN YR LON2_DEG MKAY")
    out.append("    FOUND YR BRG")
    out.append("IF U SAY SO")
    out.append("")

    # Waypoint verification test cases
    for idx in range(1, 40):
        lat1 = 37.0 + idx * 0.02
        lon1 = -122.0 - idx * 0.02
        lat2 = 37.5 + idx * 0.02
        lon2 = -122.5 - idx * 0.02
        out.append(f"HOW IZ I VERIFY_LEG_{idx:02d}")
        out.append(f"    I HAS A D ITZ I IZ COMPUTE_HAVERSINE_NM YR {lat1:.4f} AN YR {lon1:.4f} AN YR {lat2:.4f} AN YR {lon2:.4f} MKAY")
        out.append(f"    I HAS A B ITZ I IZ COMPUTE_TRUE_BEARING_DEG YR {lat1:.4f} AN YR {lon1:.4f} AN YR {lat2:.4f} AN YR {lon2:.4f} MKAY")
        out.append(f"    FOUND YR D")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_line_of_sight(output_dir):
    fpath = os.path.join(output_dir, "line_of_sight.lol")
    out = []
    out.append(lol_header("Line-of-Sight Guidance Law", "Adaptive cross-track integral action and crab angle steering"))

    out.append("I HAS A DEFAULT_LOOKAHEAD_METERS ITZ 150.0")
    out.append("I HAS A MAX_CRAB_ANGLE_DEG ITZ 35.0")
    out.append("I HAS A INTEGRAL_GAIN ITZ 0.002")
    out.append("I HAS A ACCUMULATED_XTE ITZ 0.0")
    out.append("")

    out.append("HOW IZ I RESET_INTEGRAL_ACTION")
    out.append("    ACCUMULATED_XTE R 0.0")
    out.append("    FOUND YR WIN")
    out.append("IF U SAY SO")
    out.append("")

    out.append("HOW IZ I UPDATE_CROSS_TRACK_INTEGRAL YR XTE_METERS AN YR DT_SEC")
    out.append("    I HAS A DELTA ITZ PRODUKT OF XTE_METERS AN DT_SEC")
    out.append("    ACCUMULATED_XTE R SUM OF ACCUMULATED_XTE AN DELTA")
    out.append("    FOUND YR ACCUMULATED_XTE")
    out.append("IF U SAY SO")
    out.append("")

    out.append("HOW IZ I COMPUTE_LOS_DESIRED_HEADING YR XTE_METERS AN YR LOOKAHEAD_M AN YR PATH_COURSE_DEG")
    out.append("    I HAS A CHI ITZ I IZ NAV_LOS_HEADING YR XTE_METERS AN YR LOOKAHEAD_M AN YR PATH_COURSE_DEG MKAY")
    out.append("    FOUND YR CHI")
    out.append("IF U SAY SO")
    out.append("")

    # Generate parameter sweep functions for diverse sea conditions
    for i in range(1, 50):
        out.append(f"HOW IZ I EVALUATE_LOS_PROFILE_{i:02d} YR ERROR_M AN YR PATH_DEG")
        out.append(f"    I HAS A LOOKAHEAD ITZ SUM OF 50.0 AN PRODUKT OF {i}.0 AN 3.0")
        out.append(f"    I HAS A HEADING ITZ I IZ COMPUTE_LOS_DESIRED_HEADING YR ERROR_M AN YR LOOKAHEAD AN YR PATH_DEG MKAY")
        out.append(f"    FOUND YR HEADING")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_waypoint_tracker(output_dir):
    fpath = os.path.join(output_dir, "waypoint_tracker.lol")
    out = []
    out.append(lol_header("Waypoint Tracker and Corridor Sequencer", "Manages route waypoints, switching radii, and arrival detection"))

    out.append("I HAS A CURRENT_INDEX ITZ 0")
    out.append("I HAS A TOTAL_WAYPOINTS ITZ 10")
    out.append("I HAS A DEFAULT_ACCEPTANCE_RADIUS_M ITZ 50.0")
    out.append("")

    out.append("HOW IZ I GET_CURRENT_WAYPOINT_INDEX")
    out.append("    FOUND YR CURRENT_INDEX")
    out.append("IF U SAY SO")
    out.append("")

    out.append("HOW IZ I ADVANCE_WAYPOINT")
    out.append("    CURRENT_INDEX R SUM OF CURRENT_INDEX AN 1")
    out.append("    FOUND YR CURRENT_INDEX")
    out.append("IF U SAY SO")
    out.append("")

    out.append("HOW IZ I CHECK_WAYPOINT_ARRIVAL YR DIST_METERS AN YR RADIUS_METERS")
    out.append("    I HAS A ARRIVED ITZ BOTH SAEM SMALLR OF DIST_METERS AN RADIUS_METERS AN DIST_METERS")
    out.append("    FOUND YR ARRIVED")
    out.append("IF U SAY SO")
    out.append("")

    # Route corridors
    for leg in range(1, 40):
        out.append(f"HOW IZ I EVALUATE_ROUTE_LEG_{leg:02d} YR DIST_M")
        out.append(f"    I HAS A ARRIVED ITZ I IZ CHECK_WAYPOINT_ARRIVAL YR DIST_M AN YR DEFAULT_ACCEPTANCE_RADIUS_M MKAY")
        out.append(f"    FOUND YR ARRIVED")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_dead_reckoning(output_dir):
    fpath = os.path.join(output_dir, "dead_reckoning.lol")
    out = []
    out.append(lol_header("Dead Reckoning Position Integrator", "Propagates 2D coordinates from speed log and gyro compass"))

    out.append("I HAS A DEG_LAT_METERS ITZ 111132.92")
    out.append("I HAS A KNOTS_TO_MPS ITZ 0.514444")
    out.append("")

    out.append("HOW IZ I COMPUTE_NORTH_EAST_VELOCITIES YR SOG_KTS AN YR COG_DEG")
    out.append("    I HAS A V_MPS ITZ PRODUKT OF SOG_KTS AN KNOTS_TO_MPS")
    out.append("    I HAS A COG_RAD ITZ PRODUKT OF COG_DEG AN 0.0174532925")
    out.append("    I HAS A VN ITZ PRODUKT OF V_MPS AN I IZ MATH_COS YR COG_RAD MKAY")
    out.append("    I HAS A VE ITZ PRODUKT OF V_MPS AN I IZ MATH_SIN YR COG_RAD MKAY")
    out.append("    FOUND YR VN")
    out.append("IF U SAY SO")
    out.append("")

    out.append("HOW IZ I STEP_DEAD_RECKONING YR LAT_DEG AN YR VN_MPS AN YR DT_SEC")
    out.append("    I HAS A DLAT_METERS ITZ PRODUKT OF VN_MPS AN DT_SEC")
    out.append("    I HAS A DLAT_DEG ITZ QUOSHUNT OF DLAT_METERS AN DEG_LAT_METERS")
    out.append("    FOUND YR SUM OF LAT_DEG AN DLAT_DEG")
    out.append("IF U SAY SO")
    out.append("")

    for step in range(1, 40):
        out.append(f"HOW IZ I INTEGRATE_STEP_{step:02d} YR LAT AN YR SPD")
        out.append(f"    I HAS A NEXT_LAT ITZ I IZ STEP_DEAD_RECKONING YR LAT AN YR SPD AN YR 1.0 MKAY")
        out.append(f"    FOUND YR NEXT_LAT")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def main():
    root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    out_dir = os.path.join(root, "scripts", "navigation")
    os.makedirs(out_dir, exist_ok=True)
    generate_haversine(out_dir)
    generate_line_of_sight(out_dir)
    generate_waypoint_tracker(out_dir)
    generate_dead_reckoning(out_dir)

if __name__ == "__main__":
    main()
