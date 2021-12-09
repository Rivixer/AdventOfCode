from typing import Dict, List, Set, Tuple
from advent_of_code import AdventOfCode


class Day08(AdventOfCode):
    def __init__(self, test = False):
        self.test = test
        super().__init__(8, test)
        with open(f'Day08/{"test" if test else "segments"}.txt') as f:
            lines = f.readlines()

        self.data1: List[str] = list(map(lambda i: i.split(' ')[:10], lines))
        self.data2: List[str] = list(map(lambda i: i.split(' ')[11:-1] + [i.split(' ')[-1].strip()], lines))
        self.part1()
        self._reset_lists()
        self._signals_with_3_len = []
        self.part2_answer = 0
        self.part2()

    def part1(self):
        counter = 0
        for line_number in range(len(self.data2)):
            d2 = list(map(lambda i: "".join(sorted(i)), self.data2[line_number]))
            for d in d2:
                if len(d) in (2, 4, 3, 7):
                    counter += 1
        super().print_answer(1, counter)

    def part2(self):
        for line_number in range(len(self.data2)):
            d2 = list(map(lambda i: "".join(sorted(i)), self.data2[line_number]))
            d1 = list(map(lambda i: "".join(sorted(i)), self.data1[line_number]))
            self._reset_lists()
            for d in d2 + d1:
                self._calc_tab(d)
            self._calc_2()
            self._count_all_possibilities(d1, d2)
        super().print_answer(2, self.part2_answer)

    def _convert(self, item: str):
        result = ""
        for i in item:
            result += self.tab[i]
        a = ''.join(sorted(result))
        return a

    def _get_number(self, d: str):
        item = ''.join(sorted(d)) 
        if item == self._convert('abcdeg'):
            return 0
        if item == self._convert('ab'):
            return 1
        if item == self._convert('acdfg'):
            return 2
        if item == self._convert('abcdf'):
            return 3
        if item == self._convert('abef'):
            return 4
        if item == self._convert('bcdef'):
            return 5
        if item == self._convert('bcdefg'):
            return 6
        if item == self._convert('abd'):
            return 7
        if item == self._convert('abcdefg'):
            return 8
        if item == self._convert('abcdef'):
            return 9

    def _check_is_good(self, d1: List[str], d2: List[str]):
        answer = ""
        for i, d in enumerate(d1 + d2):
            if (a:= self._get_number(d)) is None:
                return False
            if i>= 10:
                answer += str(a)
        self.part2_answer += int(answer)
        return True

    def _count_all_possibilities(self, d1: List[str], d2: List[str]):
        pairs = []
        for i, j in self.tab.items():
            for a, b in self.tab.items():
                if a != i:
                    if j == b and [i ,a] not in pairs and [a, i] not in pairs:
                        pairs.append([i ,a])
        tab_copy = self.tab.copy()

        self.tab[pairs[0][0]] = self.tab[pairs[0][0]][0]
        self.tab[pairs[0][1]] = self.tab[pairs[0][1]][1]
        self.tab[pairs[1][0]] = self.tab[pairs[1][0]][0]
        self.tab[pairs[1][1]] = self.tab[pairs[1][1]][1]
        self.tab[pairs[2][0]] = self.tab[pairs[2][0]][0]
        self.tab[pairs[2][1]] = self.tab[pairs[2][1]][1]

        if self._check_is_good(d1, d2):
            return

        self.tab = tab_copy.copy()

        self.tab[pairs[0][0]] = self.tab[pairs[0][0]][0]
        self.tab[pairs[0][1]] = self.tab[pairs[0][1]][1]
        self.tab[pairs[1][0]] = self.tab[pairs[1][0]][0]
        self.tab[pairs[1][1]] = self.tab[pairs[1][1]][1]
        self.tab[pairs[2][0]] = self.tab[pairs[2][0]][1]
        self.tab[pairs[2][1]] = self.tab[pairs[2][1]][0]

        if self._check_is_good(d1, d2):
            return

        self.tab = tab_copy.copy()

        self.tab[pairs[0][0]] = self.tab[pairs[0][0]][0]
        self.tab[pairs[0][1]] = self.tab[pairs[0][1]][1]
        self.tab[pairs[1][0]] = self.tab[pairs[1][0]][1]
        self.tab[pairs[1][1]] = self.tab[pairs[1][1]][0]
        self.tab[pairs[2][0]] = self.tab[pairs[2][0]][0]
        self.tab[pairs[2][1]] = self.tab[pairs[2][1]][1]

        if self._check_is_good(d1, d2):
            return

        self.tab = tab_copy.copy()

        self.tab[pairs[0][0]] = self.tab[pairs[0][0]][0]
        self.tab[pairs[0][1]] = self.tab[pairs[0][1]][1]
        self.tab[pairs[1][0]] = self.tab[pairs[1][0]][1]
        self.tab[pairs[1][1]] = self.tab[pairs[1][1]][0]
        self.tab[pairs[2][0]] = self.tab[pairs[2][0]][1]
        self.tab[pairs[2][1]] = self.tab[pairs[2][1]][0]

        if self._check_is_good(d1, d2):
            return

        self.tab = tab_copy.copy()

        self.tab[pairs[0][0]] = self.tab[pairs[0][0]][1]
        self.tab[pairs[0][1]] = self.tab[pairs[0][1]][0]
        self.tab[pairs[1][0]] = self.tab[pairs[1][0]][0]
        self.tab[pairs[1][1]] = self.tab[pairs[1][1]][1]
        self.tab[pairs[2][0]] = self.tab[pairs[2][0]][0]
        self.tab[pairs[2][1]] = self.tab[pairs[2][1]][1]

        if self._check_is_good(d1, d2):
            return

        self.tab = tab_copy.copy()

        self.tab[pairs[0][0]] = self.tab[pairs[0][0]][1]
        self.tab[pairs[0][1]] = self.tab[pairs[0][1]][0]
        self.tab[pairs[1][0]] = self.tab[pairs[1][0]][0]
        self.tab[pairs[1][1]] = self.tab[pairs[1][1]][1]
        self.tab[pairs[2][0]] = self.tab[pairs[2][0]][1]
        self.tab[pairs[2][1]] = self.tab[pairs[2][1]][0]

        if self._check_is_good(d1, d2):
            return

        self.tab = tab_copy.copy()

        self.tab[pairs[0][0]] = self.tab[pairs[0][0]][1]
        self.tab[pairs[0][1]] = self.tab[pairs[0][1]][0]
        self.tab[pairs[1][0]] = self.tab[pairs[1][0]][1]
        self.tab[pairs[1][1]] = self.tab[pairs[1][1]][0]
        self.tab[pairs[2][0]] = self.tab[pairs[2][0]][0]
        self.tab[pairs[2][1]] = self.tab[pairs[2][1]][1]

        if self._check_is_good(d1, d2):
            return

        self.tab = tab_copy.copy()

        self.tab[pairs[0][0]] = self.tab[pairs[0][0]][1]
        self.tab[pairs[0][1]] = self.tab[pairs[0][1]][0]
        self.tab[pairs[1][0]] = self.tab[pairs[1][0]][1]
        self.tab[pairs[1][1]] = self.tab[pairs[1][1]][0]
        self.tab[pairs[2][0]] = self.tab[pairs[2][0]][1]
        self.tab[pairs[2][1]] = self.tab[pairs[2][1]][0]

        if self._check_is_good(d1, d2):
            return

    def _reset_lists(self):
        self.tab: Dict[int, Set[str]] = dict()
        letters = list('abcdefg')
        for i in letters:
            self.tab[i] = letters.copy()

    def _calc_tab(self, signal: str):
        def remove(index: str, letter: str):
            try:
                self.tab[index].remove(letter)
            except: pass

        for s in signal:
            if len(signal) == 3:
                remove('e', s)
                remove('f', s)
                remove('g', s)
                remove('c', s)
                self._signals_with_3_len.append(signal)
            if len(signal) == 2:
                remove('d', s)
                remove('e', s)
                remove('f', s)
                remove('g', s)
                remove('c', s)
            if len(signal) == 4:
                remove('d', s)
                remove('g', s)
                remove('c', s)

    def _index_with_length(self, lenght: int) -> List[str]:
        result = []
        for j, i in self.tab.items():
            if len(i) == lenght:
                result.append(j)
        return result

    def _delete_in_tab(self, what: List[str], *exc):
        for i in self.tab.keys():
            if i not in exc:
                for w in what:
                    try:
                        self.tab[i].remove(w)
                    except: pass
    
    def _calc_2(self):
        l3 = self._index_with_length(3)[0]
        l2a = self._index_with_length(2)
        l2 = l2a[0]

        for l in self.tab[l2]:
            try:
                self.tab[l3].remove(l)
            except: pass
        self._delete_in_tab(self.tab[l3], l3)
        self._delete_in_tab(self.tab[l2], l2a[0], l2a[1])

        l2a2 = self._index_with_length(2)
        for l in l2a:
            try:
                l2a2.remove(l)
            except:pass

        self._delete_in_tab(self.tab[l2a2[0]], l2a2[0], l2a2[1])
        self.tab['d'] = self.tab['d'][0]
        
    def _get_number_in_screen_using_len(self, signal: str):
        if len(signal) == 2:
            return 1
        if len(signal) == 4:
            return 4
        if len(signal) == 3:
            return 7
        if len(signal) == 7:
            return 8

    def _print_screens_list(self):
        for i, j in self.tab.items():
            print(i, j)
        print(self.part2_answer)
        