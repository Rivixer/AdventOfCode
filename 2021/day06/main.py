from advent_of_code import AdventOfCode


class Day06(AdventOfCode):
    def __init__(self, test = False, part1 = True, part2 = True):
        self.test = test
        super().__init__(6, test)
        self.data = super().load_list_from_file(int, 'data', splitter=',', oneline=True)
        if part1:
            self.part1()
        if part2:
            self.part2()

    def part1(self):
        data = self.data.copy()
        for _ in range(80):
            data = list(map(lambda i: i - 1, data))
            for i in range(len(data)):
                if data[i] == -1:
                    data[i] = 6
                    data.append(8)
        super().print_answer(1, len(data))

    def part2(self):
        dict = {
            -1: 0,
            0: 0,
            1: 0,
            2: 0,
            3: 0,
            4: 0,
            5: 0,
            6: 0,
            7: 0,
            8: 0,
        }
        data = self.data.copy()
        for d in data:
            dict[d] += 1
        for _ in range(256):
            for i, j in dict.copy().items():
                if i >= 0:
                    dict[i-1] = j
            dict[8] = dict.get(-1)
            dict[6] += dict.get(-1)
            dict[-1] = 0
        super().print_answer(2, sum(map(lambda i: i[1], dict.items())))

        

    