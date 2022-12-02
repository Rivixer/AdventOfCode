from advent_of_code import AdventOfCode


class Day17(AdventOfCode):
    def __init__(self, test = False, part1 = True, part2 = True):
        self.test = test
        super().__init__(17, test)
        self._load_data()
        if part1:
            self.part1()
        if part2:
            self.part2()

    def _load_data(self):
        data = super().load_list_from_file(str, 'trajectory', oneline=True)
        self.x_start, self.x_stop = map(int, data[2][2:-1].split('..'))
        self.y_stop, self.y_start = map(int, data[3][2:].split('..'))

    def part1(self):
        global_max_height = 0
        for i in range(1000):
            for j in range(1000):
                max_height = 0
                hit = False
                x = 0
                y = 0
                x_velocity = i
                y_velosity = j
                while True:
                    x += x_velocity
                    y += y_velosity
                    x_velocity -= 1 if x_velocity else 0
                    y_velosity -= 1

                    if x > self.x_stop or y < self.y_stop:
                        break
                    
                    if self.x_start <= x <= self.x_stop and self.y_start >= y >= self.y_stop:
                        hit = True
                        break

                    if y > max_height:
                        max_height = y

                if hit and max_height > global_max_height:
                    global_max_height = max_height

        super().print_answer(1, global_max_height)

    def part2(self):
        count = 0

        for i in range(1000):
            for j in range(-1000, 1000):
                hit = False
                x = 0
                y = 0
                x_velocity = i
                y_velosity = j

                while True:
                    x += x_velocity
                    y += y_velosity
                    x_velocity -= 1 if x_velocity else 0
                    y_velosity -= 1

                    if x > self.x_stop or y < self.y_stop:
                        break
                    
                    if self.x_start <= x <= self.x_stop and self.y_start >= y >= self.y_stop:
                        count += 1
                        break

        super().print_answer(2, count)
