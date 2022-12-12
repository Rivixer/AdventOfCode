from priority_queue import PriorityQueue

with open('input.txt', 'r') as f:
    lines = list(map(str.strip, f.readlines()))

grid = [list(line) for line in lines]

height = len(grid)
width = len(grid[0])


def get_prio(y, x) -> int:
    if grid[y][x] == 'E':
        return ord('z') & 0b11111
    return ord(grid[y][x]) & 0b11111


def solve(queue: PriorityQueue):
    history: list[tuple[int, int]] = []

    while len(queue) > 0:
        prio, (y, x) = queue.get()

        if (y, x) in history:
            continue
        history.append((y, x))

        if grid[y][x] == 'E':
            print(prio - 1)
            break

        for yy, xx in ((0, 1), (1, 0), (-1, 0), (0, -1)):
            yyy = y + yy
            xxx = x + xx
            if yyy < 0 or xxx < 0 or yyy >= height or xxx >= width:
                continue
            if get_prio(yyy, xxx) - 1 > get_prio(y, x):
                continue
            queue.add(prio+1, (yyy, xxx))


queue1 = PriorityQueue()
queue2 = PriorityQueue()

for y in range(height):
    for x in range(width):
        if grid[y][x] == 'S':
            queue1.add(1, (y, x))
        if get_prio(y, x) == 1:
            queue2.add(1, (y, x))

solve(queue1)
solve(queue2)
