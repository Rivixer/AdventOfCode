from advent_of_code import AdventOfCode


class Day05(AdventOfCode):
    def __init__(self, test = False):
        self.test = test
        super().__init__(5, test)
        self.tabs = self._load_tabs_from_file()
        self.mask = self._generate_mask()
        self.part1()
        self.part2()

    def _load_tabs_from_file(self):
        with open(f'day05/{"test" if self.test else "vent"}.txt') as f:
            lines = f.readlines()
        
        tabs = list(map(lambda i: i.strip().split(' -> '), lines))

        return tabs

    def _generate_mask(self):
        amount = 10 if self.test else 1000
        return [[0]*amount for _ in range(amount)]

    def _count_mask(self, startX, startY, stopX, stopY):
        minX = min(startX, stopX)
        maxX = max(startX, stopX) + 1
        minY = min(startY, stopY)
        maxY = max(startY, stopY) + 1
        if startX == stopX:
            for y in range(minY, maxY):
                self.mask[y][startX] += 1
        elif startY == stopY:
            for x in range(minX, maxX):
                self.mask[startY][x] += 1
        elif stopY - startY == stopX - startX:
            for x, y in zip(range(minX, maxX), range(minY, maxY)):
                self.mask[y][x] += 1
        elif abs(stopY - startY) == stopX - startX:
            for x, y in zip(range(minX, maxX), range(maxY-1, minY-1, -1)):
                self.mask[y][x] += 1
        elif stopY - startY == abs(stopX - startX):
            for x, y in zip(range(maxX-1, minX-1, -1), range(minY, maxY)):
                self.mask[y][x] += 1
        elif abs(stopY - startY) == abs(stopX - startX):
            for x, y in zip(range(maxX-1, minX-1, -1), range(maxY-1, minY-1, -1)):
                self.mask[y][x] += 1


    def _print_mask(self):
        for m in self.mask:
            print(m)

    def _count_more_than1(self):
        result = 0
        for m in self.mask:
            result += sum(map(lambda i: 1 if i >= 2 else 0, m))
        return result

    def part1(self):
        for tab in self.tabs:
            startX = int(tab[0].split(',')[0])
            startY = int(tab[0].split(',')[1])
            stopX = int(tab[1].split(',')[0])
            stopY = int(tab[1].split(',')[1])
            if startX != stopX and startY != stopY:
                continue
            self._count_mask(startX, startY, stopX, stopY)
        super().print_answer(1, self._count_more_than1())

    def part2(self):
        self.mask = self._generate_mask()
        for tab in self.tabs:
            startX = int(tab[0].split(',')[0])
            startY = int(tab[0].split(',')[1])
            stopX = int(tab[1].split(',')[0])
            stopY = int(tab[1].split(',')[1])
            self._count_mask(startX, startY, stopX, stopY)
        super().print_answer(2, self._count_more_than1())
            
