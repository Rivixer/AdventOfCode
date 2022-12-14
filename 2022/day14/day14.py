import re
from dataclasses import dataclass


@dataclass(slots=True)
class Cave:
    grid: dict[tuple[int, int], str]
    abyss: int
    part: int

    def __can_drop(self, x: int, y: int) -> bool:
        if y >= self.abyss:
            return False
        try:
            return self.grid[(x, y)] not in ('#', 'o')
        except:
            return True

    def drop(self, x: int, y: int) -> bool:
        if x == 500 and y == 0 and self.grid[(x, y)] == 'o':
            return False
        if self.__can_drop(x, y+1):
            return self.drop(x, y+1)
        if self.__can_drop(x-1, y+1):
            return self.drop(x-1, y+1)
        if self.__can_drop(x+1, y+1):
            return self.drop(x+1, y+1)
        if self.part == 1 and y + 1 == self.abyss:
            return False
        self.grid[(x, y)] = 'o'
        return True


def solve(part: int):
    i = 0
    cave = Cave(grid.copy(), abyss + 2, part)
    while cave.drop(500, 0):
        i += 1
    print(i)


if __name__ == '__main__':
    with open('input.txt') as f:
        lines = f.readlines()

    grid = dict()
    for line in lines:
        points = re.findall(r'\d+,\d+', line)
        for i in range(len(points) - 1):
            x1, y1 = map(int, re.findall(r'\d+', points[i]))
            x2, y2 = map(int, re.findall(r'\d+', points[i+1]))
            for x in range(min(x1, x2), max(x1, x2) + 1):
                for y in range(min(y1, y2), max(y1, y2) + 1):
                    grid[(x, y)] = '#'

    grid[(500, 0)] = '.'
    abyss = max(map(lambda k: k[1], grid.keys()))

    solve(1)
    solve(2)
