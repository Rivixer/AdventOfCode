import re

with open('input.txt') as f:
    lines = list(map(str.strip, f.readlines()))

result = set()
for line in lines:
    x, y, xx, yy = map(int, re.findall(r'\d+', line))
    dx = abs(xx - x) + abs(yy - y) - abs(2000000 - y)
    if dx >= 0:
        result.update(range(x-dx, x+dx+1))

print(len(result)-1)
