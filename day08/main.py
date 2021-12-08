from typing import List
from advent_of_code import AdventOfCode


class Day08(AdventOfCode):
    def __init__(self, test = False):
        self.test = test
        super().__init__(8, test)
        with open(f'Day08/{"test" if test else "segments"}.txt') as f:
            lines = f.readlines()

        self.data1: List[str] = list(map(lambda i: i.split(' ')[:10], lines))
        self.data2: List[str] = list(map(lambda i: i.split(' ')[11:-1] + [i.split(' ')[-1].strip()], lines))
        self.part1()

    def part1(self):
        counter = 0
        for line_number in range(len(self.data2)):
            d2 = list(map(lambda i: "".join(sorted(i)), self.data2[line_number]))
            for d in d2:
                if len(d) in (2, 4, 3, 7):
                    counter += 1
        print(counter)
