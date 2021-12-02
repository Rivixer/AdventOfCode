from advent_of_code import AdventOfCode


class Day02(AdventOfCode):
    def __init__(self, test = False):
        super().__init__(2, test)
        self.instructions = super().load_list_from_file(str, 'data', 0)
        self.numbers = super().load_list_from_file(int, 'data', 1)
        self.part1()

    def part1(self):       
        horizontal = 0
        depth = 0

        for i, n in zip(self.instructions, self.numbers):
            if i == 'forward':
                horizontal += n
            elif i == 'down':
                depth += n
            elif i == 'up':
                depth -= n

        super().print_answer(1, f'{horizontal=}, {depth=} ({horizontal*depth})')