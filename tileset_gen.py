# Generate a 16x16 tileset (each tile 32x32 px) per the user's mapping.
# We'll produce two files:
# 1) tileset.png (clean, transparent background for unused tiles)
# 2) tileset_preview.png (with grid + tile indices overlaid to visually verify positions)
# And a CSV mapping file to document indices -> tile names/letters/properties.

from PIL import Image, ImageDraw, ImageFont
import csv
from math import sin, cos, pi

TILE_SIZE = 32
GRID_W = 16
GRID_H = 16
IMG_W = GRID_W * TILE_SIZE
IMG_H = GRID_H * TILE_SIZE

# Create base transparent image
tileset = Image.new("RGBA", (IMG_W, IMG_H), (0, 0, 0, 0))
draw = ImageDraw.Draw(tileset)

# Small helpers
def idx_to_xy(idx):
    r = idx // GRID_W
    c = idx % GRID_W
    return c * TILE_SIZE, r * TILE_SIZE

def rect(x, y, w=TILE_SIZE, h=TILE_SIZE):
    return (x, y, x + w - 1, y + h - 1)

def fill_tile(x, y, color):
    draw.rectangle(rect(x, y), fill=color)

def dither_tile(x, y, c1, c2, step=4):
    # Simple checker pattern for texture
    for yy in range(y, y + TILE_SIZE):
        for xx in range(x, x + TILE_SIZE):
            if ((xx + yy) % step) == 0:
                draw.point((xx, yy), fill=c1)
            elif ((xx + yy) % step) == 2:
                draw.point((xx, yy), fill=c2)

def draw_floor(idx):
    x,y = idx_to_xy(idx)
    fill_tile(x, y, (188, 188, 188, 255))
    dither_tile(x, y, (210,210,210,255), (160,160,160,255), step=4)

def draw_grass(idx):
    x,y = idx_to_xy(idx)
    fill_tile(x, y, (60, 160, 50, 255))
    dither_tile(x, y, (80, 190, 70, 255), (40, 130, 40, 255), step=4)

def draw_stone(idx):
    x,y = idx_to_xy(idx)
    fill_tile(x, y, (150, 150, 150, 255))
    # cobbles
    cobble = ImageDraw.Draw(tileset)
    for i in range(3):
        for j in range(2):
            rx = x + 4 + i*9 + (j%2)*3
            ry = y + 5 + j*14
            cobble.rounded_rectangle((rx, ry, rx+7, ry+10), radius=2, outline=(120,120,120,255), fill=(170,170,170,255))

def draw_wood(idx):
    x,y = idx_to_xy(idx)
    fill_tile(x, y, (146, 101, 60, 255))
    # planks
    for i in range(0, TILE_SIZE, 10):
        draw.rectangle((x+i, y, x+i+8, y+TILE_SIZE-1), outline=(100,70,40,255))
        # wood grain lines
        for gy in range(y+4, y+TILE_SIZE-2, 7):
            draw.line((x+i+2, gy, x+i+6, gy), fill=(120,85,50,255))

