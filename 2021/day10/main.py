from typing import List
from advent_of_code import AdventOfCode
from day10.stack import Stack


class Day10(AdventOfCode):
    def __init__(self, test = False):
        self.test = test
        super().__init__(10, test)
        self.data: List[str] = super().load_list_from_file(str, 'characters')
        self.characters = {'(': ')', '[': ']', '{': '}', '<': '>'}
        self.part1and2()

    def part1and2(self):
        lines = self.data.copy()
        wrong_chars = []
        missing_chars = []
        for line in lines:
            stack = Stack()
            for char in line:
                if char in ('(', '[', '{', '<'):
                    stack.add_item(char)
                else:
                    open_item = stack.remove_last_item()
                    if self.characters.get(open_item) != char:
                        wrong_chars.append(char)
                        self.data.remove(line)
                        break
            else:
                missing_chars.append(list(map(lambda i: self.characters.get(i), stack.get_all_items()[::-1])))

        table = {')': 3, ']': 57, '}': 1197, '>': 25137}
        result = sum(map(lambda i: table.get(i), wrong_chars))
        super().print_answer(1, result)

        scores = []
        table = {')': 1, ']': 2, '}': 3, '>': 4}
        for missing_char in missing_chars:
            result = 0
            for char in missing_char:
                result *= 5
                result += table.get(char)
            scores.append(result)

        scores.sort()
        super().print_answer(2, scores[len(scores)//2])



    