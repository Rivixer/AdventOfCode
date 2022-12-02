from advent_of_code import AdventOfCode
import math
import time


class Day18(AdventOfCode):
    def __init__(self, test = False, part1 = True, part2 = True):
        self.test = test
        super().__init__(18, test)
        self.data = super().load_list_from_file(str, 'snailfish')

        if part1:
            self.part1()
        if part2:
            self.part2()

    def get_max_degree(self, data):
        max_degree = 0
        now_degree = 0
        for d in data:
            if d == '[':
                now_degree += 1
            elif d == ']':
                if now_degree > max_degree:
                    max_degree = now_degree
                now_degree -= 1
        return max_degree

    def any_number_greater_than_9(self, data):
        numbers = '0123456789'
        for i in range(len(data)):
            if data[i] in numbers and data[i+1] in numbers:
                return True
        return False

    def explode(self, data: str, max_degree: int) -> str:
        numbers = "0123456789"

        def get_tab_indexes_and_lengths():
            now_degree = 0
            i = 0
            while i < len(data):
                if data[i] == '[':
                    now_degree += 1
                elif data[i] == ']':
                    now_degree -= 1
                elif max_degree == now_degree:
                    first = i
                    first_length = 1 if data[i+1] not in numbers else 2
                    second = i + (2 if data[i+1] not in numbers else 3)
                    second_length = 1 if data[second + 1] not in numbers else 2
                    max_tab_indexes = [first,second]
                    max_tab_lengths = [first_length, second_length]
                    break
                i += 1
            return max_tab_indexes, max_tab_lengths

        def get_left_number_index_and_length():
            left_number_index: int = None
            left_number_length: int = 0
            i = max_tab_indexes[0] - 1
            while i > 0:
                if data[i] in numbers and data[i-1] not in numbers:
                    left_number_index = i
                    left_number_length = 1
                    break
                if data[i] in numbers and data[i- 1] in numbers:
                    left_number_index = i - 1
                    left_number_length = 2
                    break
                i -= 1
            return left_number_index, left_number_length

        def get_right_number_index_and_length():
            right_number_index: int = None
            right_number_length: int = 0
            i = max_tab_indexes[1] + max_tab_lengths[1] + 1
            while i < len(data):
                if data[i] in numbers and data[i + 1] not in numbers:
                    right_number_index = i
                    right_number_length = 1
                    break
                if data[i] in numbers and data[i + 1] in numbers:
                    right_number_index = i
                    right_number_length = 2
                    break
                i += 1
            return right_number_index, right_number_length

        max_tab_indexes, max_tab_lengths = get_tab_indexes_and_lengths()
        left_number_index, left_number_length = get_left_number_index_and_length()
        right_number_index, right_number_length = get_right_number_index_and_length()

        if left_number_index is not None:
            old_number = data[left_number_index:left_number_index + left_number_length]
            new_number = int(old_number) + int(data[max_tab_indexes[0]:max_tab_indexes[0] + max_tab_lengths[0]])
            data = data[:left_number_index] + str(new_number) + data[left_number_index+left_number_length:]

            max_tab_indexes, max_tab_lengths = get_tab_indexes_and_lengths()
            right_number_index, right_number_length = get_right_number_index_and_length()

        if right_number_index is not None:
            old_number = int(data[right_number_index:right_number_index+right_number_length])
            new_number = old_number + int(data[max_tab_indexes[1]:max_tab_indexes[1]+max_tab_lengths[1]])

            data = data[:right_number_index] + str(new_number) + data[right_number_index+right_number_length:]

        data = data[:max_tab_indexes[0]  -1] + '0' + data[max_tab_indexes[1]+max_tab_lengths[1]+1:]
        return data

    def addition(self, data: str, data2:str = None) -> str:
        if data2 is None:
            return '[' + data + ',' + self.data.pop(0) + ']'
        return '[' + data + ',' + data2 + ']'

    def split(self, data: str) -> str:
        numbers = '0123456789'
        for i in range(len(data)):
            if data[i] in numbers and data[i+1] in numbers:
                number = int(data[i]) * 10 + int(data[i+1])
                data = data[:i] + f'[{math.floor(number/2)},{math.ceil(number/2)}]' + data[i+2:]
                return data

    def magnitude(self, data: str):
        while True:
            try:
                stop = data.index(']')
            except ValueError:
                break
            start = abs(data[:stop][::-1].index('[') - stop + 1)
            tmp = data[start+1:stop]
            data = data[:start] + f'{3 * int(tmp.split(",")[0]) + 2 * int(tmp.split(",")[1])}' + data[stop+1:]
        return data

    def part1(self):
        data = self.data.pop(0)
        data = self.addition(data)

        while True:
            max_degree = self.get_max_degree(data)
            if max_degree > 4:
                data = self.explode(data, max_degree)
            elif self.any_number_greater_than_9(data):
                data = self.split(data)
            else:
                try:
                    data = self.addition(data)
                except IndexError:
                    break

        super().print_answer(1, self.magnitude(data))
        
    def part2(self):
        start = time.time()
        magnitudes = []
        for d1 in self.data:
            for d2 in self.data:
                if d1 == d2:
                    continue
                data = self.addition(d1, d2)
                while True:
                    max_degree = self.get_max_degree(data)
                    if max_degree > 4:
                        data = self.explode(data, max_degree)
                    elif self.any_number_greater_than_9(data):
                        data = self.split(data)
                    else:
                        break
                magnitudes.append(int(self.magnitude(data)))
        super().print_answer(2, max(magnitudes), time.time() - start)
