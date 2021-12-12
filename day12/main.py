from typing import List
from advent_of_code import AdventOfCode
from day12.cave import Cave
from day12.stack import Stack


class Day12(AdventOfCode):
    def __init__(self, test = False):
        self.test = test
        super().__init__(12, test)
        self.caves: List[Cave] = []
        self.part1_answer = 0
        self.part2_answer = 0
        self._load_data()
        self.part1()
        self.part2()

    def _check_in_caves(self, cave: str):
        for c in self.caves:
            if str(c) == cave:
                return True
        return False

    def _get_cave_by_name(self, name: str):
        for c in self.caves:
            if str(c) == name:
                return c

    def _load_data(self):
        lines: List[List[str]] = super().load_list_from_file(str, 'paths')
        for line in lines:
            c1, c2 = line.split('-')
            if not self._check_in_caves(c1):
                self.caves.append(Cave(c1))
            if not self._check_in_caves(c2):
                self.caves.append(Cave(c2))
            cave1 = self._get_cave_by_name(c1)
            cave2 = self._get_cave_by_name(c2)
            cave1.add_connection(cave2)
            
        for cave in self.caves:
            for conn in cave.connections:
                c = self._get_cave_by_name(str(conn))
                c.add_connection(cave)

    def pathing(self, cave: Cave, s: Stack):
        last_stack = Stack(s.get_all_items().copy())
        i: Cave
        for i in cave.connections:
            stack = Stack(last_stack.get_all_items().copy())
            if i.is_small and i in stack.get_all_items() or str(i) == 'start':
                continue
            stack.add_item(i)
            if str(i) != 'end':
                self.pathing(i, stack)
            else:
                self.part1_answer += 1

    def _how_much_cave_in_path(self, stack: Stack, cave):
        result = 0
        for c in stack.get_all_items():
            if c == cave:
                result += 1
        return result

    def pathing2(self, cave: Cave, s: Stack):
        last_stack = Stack(s.get_all_items().copy(), s.visited_small_2_times)
        i: Cave
        for i in cave.connections:
            stack = Stack(last_stack.get_all_items().copy(), s.visited_small_2_times)
            if i.is_small and i in stack.get_all_items() and stack.visited_small_2_times or str(i) == 'start':
                continue
            stack.add_item(i)
            if i.is_small and not stack.visited_small_2_times and self._how_much_cave_in_path(stack, i) == 2:
                stack.visited_small_2_times = True
            if str(i) != 'end':
                self.pathing2(i, stack)
            else:
                self.part2_answer += 1
            
    def part1(self):
        stack = Stack()
        start = self._get_cave_by_name('start')
        stack.add_item(start)
        self.pathing(start, stack)
        super().print_answer(1, self.part1_answer)

    def part2(self):
        stack = Stack()
        start = self._get_cave_by_name('start')
        stack.add_item(start)
        self.pathing2(start, stack)
        super().print_answer(2, self.part2_answer)