from advent_of_code import AdventOfCode


class Day03(AdventOfCode):
    def __init__(self, test = False):
        super().__init__(3, test)
        self.bits = super().load_list_from_file(str, 'bits')
        self.part1()
        self.part2()

    def part1(self):
        gamma = ""
        for k in range(len(self.bits[0])):
            gamma += '1' if sum(map(lambda i: int(i[k]), self.bits)) > len(self.bits) // 2 else '0'

        gamma = int(gamma, 2)
        epsilon = ~gamma & ((1 << len(self.bits[0])) - 1)
        super().print_answer(1, f'{str(bin(gamma))[2:]} ({gamma}) | {str(bin(epsilon))[2:]} ({epsilon}) => {gamma * epsilon}')


    def part2(self):
        oxygen = self.bits.copy()
        for k in range(len(self.bits[0])):
            if len(oxygen) == 1: break
            if len(oxygen) == 2:
                oxygen = list(filter(lambda i: i[k] == '1', oxygen))
            else:
                oxygen = list(filter(lambda i: i[k] == ('1' if sum(map(lambda i: int(i[k]), oxygen))  > len(oxygen) // 2 else '0'), oxygen))
        
        co2 = self.bits.copy()
        for k in range(len(self.bits[0])):
            if len(co2) == 1: break
            if len(co2) == 2:
                co2 = list(filter(lambda i: i[k] == '0', co2))
            else:
                co2 = list(filter(lambda i: i[k] == ('1' if sum(map(lambda i: int(i[k]), co2)) < len(co2) // 2 else '0'), co2))

        super().print_answer(2, f'{oxygen[0]} ({int(oxygen[0], 2)}), {co2[0]} ({int(co2[0], 2)}) => {int(oxygen[0], 2)*int(co2[0], 2)}')