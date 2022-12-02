from typing import List
from advent_of_code import AdventOfCode

class Instruction:
    def __init__(self, *args):
        self.on = args[0] == 'on'
        self.x_start = args[1]
        self.x_stop = args[2]
        self.y_start = args[3]
        self.y_stop = args[4]
        self.z_start = args[5]
        self.z_stop = args[6]

    def __str__(self):
        return f"{self.on}, {self.x_start}, {self.x_stop}, {self.y_start}, {self.y_stop}, {self.z_start}, {self.z_stop}"

class Day22(AdventOfCode):
    def __init__(self, test = False, part1 = True, part2 = True):
        self.test = test
        super().__init__(22, test)
        self.load_data()

        if part1:
            self.part1()
        if part2:
            self.part2()

    def load_data(self, part2=False):
        with open(f'day22/{"test" if self.test else "reboot"}.txt') as f:
            lines = list(map(lambda i: i.strip(), f.readlines()))

        self.instructions: List[Instruction] = []
        for line in lines:
            on = line.split(' ')[0]
            x_start, x_stop = map(int,''.join(line.split(' ')[1:]).split(',')[0][2:].split('..'))
            y_start, y_stop = map(int,''.join(line.split(' ')[1:]).split(',')[1][2:].split('..'))
            z_start, z_stop = map(int,''.join(line.split(' ')[1:]).split(',')[2][2:].split('..'))
            if part2 is False:
                if x_start < -50 or x_stop > 50 or y_start < -50 or y_stop > 50 or z_start < -50 or z_stop > 50:
                    continue
            self.instructions.append(Instruction(on, x_start, x_stop, y_start, y_stop, z_start, z_stop))

    def part1(self):
        result = set()
        for index, i in enumerate(self.instructions):
            print(index, '/', len(self.instructions))
            for x in range(i.x_start, i.x_stop + 1):
                for y in range(i.y_start, i.y_stop + 1):
                    for z in range(i.z_start, i.z_stop + 1):
                        if i.on:
                            result.add((x, y, z))
                        else:
                            try:
                                result.remove((x, y, z))
                            except KeyError:
                                pass
        super().print_answer(1, len(result))

    def part2(self):
        pass