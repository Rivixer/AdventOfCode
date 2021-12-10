from typing import List
from advent_of_code import AdventOfCode
from day10.stack import Stack


class Day10(AdventOfCode):
    def __init__(self, test = False):
        self.test = test
        super().__init__(10, test)
        self.data: List[str] = super().load_list_from_file(str, 'characters')
        self.characters = {'(': ')', '[': ']', '{': '}', '<': '>'}
        self.part1()
        #self.part2()

    def part1(self):
        lines = self.data.copy()
        wrong_chars = []
        for line in lines:
            stack = Stack()
            for char in line:
                if char in ('(', '[', '{', '<'):
                    stack.add_item(char)
                else:
                    open_item = stack.remove_last_item()
                    if self.characters.get(open_item) != char:
                        wrong_chars.append(char)
                        break
        table = {')': 3, ']': 57, '}': 1197, '>': 25137}
        result = sum(map(lambda i: table.get(i), wrong_chars))
        super().print_answer(1, result)


    