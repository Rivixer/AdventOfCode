from advent_of_code import AdventOfCode


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
            fuel = 0
            for d in data:
                fuel += abs(d - i)
            if min_fuel is None or fuel < min_fuel:
                min_fuel = fuel
        super().print_answer(1, min_fuel)


    def part2(self):
        pass