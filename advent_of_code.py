from typing import Type


class AdventOfCode:
    def __init__(self, day: int, test: bool = False):
        self.day = day
        self.test = test
        
    def load_list_from_file(self, type: Type, filename: str = 'file') -> list:
        filename = 'test.txt' if self.test else f'{filename}.txt'
        
        with open(f'Day01/{filename}') as f:
            result = list(map(type, f.readlines()))

        return result

    def print_answer(self, part: int,  answer):
        if self.test:
            print('TEST | ', end='')
        print(f'Day {self.day} / part {part}: {answer}')