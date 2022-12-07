with open('input.txt', 'r') as f:
    data = list(map(str.strip, f.readlines()))

path: list[str] = list()
result: dict[str, int] = dict()

for d in data:
    if d == '$ cd ..':
        path = path[:-1]
    elif d.startswith('$ cd'):
        path.append(d[5:])

    try:
        mem = int(d.split(' ')[0])
    except ValueError:
        continue

    try:
        result['/'.join(path)] += mem
    except KeyError:
        result['/'.join(path)] = mem

    for i in range(1, len(path)):
        try:
            result['/'.join(path[:i])] += mem
        except KeyError:
            result['/'.join(path[:i])] = mem

# part 1
print(sum([v for v in result.values() if v < 100_000]))
# part 2
print(min([v for v in result.values() if 70_000_000 - result['/'] + v >= 30_000_000]))