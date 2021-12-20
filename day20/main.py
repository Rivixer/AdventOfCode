from advent_of_code import AdventOfCode

class Day20(AdventOfCode):
    def __init__(self, test = False, part1 = True, part2 = True):
        self.test = test
        super().__init__(20, test)
        self.load_data()

        if part1:
            self.part1()
        if part2:
            self.part2()

    def load_data(self):
        lines = super().load_list_from_file(str, 'image')
        space_index = lines.index('')
        self.enhancement = ''.join(lines[:space_index])
        self.image = lines[space_index+1:]
    
    def get_character_from_enhancement(self, number: int):
        return self.enhancement[number]

    def convert_characters_to_dec_number(self, characters):
        return int(''.join(map(lambda i: '0' if i == '.' else '1', characters)), 2)

    def print_image(self, remove_sides=True):
        for i in (self.image[5:-5] if remove_sides else self.image):
            print(i[5:-5] if remove_sides else i)

    def add_ring_around_image(self):
        for i in range(len(self.image)):
            self.image[i] = '.' + self.image[i] + '.'
        self.image.insert(0, '.'*len(self.image[0]))
        self.image.append('.'*len(self.image[0]))

    def get_characters_around_image(self, row: int, column: int):
        ring = ((row-1, column-1), (row-1, column), (row-1, column+1),\
                 (row, column-1),   (row, column),   (row, column+1), \
                (row+1, column-1), (row+1, column), (row+1, column+1) )
        
        result = ""
        for r, c in ring:
            result += self.image[r][c]
        return result

    def get_empty_image(self):
        return ["."*(len(self.image))]*(len(self.image[0]))

    def update_str(self, string: str, index: int, character: str):
        return string[:index] + character + string[index+1:]

    def how_many_lit(self):
        result = 0
        for i in self.image[5:-5]:
            result += i[5:-5].count('#')
        return result

    def part1(self):
        for _ in range(5):
            self.add_ring_around_image()
        
        for _ in range(2):
            self.add_ring_around_image()
            new_image = self.get_empty_image()
            for row in range(1, len(self.image)-1):
                for column in range(1, len(self.image[row])-1):
                    characters = self.get_characters_around_image(row, column)
                    number = self.convert_characters_to_dec_number(characters)
                    character = self.get_character_from_enhancement(number)
                    new_image[row] = self.update_str(new_image[row], column, character)
            self.image = new_image

        super().print_answer(1, self.how_many_lit())

        self.print_image()

    def part2(self):
        pass