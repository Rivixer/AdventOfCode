from typing import List
from advent_of_code import AdventOfCode


class Day04(AdventOfCode):
    def __init__(self, test = False):
        self.test = test
        super().__init__(4, test)
        self.numbers = self._load_numbers()
        self.tabs = self._load_tabs()
        self.part1()

    def _load_numbers(self):
        with open(f'day04/{"bingo" if not self.test else "test"}.txt') as f:
            numbers = list(map(lambda i: int(i), f.readlines()[0].split(',')))
        return numbers

    def _load_tabs(self):
        with open(f'day04/{"bingo" if not self.test else "test"}.txt') as f:
            lines = f.readlines()[2:]

        def converter(i):
            try:
                return int(i)
            except:
                return -1

        tabs: List[List] = []
        for line in lines:
            x = (list(map(lambda i: converter(i), line.split(' '))))
            while True:
                try:
                    x.remove(-1)
                except ValueError:
                    break
            tabs.append(x)

        i=0
        Ttabs = []
        for tab in tabs:
            if not len(tab):
                i += 1
                continue
            try:
                Ttabs[i] += [tab]
            except IndexError:
                Ttabs.append([tab])

        return Ttabs

    def _check_win(self) -> int:
        """Return number board if true (first board -> 0)"""

        for index, tab in enumerate(self.tabs):
            for t in tab:
                if all(map(lambda i: i == -1, t)):
                    return index

        for index, tab in enumerate(self.tabs):
            for k in range(5):
                if all(map(lambda i: i[k] == -1, tab)):
                    return index

        return False

    def _get_sum_of_unmarked(self, board_index):
        result = 0
        tabs = self._load_tabs()
        for ta1, ta2 in zip(tabs[board_index], self.tabs[board_index]):
            for t1, t2 in zip(ta1, ta2):
                if (t2 != -1):
                    result += t1
        return result

    def part1(self):
        new_tab = self.tabs.copy()
        for number in self.numbers:
            for tab in range(len(new_tab)):
                for t in range(len(new_tab[tab])):
                    new_tab[tab][t] = (list(map(lambda i: -1 if i == number or i == -1 else i, new_tab[tab][t])))
            if (board_index := self._check_win()) is not False :
                break
        sum_of_unmarked = self._get_sum_of_unmarked(board_index)
        super().print_answer(1, sum_of_unmarked * number)