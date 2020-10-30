#!/usr/bin/env python3
"""
Common utilities and headers for LOLCODE suite generation.
Zero em-dash compliance guaranteed.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

def lol_header(title, description):
    lines = [
        "HAI 1.2",
        "CAN HAS STDIO?",
        f"BTW ============================================================================",
        f"BTW GUDGEON MARITIME AUTONOMY :: {title.upper()}",
        f"BTW {description}",
        f"BTW ============================================================================",
        "",
    ]
    return "\n".join(lines)

def lol_footer():
    return "\nKTHXBYE\n"
