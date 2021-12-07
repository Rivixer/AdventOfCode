from advent_of_code import AdventOfCode
import time


class Day07(AdventOfCode):
    def __init__(self, test = False):
        self.test = test
        super().__init__(7, test)
        self.data = super().load_list_from_file(int, 'fuel', splitter=',', oneline=True)
        self.part1()
        self.part2()

    def part1(self):
        data = self.data.copy()
        min_fuel = None
        for i in range(min(self.data), max(self.data)):
            fuel = sum([abs(d-i) for d in data])
            if min_fuel is None or fuel < min_fuel:
                min_fuel = fuel
        super().print_answer(1, min_fuel)

    def _count_fuel(self, steps: int):
        return sum([i for i in range(steps+1)])
        

    def part2(self):
        start = time.time()
        data = self.data.copy()
        min_fuel = None
        for i in range(min(self.data), max(self.data)):
            fuel = sum([self._count_fuel(abs(d-i)) for d in data])
            if min_fuel is None or fuel < min_fuel:
                min_fuel = fuel
        stop = time.time()
        super().print_answer(2, min_fuel, stop-start)