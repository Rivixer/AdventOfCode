import json
import math
from functools import cmp_to_key


with open('input.txt', 'r') as f:
    data = f.read().split('\n\n')

items = list(map(lambda i: list(map(json.loads, i.split('\n'))), data))


def compare(a: list | int, b: list | int) -> int:
    if isinstance(a, int) and isinstance(b, int):
        return a - b
    if isinstance(a, int) and isinstance(b, list):
        return compare([a], b)
    if isinstance(a, list) and isinstance(b, int):
        return compare(a, [b])
    if isinstance(a, list) and isinstance(b, list):
        for aa, bb in zip(a, b):
            x = compare(aa, bb)
            if x != 0:
                return x
        return len(a) - len(b)
    raise Exception('compare returned None')


# part 1
print(sum([i + 1 for i, item in enumerate(items) if compare(*item) <= 0]))

# part 2
div_packet = [[[2]], [[6]]]
items2 = list(map(lambda i: i[0], items)) + \
    list(map(lambda i: i[1], items)) + div_packet
items2.sort(key=cmp_to_key(compare))
print(math.prod([i+1 for i, item in enumerate(items2) if item in div_packet]))
