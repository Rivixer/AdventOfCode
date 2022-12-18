from __future__ import annotations

import re
import sys
from typing import Generator
from dataclasses import dataclass

sys.setrecursionlimit(10000)


@dataclass
class Cube:
    x: int
    y: int
    z: int

    def __iter__(self) -> Generator[int, None, None]:
        yield from (self.x, self.y, self.z)

    def __hash__(self) -> int:
        return hash((self.x, self.y, self.z))

    @property
    def neighbors(self) -> tuple[Cube, ...]:
        return (
            Cube(self.x-1, self.y, self.z),
            Cube(self.x+1, self.y, self.z),
            Cube(self.x, self.y-1, self.z),
            Cube(self.x, self.y+1, self.z),
            Cube(self.x, self.y, self.z-1),
            Cube(self.x, self.y, self.z+1),
        )


with open('input.txt', 'r') as f:
    lines = f.readlines()

cubes: set[Cube] = set()

for line in lines:
    x, y, z = map(int, re.findall(r'\d+', line))
    cubes.add(Cube(x, y, z))

# part 1
result = 0
for cube in cubes:
    for neighbor in cube.neighbors:
        if neighbor not in cubes:
            result += 1
print(result)

# part 2
xs = tuple(map(lambda i: i.x, cubes))
ys = tuple(map(lambda i: i.y, cubes))
zs = tuple(map(lambda i: i.z, cubes))
min_x, max_x = min(xs), max(xs)
min_y, max_y = min(ys), max(ys)
min_z, max_z = min(zs), max(zs)
exteriors: set[Cube] = set()


def is_exterior(cube: Cube, history: set[Cube]) -> bool:
    if cube in cubes or cube in history:
        return False
    history.add(cube)

    x, y, z = cube
    if cube in exteriors or not (min_x < x < max_x) or not (min_y < y < max_y) or not (min_z < z < max_z):
        exteriors.update(history - cubes)
        return True

    for neighbor in cube.neighbors:
        if is_exterior(neighbor, history):
            return True
    return False


result = 0
for cube in cubes:
    for neighbor in cube.neighbors:
        if is_exterior(neighbor, set()):
            result += 1
print(result)
