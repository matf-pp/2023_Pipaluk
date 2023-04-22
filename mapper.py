import pygame as pg
from pygame.color import Color
from pygame import Rect
from pygame.time import Clock
import json
from sys import argv

window = pg.display.set_mode((1200, 800))
clock = Clock()

class Tile:
    NONE = 'NONE'
    FLOOR = 'FLOOR'
    WALL = 'WALL'
    LIQUID = 'LIQUID'
colors = {
    Tile.NONE: Color(240, 240, 240),
    Tile.FLOOR: Color(128, 128, 128),
    Tile.WALL: Color(0, 0, 0),
    Tile.LIQUID: Color(0, 0, 255)
}
num_to_str = {
    0: Tile.NONE,
    1: Tile.FLOOR,
    2: Tile.WALL,
    3: Tile.LIQUID
}
str_to_num = {
    Tile.NONE: 0,
    Tile.FLOOR: 1,
    Tile.WALL: 2,
    Tile.LIQUID: 3
}

CELL = 30
try:
    with open(f'{argv[1]}') as file:
        obj =  json.load(file)
        # print(argv[1], obj)
        tilemap = obj['tilemap']
        tilemap = [
            [num_to_str[tile] for tile in row]
            for row in tilemap
        ]
except Exception:
    tilemap = [[Tile.FLOOR]]
print(tilemap)

brushes = [Tile.FLOOR, Tile.WALL, Tile.LIQUID]
brush = 0
pg.display.set_caption(f'Mapper | {brushes[brush]}')

def normalize_tilemap():
    global tilemap
    # remove empty rows on top
    while tilemap and tilemap[0].count(Tile.NONE) == len(tilemap[0]):
        tilemap.pop(0)
    # remove empty rows on bottom
    while tilemap and tilemap[-1].count(Tile.NONE) == len(tilemap[-1]):
        tilemap.pop(-1)
    if not tilemap:
        return
    # find longest row, and pad others to match
    longest = max([len(row) for row in tilemap])
    for row in tilemap:
        while len(row) < longest:
            row.append(Tile.NONE)
    # remove empty columns on the left
    while tilemap and all([row[0]==Tile.NONE for row in tilemap]):
        tilemap = [row[1:] for row in tilemap]
    # remove empty columns on the right
    while tilemap and all([row[-1]==Tile.NONE for row in tilemap]):
        tilemap = [row[:-1] for row in tilemap]

def pad_tilemap():
    global tilemap
    tilemap.insert(0, [Tile.NONE]*len(tilemap[0]))
    tilemap.append([Tile.NONE]*len(tilemap[0]))
    for row in tilemap:
        row.insert(0, Tile.NONE)
        row.append(Tile.NONE)

def set_tile(tile, row, col):
    global tilemap
    while row >= len(tilemap):
        tilemap.append([Tile.NONE])
    while col >= len(tilemap[row]):
        tilemap[row].append(Tile.NONE)
    tilemap[row][col] = tile

running = True
while running:
    for event in pg.event.get():
        match event.type:
            case pg.QUIT:
                running = False
            case pg.KEYDOWN if event.key == pg.K_ESCAPE:
                running = False
            case pg.MOUSEWHEEL:
                brush = (brush + 1) % len(brushes)
                pg.display.set_caption(f'Mapper | {brushes[brush]}')
            case pg.KEYDOWN if event.key == pg.K_n:
                normalize_tilemap()
                pad_tilemap()

    window.fill(Color('white'))

    if pg.mouse.get_pressed()[0]:
        x, y = pg.mouse.get_pos()
        row, col = y // CELL, x // CELL
        set_tile(brushes[brush], row, col)
        normalize_tilemap()
        pad_tilemap()
    if pg.mouse.get_pressed()[2]:
        x, y = pg.mouse.get_pos()
        row, col = y // CELL, x // CELL
        set_tile(Tile.NONE, row, col)
        normalize_tilemap()
        pad_tilemap()
    
    for i, row in enumerate(tilemap):
        for j, tile in enumerate(row):
            pg.draw.rect(window, colors[tile], Rect(CELL*j+1, CELL*i+1, CELL-1, CELL-1))

    pg.display.flip()
    clock.tick(30)

# tilemap = [
#     [str_to_num[tile] for tile in row]
#     for row in tilemap
# ]
# print(json.dumps(tilemap))

normalize_tilemap()
print('[')
for i, row in enumerate(tilemap):
    print(f'    {[str_to_num[tile] for tile in row]}{"," if i+1!=len(tilemap) else ""}')
print(']')