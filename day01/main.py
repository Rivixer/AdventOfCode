from advent_of_code import AdventOfCode


class Day01(AdventOfCode):
    def __init__(self, test = False):
        super().__init__(test)
        self.measurements = super().load_list_from_file(int)
        self.part1()

    def part1(self):
        counter = 0
        for i in range(1, len(self.measurements)):
            if (self.measurements[i] > self.measurements[i-1]):
                counter +=1

        super().print_answer(counter)
