from advent_of_code import AdventOfCode

class Day25(AdventOfCode):
    def __init__(self, test = False, part1 = True, part2 = True):
        self.test = test
        super().__init__(25, test)
        self.data = super().load_list_from_file(str, 'seacucumber')

        if part1:
            self.part1()
        if part2:
            self.part2()

    def part1(self):
        last_data = self.data.copy()
        index = 0
        while True:
            index += 1
            new_data = ['.'*len(self.data[0])]*len(self.data)

            for row in range(len(self.data)):
                for column in range(len(self.data[row])):
                    if self.data[row][column] == '>':
                        if column + 1 == len(self.data[row]):
                            if self.data[row][0] == '.':
                                new_data[row] = '>' + new_data[row][1:]
                            else:
                                new_data[row] = new_data[row][:column] + '>' + new_data[row][column+1:]
                        elif self.data[row][column + 1] == '.':
                            new_data[row] = new_data[row][:column] + '.>' + new_data[row][column+2:]
                        else:
                            new_data[row] = new_data[row][:column] + '>' + new_data[row][column+1:]
                    elif self.data[row][column] == 'v':
                        new_data[row] = new_data[row][:column] + 'v' + new_data[row][column+1:]

            self.data=new_data.copy()

            for row in range(len(self.data)):
                for column in range(len(self.data[row])):
                    if self.data[row][column] == 'v':
                        if row + 1 == len(self.data):
                            if self.data[0][column] == '.':
                                new_data[0] = new_data[0][:column] + 'v' + new_data[0][column+1:]
                                new_data[row] = new_data[row][:column] + '.' + new_data[row][column+1:]
                            else:
                                new_data[row] = new_data[row][:column] + 'v' + new_data[row][column+1:]
                        elif self.data[row+1][column] == '.':
                            new_data[row+1] = new_data[row+1][:column] + 'v' + new_data[row+1][column+1:]
                            new_data[row] = new_data[row][:column] + '.' + new_data[row][column+1:]
                        else:
                            new_data[row] = new_data[row][:column] + 'v' + new_data[row][column+1:]
                    elif self.data[row][column] == '>':
                        new_data[row] = new_data[row][:column] + '>' + new_data[row][column+1:]

            self.data=new_data.copy()

            if last_data == self.data:
                break

            last_data = self.data.copy()

        super().print_answer(1, index)

    def part2(self):
        pass