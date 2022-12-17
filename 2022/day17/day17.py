from typing import Generator
from dataclasses import dataclass
from copy import deepcopy
import time

t = time.perf_counter()


@dataclass
class Shape:
    positions: list[list[int]]

    def shift_left(self) -> None:
        for stone in self.positions:
            stone[0] -= 1

    def shift_right(self) -> None:
        for stone in self.positions:
            stone[0] += 1

    def shift_down(self) -> None:
        for stone in self.positions:
            stone[1] += 1


class Grid:
    stones: list[list[int]] = [[i, 3] for i in range(7)]

    def shift_down(self, height: int) -> None:
        for stone in self.stones:
            stone[1] += height

    def can_shift_down(self, shape: Shape) -> bool:
        for stone in reversed(self.stones):
            for position in shape.positions:
                if stone == [position[0], position[1]+1]:
                    return False
        return True

    def can_shift_right(self, shape: Shape) -> bool:
        for pos in shape.positions:
            if pos[0] == 6:
                return False

        for stone in reversed(self.stones):
            for position in shape.positions:
                if stone == [position[0] + 1, position[1]]:
                    return False
        return True

    def can_shift_left(self, shape: Shape) -> bool:
        for pos in shape.positions:
            if pos[0] == 0:
                return False

        for stone in reversed(self.stones):
            for position in shape.positions:
                if stone == [position[0] - 1, position[1]]:
                    return False
        return True

    def add(self, shape: Shape) -> None:
        for position in shape.positions:
            self.stones.append(position)


shapes = [
    Shape([[2, 0], [3, 0], [4, 0], [5, 0]]),
    Shape([[3, 2], [2, 1], [3, 1], [4, 1], [3, 0]]),
    Shape([[2, 2], [3, 2], [4, 2], [4, 1], [4, 0]]),
    Shape([[2, 3], [2, 2], [2, 1], [2, 0]]),
    Shape([[2, 1], [3, 1], [2, 0], [3, 0]]),
]


def get_shape() -> Generator[Shape, None, None]:
    while True:
        yield from deepcopy(shapes)


def get_shift() -> Generator[str, None, None]:
    with open('input.txt', 'r') as f:
        shifts = f.readline().strip()

    while True:
        yield from shifts


grid = Grid()
shape_gen = get_shape()
shift_gen = get_shift()


for i in range(2022):
    shape = next(shape_gen)
    grid.shift_down(max(map(lambda i: i[1], shape.positions))+1)

    if i % 100 == 0:
        print('{}/2022 {:.2f} %'.format(i, i/20.22))

    while True:
        shift = next(shift_gen)

        if shift == '>':
            if grid.can_shift_right(shape):
                shape.shift_right()
        elif shift == '<':
            if grid.can_shift_left(shape):
                shape.shift_left()

        if not grid.can_shift_down(shape):
            break

        shape.shift_down()

    grid.add(shape)
    grid.shift_down(3-min(map(lambda i: i[1], grid.stones)))

print(max(map(lambda i: i[1], grid.stones)) - 3)
print(time.perf_counter() - t)
