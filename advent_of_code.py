from typing import Type


class AdventOfCode:
    def __init__(self, test: bool = False):
        self.test = test

    def load_list_from_file(self, type: Type) -> list:
        filename = 'test.txt' if self.test else 'measurements.txt'
        
        with open(f'Day01/{filename}') as f:
            measurements = list(map(type, f.readlines()))

        return measurements

    def print_answer(self, answer):
        if self.test:
            print('TEST | ', end='')
        print(f'Day 01 / part 1: {answer}')