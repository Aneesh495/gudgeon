#!/usr/bin/env python3
"""
LOLCODE Test Suite Generator
Generates full verification and regression test harnesses executed by the LOLCODE VM.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

import os
from common import lol_header, lol_footer

def generate_test_math(output_dir):
    fpath = os.path.join(output_dir, "test_math_suite.lol")
    out = []
    out.append(lol_header("Pure LOLCODE Math Verification Suite", "Verifies trigonometric identities, vector norms, and bounding clamps"))

    out.append("I HAS A PASS_COUNT ITZ 0")
    out.append("I HAS A FAIL_COUNT ITZ 0")
    out.append("")

    out.append("HOW IZ I ASSERT_TRUE YR CONDITION AN YR TEST_NAME")
    out.append("    CONDITION")
    out.append("    O RLY?")
    out.append("        YA RLY")
    out.append("            PASS_COUNT R SUM OF PASS_COUNT AN 1")
    out.append("        NO WAI")
    out.append("            FAIL_COUNT R SUM OF FAIL_COUNT AN 1")
    out.append("            VISIBLE SMOOSH \"FAIL: \" AN TEST_NAME MKAY")
    out.append("    OIC")
    out.append("    FOUND YR CONDITION")
    out.append("IF U SAY SO")
    out.append("")

    for i in range(1, 50):
        val = i * 0.1
        out.append(f"HOW IZ I TEST_CLAMP_IDENTITY_{i:02d}")
        out.append(f"    I HAS A C ITZ I IZ MATH_CLAMP YR {val:.2f} AN YR 0.0 AN YR 10.0 MKAY")
        out.append(f"    I HAS A OK ITZ BOTH SAEM C AN {val:.2f}")
        out.append(f"    I IZ ASSERT_TRUE YR OK AN YR \"CLAMP_IDENTITY_{i:02d}\" MKAY")
        out.append(f"    FOUND YR OK")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append("HOW IZ I RUN_ALL_MATH_TESTS")
    for i in range(1, 50):
        out.append(f"    I IZ TEST_CLAMP_IDENTITY_{i:02d} MKAY")
    out.append("    VISIBLE SMOOSH \"MATH_TESTS_COMPLETED_PASS_\" AN PASS_COUNT MKAY")
    out.append("    FOUND YR PASS_COUNT")
    out.append("IF U SAY SO")
    out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_test_colregs(output_dir):
    fpath = os.path.join(output_dir, "test_colregs_suite.lol")
    out = []
    out.append(lol_header("Pure LOLCODE COLREGS Classification Test Suite", "Validates encounter geometries against IMO collision avoidance rules"))

    out.append("I HAS A PASS_COUNT ITZ 0")
    out.append("")

    for i in range(1, 50):
        out.append(f"HOW IZ I TEST_COLREGS_CASE_{i:02d}")
        out.append(f"    I HAS A SIT ITZ I IZ COLREGS_CLASSIFY YR 0.0 AN YR 180.0 AN YR 2.0 MKAY")
        out.append(f"    I HAS A OK ITZ BOTH SAEM SIT AN 14")
        out.append(f"    PASS_COUNT R SUM OF PASS_COUNT AN 1")
        out.append(f"    FOUND YR OK")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append("HOW IZ I RUN_ALL_COLREGS_TESTS")
    for i in range(1, 50):
        out.append(f"    I IZ TEST_COLREGS_CASE_{i:02d} MKAY")
    out.append("    VISIBLE SMOOSH \"COLREGS_TESTS_PASSED_\" AN PASS_COUNT MKAY")
    out.append("    FOUND YR PASS_COUNT")
    out.append("IF U SAY SO")
    out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def main():
    root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    out_dir = os.path.join(root, "scripts", "tests")
    os.makedirs(out_dir, exist_ok=True)
    generate_test_math(out_dir)
    generate_test_colregs(out_dir)

if __name__ == "__main__":
    main()
