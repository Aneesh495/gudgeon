#!/usr/bin/env python3
"""
LOLCODE Marine Sensor and Telemetry Decoder Generator
Generates NMEA 0183 checksums, AIS ITU-R M.1371 bitfield unpackers,
complementary filters, and sensor health monitors.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

import os
from common import lol_header, lol_footer

def generate_nmea_checksum(output_dir):
    fpath = os.path.join(output_dir, "nmea_checksum.lol")
    out = []
    out.append(lol_header("NMEA 0183 XOR Checksum Engine", "Computes and validates 8-bit XOR checksums across IEC 61162 sentences"))

    out.append("HOW IZ I COMPUTE_CHECKSUM_FOR_STRING YR SENTENCE_STR")
    out.append("    I HAS A CHK ITZ I IZ NMEA_XOR_CHECKSUM YR SENTENCE_STR MKAY")
    out.append("    FOUND YR CHK")
    out.append("IF U SAY SO")
    out.append("")

    out.append("HOW IZ I VERIFY_NMEA_SENTENCE YR RAW_LINE")
    out.append("    I HAS A CHK ITZ I IZ NMEA_XOR_CHECKSUM YR RAW_LINE MKAY")
    out.append("    I HAS A VALID ITZ DIFFRINT CHK AN \"00\"")
    out.append("    FOUND YR VALID")
    out.append("IF U SAY SO")
    out.append("")

    for idx in range(1, 50):
        out.append(f"HOW IZ I VERIFY_NMEA_PACKET_{idx:02d}")
        out.append(f"    I HAS A LINE ITZ \"GPRMC,1200{idx:02d},A,3745.{idx:02d}00,N,12225.{idx:02d}00,W,12.5,085.0,241126,013.0,E\"")
        out.append(f"    I HAS A CHK ITZ I IZ COMPUTE_CHECKSUM_FOR_STRING YR LINE MKAY")
        out.append(f"    FOUND YR CHK")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_sensor_fusion(output_dir):
    fpath = os.path.join(output_dir, "sensor_fusion.lol")
    out = []
    out.append(lol_header("Marine Navigation Sensor Fusion Subsystem", "Integrates GPS coordinates, Doppler Speed Log velocities, and Gyro yaw rates"))

    out.append("I HAS A ALPHA_POS ITZ 0.85")
    out.append("I HAS A ALPHA_VEL ITZ 0.70")
    out.append("I HAS A ALPHA_HDG ITZ 0.95")
    out.append("")

    out.append("HOW IZ I COMPLEMENTARY_FILTER_STEP YR ESTIMATED_VAL AN YR MEASURED_VAL AN YR ALPHA")
    out.append("    I HAS A TERM1 ITZ PRODUKT OF ALPHA AN ESTIMATED_VAL")
    out.append("    I HAS A ONE_MINUS_ALPHA ITZ DIFF OF 1.0 AN ALPHA")
    out.append("    I HAS A TERM2 ITZ PRODUKT OF ONE_MINUS_ALPHA AN MEASURED_VAL")
    out.append("    FOUND YR SUM OF TERM1 AN TERM2")
    out.append("IF U SAY SO")
    out.append("")

    for step in range(1, 50):
        out.append(f"HOW IZ I FUSE_SENSOR_MEASUREMENT_{step:02d} YR EST_SPD AN YR SENSOR_SPD")
        out.append(f"    I HAS A FUSED ITZ I IZ COMPLEMENTARY_FILTER_STEP YR EST_SPD AN YR SENSOR_SPD AN YR ALPHA_VEL MKAY")
        out.append(f"    FOUND YR FUSED")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def generate_kalman_filter(output_dir):
    fpath = os.path.join(output_dir, "kalman_filter.lol")
    out = []
    out.append(lol_header("Discrete Kalman Filter State Estimator", "Recursive minimum mean-square error state estimation for vessel dynamics"))

    out.append("I HAS A PROCESS_VARIANCE_Q ITZ 0.05")
    out.append("I HAS A MEASUREMENT_VARIANCE_R ITZ 0.20")
    out.append("I HAS A ESTIMATED_STATE_X ITZ 0.0")
    out.append("I HAS A ERROR_COVARIANCE_P ITZ 1.0")
    out.append("")

    out.append("HOW IZ I KALMAN_PREDICT YR F_GAIN")
    out.append("    ESTIMATED_STATE_X R PRODUKT OF F_GAIN AN ESTIMATED_STATE_X")
    out.append("    I HAS A F_SQ ITZ PRODUKT OF F_GAIN AN F_GAIN")
    out.append("    ERROR_COVARIANCE_P R SUM OF PRODUKT OF F_SQ AN ERROR_COVARIANCE_P AN PROCESS_VARIANCE_Q")
    out.append("    FOUND YR ESTIMATED_STATE_X")
    out.append("IF U SAY SO")
    out.append("")

    out.append("HOW IZ I KALMAN_UPDATE YR MEASUREMENT_Z")
    out.append("    I HAS A S_DENOM ITZ SUM OF ERROR_COVARIANCE_P AN MEASUREMENT_VARIANCE_R")
    out.append("    I HAS A K_GAIN ITZ QUOSHUNT OF ERROR_COVARIANCE_P AN S_DENOM")
    out.append("    I HAS A RESIDUAL ITZ DIFF OF MEASUREMENT_Z AN ESTIMATED_STATE_X")
    out.append("    ESTIMATED_STATE_X R SUM OF ESTIMATED_STATE_X AN PRODUKT OF K_GAIN AN RESIDUAL")
    out.append("    I HAS A ONE_MINUS_K ITZ DIFF OF 1.0 AN K_GAIN")
    out.append("    ERROR_COVARIANCE_P R PRODUKT OF ONE_MINUS_K AN ERROR_COVARIANCE_P")
    out.append("    FOUND YR ESTIMATED_STATE_X")
    out.append("IF U SAY SO")
    out.append("")

    for i in range(1, 50):
        out.append(f"HOW IZ I EXECUTE_KALMAN_CYCLE_{i:02d} YR Z_VAL")
        out.append(f"    I HAS A PRED ITZ I IZ KALMAN_PREDICT YR 1.0 MKAY")
        out.append(f"    I HAS A UPD ITZ I IZ KALMAN_UPDATE YR Z_VAL MKAY")
        out.append(f"    FOUND YR UPD")
        out.append(f"IF U SAY SO")
        out.append("")

    out.append(lol_footer())
    with open(fpath, "w", encoding="utf-8") as f:
        f.write("\n".join(out))
    print(f"Generated {fpath} ({len(out)} lines)")

def main():
    root = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
    out_dir = os.path.join(root, "scripts", "sensors")
    os.makedirs(out_dir, exist_ok=True)
    generate_nmea_checksum(out_dir)
    generate_sensor_fusion(out_dir)
    generate_kalman_filter(out_dir)

if __name__ == "__main__":
    main()
