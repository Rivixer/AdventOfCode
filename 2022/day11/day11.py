import re
import math
from copy import deepcopy
from dataclasses import dataclass


@dataclass(slots=True)
class Monkey:
    backpack: list[int]
    counter: int
    operation: str
    divisor: int
    if_true: int
    if_false: int


with open('input.txt', 'r') as f:
    raw_monkeys = f.read().split('\n\n')

monkeys: list[Monkey] = list()

for raw_monkey in raw_monkeys:
    lines = raw_monkey.split('\n')
    items = list(map(int, re.findall(r'\d+', lines[1])))
    operation = lines[2].split('Operation: new =')[1]
    divisor = int(re.findall(r'\d+', lines[3])[0])
    if_true = int(re.findall(r'\d+', lines[4])[0])
    if_false = int(re.findall(r'\d+', lines[5])[0])
    monkeys.append(Monkey(items, 0, operation, divisor, if_true, if_false))


def solve(monkeys: list[Monkey], part2: bool = False):
    lcm_divs = math.lcm(*map(lambda i: i.divisor, monkeys))
    for _ in range(10000 if part2 else 20):
        for monkey in monkeys:
            monkey.counter += len(monkey.backpack)

            for old in monkey.backpack.copy():
                new = int(eval(f'{monkey.operation}'))
                if monkey.operation.find('* old') >= 0:
                    new %= lcm_divs

                if not part2:
                    new //= 3

                monkey.backpack.remove(old)
                if new % monkey.divisor == 0:
                    monkeys[monkey.if_true].backpack.append(new)
                else:
                    monkeys[monkey.if_false].backpack.append(new)

    return math.prod(sorted(map(lambda i: i.counter, monkeys))[-2:])


print(solve(deepcopy(monkeys)))
print(solve(deepcopy(monkeys), part2=True))
