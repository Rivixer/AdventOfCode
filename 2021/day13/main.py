from typing import List, Literal
from advent_of_code import AdventOfCode


class Day13(AdventOfCode):
    def __init__(self, test = False, part1 = True, part2 = True):
        self.test = test
        super().__init__(13, test)
        self._load_data()
        self._generate_tab()
        if part1:
            self.part1()
        if part2:
            self.part2()

    def _load_data(self):       
        with open(f'day13/{"test" if self.test else "manual"}.txt') as f:
            lines = f.readlines()
        space_index = lines.index('\n')
        self.coordinates = list(map(lambda i: list(map(int, i.strip().split(','))), lines[:space_index]))
        self.instructions = list(map(lambda i: i.strip(), lines[space_index + 1:]))

    def _generate_tab(self):
        self.maxX = max(map(lambda i: i[0], self.coordinates)) + 1
        self.maxY = max(map(lambda i: i[1], self.coordinates)) + 1
        self.tab: List[List[Literal['.', '#']]] = []
        for _ in range(self.maxY):
            self.tab.append([])
            for _ in range(self.maxX):
                self.tab[-1].append('.')   

    def _print_tab(self):
        for y in self.tab:
            for x in y:
                print(x, end='')
            print()

    def _merge(self, tab1, tab2):
        result = []
        for t1, t2 in zip(tab1, tab2):
            if '#' in (t1, t2):
                result.append('#')
            else:
                result.append('.')
        return result

    def _fold(self, X = None, Y = None):
        if Y is not None:
            for x, y in zip(range(self.maxX), range(self.maxY - 1, Y, -1)):
                self.tab[x] = self._merge(self.tab[x], self.tab[y])
                self.tab.pop(-1)
            self.tab.pop(-1)
            self.maxY = Y

        if X is not None:
            for y in range(self.maxY):
                left = self.tab[y][:X+1]
                right = self.tab[y][X:]
                self.tab[y] = self._merge(left, right[::-1])
            for t in self.tab:
                t.pop(-1)
            self.maxX = X

    def _count_hashtags(self):
        result = 0
        for y in self.tab:
            result += sum(map(lambda i: i == '#', y))
        return result
    
    def part1(self):
        coordinates = self.coordinates
        tab = self.tab
        for coordinate in coordinates:
            c1, c2 = coordinate
            tab[c2][c1] = '#'
        
        index, count = self.instructions[0].split(' ')[2].split('=')
        if index == 'x':
            self._fold(X=int(count))
        else:
            self._fold(Y=int(count))
        super().print_answer(1, self._count_hashtags())

    def part2(self):
        self._generate_tab()
        coordinates = self.coordinates
        tab = self.tab
        for coordinate in coordinates:
            c1, c2 = coordinate
            tab[c2][c1] = '#'

        for instruction in self.instructions:
            index, count = instruction.split(' ')[2].split('=')
            if index == 'x':
                self._fold(X=int(count))
            else:
                self._fold(Y=int(count))
        super().print_answer(2, self.tab)