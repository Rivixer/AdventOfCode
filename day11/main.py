from types import resolve_bases
from typing import List
from advent_of_code import AdventOfCode
from day11.octopus import Octopus


class Day11(AdventOfCode):
    def __init__(self, test = False):
        self.test = test
        super().__init__(11, test)
        self.data = self._load_data()
        self.octopuses = self._load_octopuses()
        self.i = 0
        self.part1()
        self.part2()
        
    def _load_data(self):
        data: List[List[int]] = [[]]
        for line in super().load_list_from_file(str, 'octopuses'):
            for char in line:
                data[-1].append(char)
            data.append([])
        return data[:-1]

    def _load_octopuses(self):
        octopuses: List[List[Octopus]] = [[]]
        for line in self.data:
            for number in line:
                octopuses[-1].append(Octopus(int(number)))
            octopuses.append([])
        return octopuses[:-1]

    def _next_step_in_all(self):
        for i in self.octopuses:
            for j in i:
                j.next_step()

    def _add_around(self, column, row):
        def add(c, r):
            try:
                octopus = self.octopuses[c][r]
            except IndexError:
                return

            if not octopus.flashed_in_this_round():
                octopus.add_energy()

        if column > 0:
            add(column - 1, row + 1)
            add(column - 1, row)
            if row > 0:
                add(column - 1, row - 1)
        if row > 0:
            add(column, row - 1)
        add(column, row + 1)
        if row > 0:
            add(column + 1, row - 1)
        add(column + 1, row)
        add(column + 1, row + 1)

    def _any_can_flash(self):
        result = []
        for i in self.octopuses:
            for j in i:
                result.append(j.can_flash())
        return any(result)

    def part1(self):
        for _ in range(100):
            self._next_step_in_all()
            while self._any_can_flash():
                for column in range(len(self.octopuses)):
                    for row in range(len(self.octopuses[column])):
                        if self.octopuses[column][row].can_flash():
                            self.octopuses[column][row].flash()
                            self._add_around(column, row)

        result = 0
        for line in self.octopuses:
            result += sum(list(map(lambda i: i._flash_count, line)))
        super().print_answer(1, result)

    def _print_octopuses(self):
        for column in range(len(self.octopuses)):
            print(self.octopuses[column])

    def _are_synchronized(self):
        result = []
        for line in self.octopuses:
            result.append(all(map(lambda i: i.flashed_in_this_round(), line)))
        return all(result)


    def part2(self):
        i = 100
        while not self._are_synchronized():
            i += 1
            self._next_step_in_all()
            while self._any_can_flash():
                for column in range(len(self.octopuses)):
                    for row in range(len(self.octopuses[column])):
                        if self.octopuses[column][row].can_flash():
                            self.octopuses[column][row].flash()
                            self._add_around(column, row)
        super().print_answer(2, i)
        