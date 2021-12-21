from advent_of_code import AdventOfCode


class Day21(AdventOfCode):
    def __init__(self, test = False, part1 = True, part2 = True):
        self.test = test
        super().__init__(21, test)
        self.start_pos_1 = 4 if test else 7
        self.start_pos_2 = 8 if test else 5

        if part1:
            self.part1()
        if part2:
            self.part2()

    def get_sum_of_roll_dice(self, dice_rolled: int):
        result = 0
        for i in range(dice_rolled, dice_rolled + 3):
            if i > 100:
                result += i % 100
            else:
                result += i
        return result

    def part1(self):
        player = 1
        dice_rolled = 1
        pos_1 = self.start_pos_1
        pos_2 = self.start_pos_2
        score_1 = 0
        score_2 = 0

        while True:
            
            sum_of_roll_dice = self.get_sum_of_roll_dice(dice_rolled)
            dice_rolled += 3
            if player == 1:
                pos_1 += sum_of_roll_dice
                pos_1 = ((pos_1 - 1) % 10) + 1
                score_1 += pos_1
            else:
                pos_2 += sum_of_roll_dice
                pos_2 = ((pos_2 - 1) % 10) + 1
                score_2 += pos_2

            if score_1 >= 1000 or score_2 >= 1000:
                break
 
            player = (player % 2) + 1

        super().print_answer(1, min([score_1, score_2])*(dice_rolled - 1))

    def part2(self):
        pass