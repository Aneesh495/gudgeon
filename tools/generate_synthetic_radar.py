#!/usr/bin/env python3
"""
Synthetic Radar Contact Track Generator
Produces multi-target kinematic profiles for multi-ship encounter benchmarks.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

import math
import json

def generate_multi_target_encounter_scenario():
    targets = [
        {"id": 101, "name": "CONTAINER_VESSEL_HEADON", "lat": 37.85, "lon": -122.4194, "sog_kts": 16.0, "cog_deg": 180.0},
        {"id": 102, "name": "BULK_CARRIER_CROSSING",    "lat": 37.80, "lon": -122.3500, "sog_kts": 14.0, "cog_deg": 270.0},
        {"id": 103, "name": "TUG_STANDON",             "lat": 37.80, "lon": -122.4800, "sog_kts": 9.0,  "cog_deg": 090.0},
        {"id": 104, "name": "FAST_FERRY_OVERTAKING",   "lat": 37.72, "lon": -122.4194, "sog_kts": 24.0, "cog_deg": 000.0},
    ]
    return targets

def main():
    scenario = generate_multi_target_encounter_scenario()
    print("================================================================================")
    print("  SYNTHETIC RADAR TARGET GENERATOR")
    print("================================================================================")
    for t in scenario:
        print(f"  Target {t['id']}: {t['name']:<25} | SOG: {t['sog_kts']:4.1f} kts | COG: {t['cog_deg']:05.1f} deg")
    print("--------------------------------------------------------------------------------")
    print(f"  Total Contacts Generated : {len(scenario)}")
    print("================================================================================")

if __name__ == "__main__":
    main()
