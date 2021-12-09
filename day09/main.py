from typing import List
from advent_of_code import AdventOfCode


class Day09(AdventOfCode):
    def __init__(self, test = False):
        self.test = test
        super().__init__(9, test)
        self.data: List[List[int]] = super().load_list_from_file(str, 'heightmap')
        self._generate_mask()
        self.part1()
        self.part2()
            
    def _generate_mask(self):
        self.mask = []
        for _ in range(len(self.data)):
            self.mask.append([False] * len(self.data[0]))

    def part1(self):
        
        for j in range(len(self.data[0])):
            for i in range(len(self.data)):
                up = False
                down = False
                left = False
                right = False
                this = self.data[i][j]
                if i > 0:
                    if self.data[i-1][j] > this:
                        up = True
                else: up = True
                if i < len(self.data) - 1:
                    if self.data[i+1][j] > this:
                        down = True
                else: down = True
                if j > 0:
                    if self.data[i][j-1] > this:
                        left = True
                else: left = True
                if j < len(self.data[0]) - 1:
                    if self.data[i][j+1] > this:
                        right = True
                else: right = True
                if all([up, down, left, right]):
                    self.mask[i][j] = True

        count = 0
        for i in self.mask:
            count += sum(map(bool, i))
        for dd, mm in zip(self.data, self.mask):
            for d, m in zip(dd, mm):
                if m:
                    count += int(d)

        super().print_answer(1, count)

    def _recursion(self, i, j, start=True, count = 0):
        if i < 0:
            return count
        if j < 0:
            return count
        if i >= len(self.data):
            return count
        if j >= len(self.data[0]):
            return count
        if self.mask[i][j] is True and not start:
            return count
        if self.data[i][j] == '9':
            return count
        self.mask[i][j] = True
        count += 1
        count = self._recursion(i - 1, j, start=False, count=count)
        count = self._recursion(i + 1, j, start=False, count=count)
        count = self._recursion(i, j + 1, start=False, count=count)
        count = self._recursion(i, j - 1, start=False, count=count)
        return count

    def part2(self):
        basins_length = []
        for i in range(len(self.data)):
            for j in range(len(self.data[0])):
                if self.mask[i][j] is False:
                    continue
                basins_length.append(self._recursion(i, j))
        basins_length.sort(reverse=True)
        super().print_answer(2, basins_length[0]*basins_length[1]*basins_length[2])