def draw_wall(idx):
    x,y = idx_to_xy(idx)
    fill_tile(x, y, (60, 60, 70, 255))
    # brick pattern
    bw, bh = 12, 6
    for row in range(0, TILE_SIZE, bh):
        offset = 0 if (row//bh)%2==0 else bw//2
        for col in range(-offset, TILE_SIZE, bw):
            rx1 = x + max(col, 0)
            ry1 = y + row
            rx2 = min(x + col + bw - 1, x + TILE_SIZE - 1)
            ry2 = y + row + bh - 1
            if rx1 <= rx2:
                draw.rectangle((rx1, ry1, rx2, ry2), outline=(40,40,45,255))

def draw_door(idx):
    x,y = idx_to_xy(idx)
    # frame
    draw.rectangle((x+2, y+2, x+TILE_SIZE-3, y+TILE_SIZE-3), fill=(125, 84, 45, 255), outline=(80,50,30,255))
    # planks
    for gy in range(y+6, y+TILE_SIZE-4, 6):
        draw.line((x+4, gy, x+TILE_SIZE-5, gy), fill=(90,60,35,255))
    # handle
    draw.ellipse((x+TILE_SIZE-10, y+TILE_SIZE//2-2, x+TILE_SIZE-7, y+TILE_SIZE//2+1), fill=(230, 210, 80, 255))

def draw_chest(idx):
    x,y = idx_to_xy(idx)
    # base
    draw.rectangle((x+3, y+8, x+TILE_SIZE-4, y+TILE_SIZE-4), fill=(140, 90, 40, 255), outline=(80,50,25,255))
    # lid
    draw.rectangle((x+3, y+3, x+TILE_SIZE-4, y+10), fill=(160, 105, 50, 255), outline=(90,60,30,255))
    # metal band
    draw.rectangle((x+TILE_SIZE//2-2, y+3, x+TILE_SIZE//2+2, y+TILE_SIZE-4), fill=(190, 190, 190, 255))
    # lock
    draw.rectangle((x+TILE_SIZE//2-2, y+12, x+TILE_SIZE//2+2, y+16), fill=(220, 210, 90, 255))

def draw_spike(idx):
    x,y = idx_to_xy(idx)
    # floor base
    fill_tile(x, y, (188,188,188,255))
    # spikes (triangles)
    spike_w = 8
    for i in range(0, TILE_SIZE, spike_w):
        draw.polygon([(x+i+4, y+24), (x+i, y+30), (x+i+8, y+30)], fill=(200, 200, 210, 255), outline=(120,120,130,255))
    # subtle shade
    dither_tile(x, y, (210,210,210,255), (160,160,160,255), step=4)

def draw_water(idx, frame):
    x,y = idx_to_xy(idx)
    base = (40, 120, 200, 200)  # semi-transparent for watery look
    fill_tile(x, y, base)
    # waves
    for yy in range(6, TILE_SIZE, 8):
        for xx in range(0, TILE_SIZE, 4):
            dx = int(2 * sin((xx + frame*4 + yy) * 0.3))
            draw.point((x+xx+dx, y+yy), fill=(180, 220, 255, 255))
            if xx % 8 == 0:
                draw.point((x+xx+dx, y+yy+1), fill=(120, 180, 240, 255))

def draw_lava(idx, frame):
    x,y = idx_to_xy(idx)
    base = (200, 60, 20, 255)
    fill_tile(x, y, base)
    # glow veins
    for yy in range(0, TILE_SIZE, 6):
        draw.line((x, y+yy, x+TILE_SIZE-1, y+yy), fill=(230, 120, 20, 255))
    # bubbles (animate position via frame)
    bubbles = [(8, 20), (20, 12), (12, 8)]
    for i, (bx, by) in enumerate(bubbles):
        bx = (bx + frame*3 + i*4) % TILE_SIZE
        draw.ellipse((x+bx-2, y+by-2, x+bx+2, y+by+2), fill=(255, 200, 80, 255), outline=(120,40,10,255))

def draw_portal(idx, frame):
    x,y = idx_to_xy(idx)
    # dark void background
    fill_tile(x, y, (30, 20, 40, 255))
    # swirling rings
    cx, cy = x + TILE_SIZE//2, y + TILE_SIZE//2
    for r in range(12, 2, -3):
        # rotate hue effect via frame offset in alpha/intensity
        alpha = 180 + int(60 * sin((r + frame*1.7)))
        col = (150 + (r*5) % 100, 100 + (r*3) % 120, 220, alpha)
        draw.ellipse((cx-r, cy-r, cx+r, cy+r), outline=col)
    # inner sparkle
    draw.point((cx + frame % 3 - 1, cy), fill=(255, 255, 255, 255))
    draw.point((cx, cy + (frame*2) % 3 - 1), fill=(220, 220, 255, 255))

# Build mapping per the user's spec (assuming 0-based tile indexing in the sheet).
MAPPING = [
    # (index, name, letter, properties, draw_fn)
    (1,  "Floor",  ".", "Walkable", lambda i: draw_floor(i)),
    (2,  "Grass",  "g", "Walkable", lambda i: draw_grass(i)),
    (3,  "Stone",  "s", "Walkable", lambda i: draw_stone(i)),
    (4,  "Wood",   "w", "Walkable", lambda i: draw_wood(i)),
    (17, "Wall",   "#", "Solid collision", lambda i: draw_wall(i)),
    (33, "Door",   "D", "Walkable, Interactive", lambda i: draw_door(i)),
    (37, "Chest",  "C", "Not walkable, Interactive", lambda i: draw_chest(i)),
    (41, "Spike",  "^", "Walkable but damages", lambda i: draw_spike(i)),
    # Animated sequences (4 frames each)
    # Water 45-48
    (45, "Water_f0", "~", "Animated (frame 0 of 4)", lambda i: draw_water(i, 0)),
    (46, "Water_f1", "~", "Animated (frame 1 of 4)", lambda i: draw_water(i, 1)),
    (47, "Water_f2", "~", "Animated (frame 2 of 4)", lambda i: draw_water(i, 2)),
    (48, "Water_f3", "~", "Animated (frame 3 of 4)", lambda i: draw_water(i, 3)),
    # Lava 49-52 (damages)
    (49, "Lava_f0", "L", "Animated, damages (frame 0 of 4)", lambda i: draw_lava(i, 0)),
    (50, "Lava_f1", "L", "Animated, damages (frame 1 of 4)", lambda i: draw_lava(i, 1)),
    (51, "Lava_f2", "L", "Animated, damages (frame 2 of 4)", lambda i: draw_lava(i, 2)),
    (52, "Lava_f3", "L", "Animated, damages (frame 3 of 4)", lambda i: draw_lava(i, 3)),
    # Portal 53-56 (interactive)
    (53, "Portal_f0", "P", "Animated, Interactive (frame 0 of 4)", lambda i: draw_portal(i, 0)),
    (54, "Portal_f1", "P", "Animated, Interactive (frame 1 of 4)", lambda i: draw_portal(i, 1)),
    (55, "Portal_f2", "P", "Animated, Interactive (frame 2 of 4)", lambda i: draw_portal(i, 2)),
    (56, "Portal_f3", "P", "Animated, Interactive (frame 3 of 4)", lambda i: draw_portal(i, 3)),
]

# Draw all mapped tiles
for idx, name, letter, props, fn in MAPPING:
    fn(idx)

# Save the clean tileset
tileset_path = "/Users/asudelevent/quests/gamedev/rust_roguelike/tileset_16x16_32px.png"
tileset.save(tileset_path, "PNG")

# Build a preview with grid + indices
preview = tileset.copy()
pdraw = ImageDraw.Draw(preview)
# semi-transparent checkerboard background so empty tiles are visible
for yy in range(0, IMG_H, 16):
    for xx in range(0, IMG_W, 16):
        if ((xx//16 + yy//16) % 2) == 0:
            pdraw.rectangle((xx, yy, xx+15, yy+15), fill=(220,220,220,255))

# Redraw tiles above the checkerboard
for idx, name, letter, props, fn in MAPPING:
    fn(idx)

# Grid lines
for gx in range(0, IMG_W+1, TILE_SIZE):
    pdraw.line((gx, 0, gx, IMG_H), fill=(0,0,0,120))
for gy in range(0, IMG_H+1, TILE_SIZE):
    pdraw.line((0, gy, IMG_W, gy), fill=(0,0,0,120))

# Tile indices text
try:
    font = ImageFont.load_default()
except:
    font = None

for idx in range(0, GRID_W*GRID_H):
    x,y = idx_to_xy(idx)
    label = str(idx)
    tw, th = pdraw.textbbox((0,0), label, font=font)[2:]
    # draw index with small background for readability
    pdraw.rectangle((x+1, y+1, x+tw+3, y+th+3), fill=(255,255,255,180))
    pdraw.text((x+2, y+2), label, fill=(0,0,0,255), font=font)

preview_path = "/Users/asudelevent/quests/gamedev/rust_roguelike/tileset_16x16_32px_preview.png"
preview.save(preview_path, "PNG")

# Export mapping CSV (for reference)
csv_path = "/mnt/data/tileset_mapping.csv"
with open(csv_path, "w", newline="", encoding="utf-8") as f:
    writer = csv.writer(f)
    writer.writerow(["Index", "Name", "Char", "Properties"])
    for idx, name, letter, props, _ in MAPPING:
        writer.writerow([idx, name, letter, props])

tileset_path, preview_path, csv_path
