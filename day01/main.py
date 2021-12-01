from advent_of_code import AdventOfCode


class Day01(AdventOfCode):
    def __init__(self, test = False):
        super().__init__(1, test)
        self.measurements = super().load_list_from_file(int, 'measurements')
        self.part1()
        self.part2()

    def part1(self):
        counter = 0
        for i in range(1, len(self.measurements)):
            if (self.measurements[i] > self.measurements[i-1]):
                counter +=1

        super().print_answer(1, counter)

    def part2(self):
        last_sum = sum(self.measurements[:3])
        counter = 0 

        for a, b ,c in zip(self.measurements[1:-2], self.measurements[2:-1], self.measurements[3:]):
            this_sum = sum([a, b, c])
            if this_sum > last_sum:
                counter += 1
            last_sum = this_sum

        super().print_answer(2, counter)