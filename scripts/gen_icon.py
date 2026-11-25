#!/usr/bin/env python3
"""Deterministically generate assets/icon.png (no binary assets in git).

Rationale: shipping the icon as a hand-pasted base64 blob proved fragile
(one mangled upload crashed CI at "Restore bundle icon"). This script
rebuilds the icon from ~90 lines of code on every machine, using only
the Python standard library (zlib + struct).

Design: dark navy vertical gradient (the app's dark theme) with a gold
four-point star — the classic Dragon Quest "sparkle" — plus a thin gold
ring for depth.
"""
import math
import os
import struct
import zlib

SIZE = 512
CX = CY = SIZE / 2.0

NAVY_TOP = (24, 33, 58)
NAVY_BOT = (9, 12, 24)
GOLD = (240, 196, 92)
GOLD_DARK = (186, 141, 52)


def lerp(a, b, t):
    return tuple(a[i] + (b[i] - a[i]) * t for i in range(3))


def star_radius(angle_from_axis, R=208.0, p=0.55):
    """Radius of a four-point star boundary at a given angle.

    angle_from_axis in [0, pi/2] measured from the horizontal axis.
    |cos(2a)| is 1 on both axes (long points up/down/left/right) and
    0 at 45 degrees (deep concave pinch); the exponent p < 1 keeps
    the arms chunky, p -> 0 gives a plain diamond.
    """
    return R * (abs(math.cos(2.0 * angle_from_axis)) ** p)


def coverage(x, y, r_bound, soft=2.0):
    d = math.hypot(x - CX, y - CY)
    if d <= r_bound - soft:
        return 1.0
    if d >= r_bound + soft:
        return 0.0
    return (r_bound + soft - d) / (2.0 * soft)


def build_rows():
    rows = []
    ring_r, ring_w = 236.0, 7.0  # decorative outer ring
    for y in range(SIZE):
        row = bytearray()
        base = lerp(NAVY_TOP, NAVY_BOT, y / (SIZE - 1))
        for x in range(SIZE):
            dx, dy = x - CX, y - CY
            d = math.hypot(dx, dy)
            ang = math.atan2(abs(dy), abs(dx))  # 0..pi/2 by symmetry
            px = base

            # faint radial glow behind the star
            glow_t = max(0.0, 1.0 - d / 260.0)
            px = lerp(px, (46, 58, 94), glow_t * 0.45)

            # gold star, shaded top-to-bottom for depth
            a = coverage(x, y, star_radius(ang))
            if a > 0.0:
                gc = lerp(GOLD, GOLD_DARK, min(1.0, max(0.0, y / (SIZE - 1))))
                px = lerp(px, gc, a)

            # thin ring, slightly darker gold
            if abs(d - ring_r) <= ring_w:
                ring_a = 1.0 - abs(d - ring_r) / ring_w
                if a < 0.35:
                    px = lerp(px, GOLD_DARK, ring_a * 0.9)

            row.extend(int(max(0, min(255, round(c)))) for c in px)
        rows.append(row)
    return rows


def write_png(path, size, rows):
    raw = b"".join(b"\x00" + bytes(r) for r in rows)

    def chunk(tag, data):
        return (
            struct.pack(">I", len(data))
            + tag
            + data
            + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)
        )

    png = b"\x89PNG\r\n\x1a\n"
    png += chunk(b"IHDR", struct.pack(">IIBBBBB", size, size, 8, 2, 0, 0, 0))
    png += chunk(b"IDAT", zlib.compress(raw, 9))
    png += chunk(b"IEND", b"")
    with open(path, "wb") as f:
        f.write(png)
    return len(png)


def main():
    out = os.path.join(
        os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
        "assets",
        "icon.png",
    )
    os.makedirs(os.path.dirname(out), exist_ok=True)
    n = write_png(out, SIZE, build_rows())
    print(f"wrote {out} ({n} bytes, {SIZE}x{SIZE} RGB)")


if __name__ == "__main__":
    main()
