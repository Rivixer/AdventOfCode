import time
from typing import Tuple

from advent_of_code import AdventOfCode
from day15.priority_queue import PiorityQueue


class Day15(AdventOfCode):
    def __init__(self, test = False, part1 = True, part2 = True):
        self.test = test
        super().__init__(15, test)
        self._load_data()
        if part1:
            self.part1()
        if part2:
            self.part2()

    def _load_data(self):
        result = [[]]
        data = super().load_list_from_file(str, 'chitons')
        for d in data:
            for letter in d:
                result[-1].append(int(letter))
            result.append([])
        self.data = result[:-1]

    def _print_tab(self):
        for d in self.data:
            print(''.join(map(str, d)))

    def _get_neighbors(self, position: Tuple[int, int], end: int):
        result = []

        x, y = position
        for neighbor in ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)):
            if all(map(lambda i: 0 <= i <= end, neighbor)):
                result.append(neighbor)
                
        return result

    def part1(self):
        queue = PiorityQueue()
        queue.add((0,0), 0)

        end = len(self.data) - 1

        risk = {}
        risk[(0, 0)] = 0

        while queue:
            position = queue.get()
            if position == (end, end):
                break

            for neighbor in self._get_neighbors(position, end):
                new_risk = risk[position] + self.data[neighbor[0]][neighbor[1]]
                if neighbor not in risk or new_risk < risk[neighbor]:
                    risk[neighbor] = new_risk
                    priority = new_risk + sum(neighbor)
                    queue.add(neighbor, priority)

        super().print_answer(1, risk[end, end])

    def part2(self):
        start = time.time()
        def count_risk(position: Tuple[int, int]):
            x, y = position
            shift_x = x // original_tab_size
            shift_y = y // original_tab_size
            original_x = x % original_tab_size
            original_y = y % original_tab_size
            result = self.data[original_x][original_y] + shift_x + shift_y
            return result if result <= 9 else (result % 10) + 1

        queue = PiorityQueue()
        queue.add((0,0), 0)

        original_tab_size = len(self.data)
        end = len(self.data) * 5 - 1

        risk = {}
        risk[(0, 0)] = 0

        while queue:
            position = queue.get()
            if position == (end, end):
                break

            for neighbor in self._get_neighbors(position, end):
                new_risk = risk[position] + count_risk(neighbor)
                if neighbor not in risk or new_risk < risk[neighbor]:
                    risk[neighbor] = new_risk
                    priority = new_risk + sum(neighbor)
                    queue.add(neighbor, priority)

        super().print_answer(2, risk[end, end], time.time() - start)