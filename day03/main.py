from advent_of_code import AdventOfCode


class Day03(AdventOfCode):
    def __init__(self, test = False):
        super().__init__(3, test)
        self.bits = super().load_list_from_file(str, 'bits')
        self.part1()
        #self.part2()

    def part1(self):
        gamma = ""
        for k in range(len(self.bits[0])):
            gamma += '1' if sum(map(lambda i: int(i[k]), self.bits)) > len(self.bits) // 2 else '0'

        gamma = int(gamma, 2)
        epsilon = ~gamma & ((1 << len(self.bits[0])) - 1)
        super().print_answer(1, f'{str(bin(gamma))[2:]} ({gamma}) | {str(bin(epsilon))[2:]} ({epsilon}) => {gamma * epsilon}')
