with open('input.txt', 'r') as f:
    lines = list(map(str.strip, f.readlines()))


# part 1
class Data:
    x: int = 1
    cycle: int = 1
    signal_strengths: list[int] = list()


def new_cycle():
    data.cycle += 1
    if data.cycle in [20] + list(range(60, 240, 40)):
        data.signal_strengths.append(data.x * data.cycle)


data = Data()
for line in lines:
    new_cycle()
    try:
        data.x += int(line.split(' ')[1])
    except (ValueError, IndexError):
        continue
    new_cycle()

print(sum(data.signal_strengths))


# part 2
class Sprite:
    start: int = 0
    stop: int = 2

    def move(self, count: int) -> None:
        self.start += count
        self.stop += count


class Row:
    row: list[str]
    sprite: Sprite

    def __init__(self, sprite: Sprite) -> None:
        self.sprite = sprite
        self.row = list()

    def draw(self) -> None:
        print(' '.join(self.row))

    def new_cycle(self) -> None:
        in_spirit = self.sprite.start <= len(self.row) <= self.sprite.stop
        self.row.append('#' if in_spirit else '.')


sprite = Sprite()
rows: list[Row] = [Row(sprite) for _ in range(6)]

cycle = 0
for line in lines:
    rows[cycle//40].new_cycle()
    cycle += 1

    try:
        i = int(line.split(' ')[1])
    except (ValueError, IndexError):
        continue

    rows[cycle//40].new_cycle()
    rows[cycle//40].sprite.move(i)
    cycle += 1

for row in rows:
    row.draw()
