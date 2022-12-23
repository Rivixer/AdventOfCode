data = [(i, int(l)) for i, l in enumerate(open('input.txt', 'r').readlines())]


def mix(data: list[tuple[int, int]]) -> list[tuple[int, int]]:
    for i in range(len(data)):
        for pos, d in enumerate(data):
            if d[0] == i:
                data.remove(d)
                new_pos = (pos+d[1]) % len(data)
                data.insert(new_pos, d)
                break
    return data


def answer(data: list[tuple[int, int]]) -> int | None:
    for pos, d in enumerate(data):
        if d[1] == 0:
            return sum(data[(i*1000+pos) % len(data)][1] for i in range(1, 4))


data1 = mix(data.copy())
print(answer(data1))

data2 = list(map(lambda i: (i[0], i[1]*811589153), data.copy()))
for _ in range(10):
    mix(data2)
print(answer(data2))
