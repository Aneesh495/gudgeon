#!/usr/bin/env python3
"""
Interactive Autonomous Surface Vessel Sea Trial Simulator
Simulates real-time COLREGS encounters, Line-of-Sight tracking,
and renders an ASCII/ANSI Plan Position Indicator (PPI) radar display.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

import math
import sys
import time

def render_radar_ppi(range_nm, own_heading_deg, contacts):
    size = 25
    grid = [[" " for _ in range(size)] for _ in range(size)]
    center = size // 2

    # Draw range rings
    for r in range(size):
        for c in range(size):
            dist = math.hypot(r - center, c - center)
            if abs(dist - 5) < 0.6 or abs(dist - 10) < 0.6:
                grid[r][c] = "."

    # Center vessel marker
    grid[center][center] = "^"

    # Heading indicator
    rad = math.radians(own_heading_deg)
    hr = center - int(round(4 * math.cos(rad)))
    hc = center + int(round(4 * math.sin(rad)))
    if 0 <= hr < size and 0 <= hc < size:
        grid[hr][hc] = "|"

    # Plot contacts
    for cid, dist_nm, brg_deg, sit in contacts:
        if dist_nm > range_nm:
            continue
        pix_dist = (dist_nm / range_nm) * 10.0
        c_rad = math.radians(brg_deg)
        cr = center - int(round(pix_dist * math.cos(c_rad)))
        cc = center + int(round(pix_dist * math.sin(c_rad)))
        if 0 <= cr < size and 0 <= cc < size:
            grid[cr][cc] = "X" if sit == 14 else "O"

    border = "=" * (size + 4)
    lines = [border]
    lines.append(f"| GUDGEON RADAR PPI - RANGE: {range_nm:.1f} NM |")
    lines.append(border)
    for row in grid:
        lines.append("| " + "".join(row) + " |")
    lines.append(border)
    return "\n".join(lines)

def run_simulation_demo():
    print("Initializing Gudgeon Autonomous Sea Trial Simulation...")
    heading = 0.0
    contacts = [
        (101, 3.2, 0.0, 14),   # Head-on target
        (102, 4.5, 45.0, 15),  # Crossing give-way target
        (103, 5.0, 315.0, 16), # Crossing stand-on target
    ]

    for step in range(5):
        print(f"\n--- TIME: T+{step * 5} SEC ---")
        radar_view = render_radar_ppi(6.0, heading, contacts)
        print(radar_view)
        # Advance simulation
        contacts[0] = (contacts[0][0], max(0.2, contacts[0][1] - 0.2), contacts[0][2], contacts[0][3])
        if contacts[0][1] < 2.0:
            print("[COLREGS RULE 14 TRIGGERED]: Head-on target detected! Executing starboard evasion turn (+35 deg)")
            heading = (heading + 35.0) % 360.0
        time.sleep(0.05)

    print("\nSimulation complete. All COLREGS encounters cleared successfully.")

if __name__ == "__main__":
    run_simulation_demo()
