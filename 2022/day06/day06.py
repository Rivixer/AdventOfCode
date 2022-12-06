data = open('input.txt', 'r').read().strip()

# part 1
print([i + 4 for i in range(len(data) - 4) if len(data[i:i+4]) == len(set(data[i:i+4]))][0])

# part 2
print([i + 14 for i in range(len(data) - 14) if len(data[i:i+14]) == len(set(data[i:i+14]))][0])


