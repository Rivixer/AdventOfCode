with open('input.txt', 'r') as f:
    data = list(map(str.strip, f.readlines()))

# part 1
def is_visible_up(i: int, j: int) -> bool:
    return not any(data[ii][j] >= data[i][j] for ii in range(i))


def is_visible_down(i: int, j: int) -> bool:
    return not any(data[ii][j] >= data[i][j] for ii in range(len(data) - 1, i, -1))


def is_visible_left(i: int, j: int) -> bool:
    return not any(data[i][jj] >= data[i][j] for jj in range(j))


def is_visible_right(i: int, j: int) -> bool:
    return not any(data[i][jj] >= data[i][j] for jj in range(len(data) - 1, j, -1))


print(len(data) * 4 - 4 + len(list(filter(any, [[
    is_visible_up(i, j),
    is_visible_down(i, j),
    is_visible_left(i, j),
    is_visible_right(i, j),
] for j in range(1, len(data) - 1) 
    for i in range(1, len(data) - 1)]))))


# part 2
import math

def trees_on_up(i: int, j: int) -> int:
    k = 0
    for ii in range(i-1, -1, -1):
        k += 1
        if data[ii][j] >= data[i][j]:
            break
    return k
    
def trees_on_down(i: int, j: int) -> int:
    k = 0
    for ii in range(i+1, len(data)):
        k += 1
        if data[ii][j] >= data[i][j]:
            break
    return k

def trees_on_left(i: int, j: int) -> int:
    k = 0
    for jj in range(j-1, -1, -1):
        k += 1
        if data[i][jj] >= data[i][j]:
            break
    return k

def trees_on_right(i: int, j: int) -> int:
    k = 0
    for jj in range(j+1, len(data)):
        k += 1
        if data[i][jj] >= data[i][j]:
            break
    return k


print(max([math.prod([
    trees_on_left(i, j),
    trees_on_right(i, j),
    trees_on_down(i, j),
    trees_on_up(i, j)
]) for j in range(1, len(data)-1)
    for i in range(1, len(data)-1)]))