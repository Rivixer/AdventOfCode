import re

# part 1
print(len(list(filter(lambda i: i[0]<=i[2] and i[1]>=i[3] or i[2]<=i[0] and i[3]>=i[1], map(lambda d: list(map(int, re.findall(r'\d+', d))), open('input.txt', 'r'))))))
# part 2
print(len(list(filter(lambda i: len(set(range(i[0], i[1]+1)).intersection(set(range(i[2], i[3]+1)))), map(lambda d: list(map(int, re.findall(r'\d+', d))), open('input.txt', 'r'))))))
