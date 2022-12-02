# part 1
print(sum((b-a-1) % 3*3 + (b-1) % 3+1 for a, b in map(lambda i: map(ord, i.split()), open('input.txt', 'r'))))
# part 2
print(sum((b-1) % 3*3 + (a+(b+2) % 3) % 3+1 for a, b in map(lambda i: map(ord, i.split()), open('input.txt', 'r'))))