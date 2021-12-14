from typing import Dict, List
from advent_of_code import AdventOfCode


class Day14(AdventOfCode):
    def __init__(self, test = False, part1 = True, part2 = True):
        self.test = test
        super().__init__(14, test)
        self._load_data()
        if part1:
            self.part1()
        if part2:
            self.part2()

    def _load_data(self):
        with open(f'day14/{"test" if self.test else "polymer"}.txt') as f:
            lines = list(map(lambda i: i.strip(), f.readlines()))

        self.template: str = lines[0]
        instructions: Dict[str, str] = {}
        for line in lines[2:]:
            l1, l2 = line.split(' -> ')
            instructions[l1] = l2
        self.instructions = instructions

    def part1(self):
        for _ in range(10):
            temp = self.template[0]
            for l1, l2 in zip(self.template, self.template[1:]):
                temp += self.instructions.get(l1 + l2)
                temp += l2
            self.template = temp

        maximum = None
        minimum = None
        for char in set(self.template):
            count = sum(map(lambda i: i == char, self.template))
            if minimum is None or count < minimum:
                minimum = count
            if maximum is None or maximum < count:
                maximum = count

        super().print_answer(1, maximum - minimum)
                

    def part2(self):
        self._load_data()
        memory: Dict[str, int] = dict()
        letters: Dict[str, int] = dict()

        for l1, l2 in zip(self.template, self.template[1:]):
            if l1 + l2 in memory.keys():
                memory[l1+l2] += 1
            else:
                memory[l1+l2] = 1
                
        for letter in self.template:
            if letter in letters.keys():
                letters[letter] += 1 
            else:
                letters[letter] = 1 

        for _ in range(40):
            new_memory: Dict[str, int] = memory.copy()
            for key, value in memory.items():

                new_item = self.instructions.get(key)
                new1 = key[0] + new_item
                new2 = new_item + key[1]

                if new_item in letters.keys():
                    letters[new_item] += value
                else:
                    letters[new_item] = value

                if new1 in new_memory.keys():
                    new_memory[new1] += value
                else :
                    new_memory[new1] = value
                if new2 in new_memory.keys():
                    new_memory[new2] += value
                else :
                    new_memory[new2] = value

                new_memory[key] -= value
                if new_memory[key] == 0:
                    new_memory.pop(key)

            memory = new_memory
        
        maximum = max(letters.values())
        minimum = min(letters.values())

        super().print_answer(2, maximum - minimum)