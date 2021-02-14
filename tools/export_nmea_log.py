#!/usr/bin/env python3
"""
NMEA 0183 and AIS Telemetry Exporter
Generates standard .nmea simulation log files for playback in maritime ECDIS
and chart plotters such as OpenCPN.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

import math
import sys

def compute_checksum(payload):
    c = 0
    for b in payload.encode("ascii"):
        c ^= b
    return f"{c:02X}"

def format_sentence(sentence_type, fields):
    payload = f"{sentence_type},{','.join(fields)}"
    chk = compute_checksum(payload)
    return f"${payload}*{chk}"

def generate_sample_log(filename="gudgeon_sea_trial.nmea"):
    base_lat = 37.7749
    base_lon = -122.4194
    speed_knots = 12.5
    heading = 085.0

    lines = []
    for t in range(60):
        hh = 12
        mm = t // 60
        ss = t % 60
        time_str = f"{hh:02d}{mm:02d}{ss:02d}"

        cur_lat = base_lat + (t * 0.0001)
        cur_lon = base_lon + (t * 0.00015)

        lat_deg = int(abs(cur_lat))
        lat_min = (abs(cur_lat) - lat_deg) * 60.0
        lat_hemi = "N" if cur_lat >= 0 else "S"

        lon_deg = int(abs(cur_lon))
        lon_min = (abs(cur_lon) - lon_deg) * 60.0
        lon_hemi = "E" if cur_lon >= 0 else "W"

        # RMC
        rmc_fields = [
            time_str, "A",
            f"{lat_deg:02d}{lat_min:07.4f}", lat_hemi,
            f"{lon_deg:03d}{lon_min:07.4f}", lon_hemi,
            f"{speed_knots:.1f}", f"{heading:.1f}", "241126", "013.0", "E"
        ]
        lines.append(format_sentence("GPRMC", rmc_fields))

        # HDT
        lines.append(format_sentence("GPHDT", [f"{heading:.1f}", "T"]))

        # ROT
        lines.append(format_sentence("GPROT", ["0.0", "A"]))

    content = "\n".join(lines) + "\n"
    print(f"Generated {len(lines)} NMEA sentences.")
    return content

if __name__ == "__main__":
    generate_sample_log()
