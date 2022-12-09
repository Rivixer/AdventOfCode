from __future__ import annotations

with open('input.txt', 'r') as f:
    data = list(map(str.strip, f.readlines()))


class Pos:
    x: int
    y: int
    __head: Pos | None

    def __init__(self, x: int, y: int, head: Pos | None = None) -> None:
        self.x = x
        self.y = y
        self.__head = head

    def __eq__(self, obj: Pos) -> bool:
        return self.x == obj.x and self.y == obj.y

    def __hash__(self) -> int:
        return self.x * 1000 + self.y

    def __repr__(self) -> str:
        return f'{self.x} {self.y}'

    def _is_around(self, pos: Pos) -> bool:
        return abs(self.x - pos.x) < 2 and abs(self.y - pos.y) < 2

    def copy(self) -> Pos:
        return Pos(self.x, self.y, self._head.copy() if self.__head is not None else None)

    @property
    def _head(self) -> Pos:
        if self.__head is None:
            raise Exception('__head is None')
        return self.__head

    def move(self) -> None:
        if self._is_around(self._head):
            return

        x_shift = self._head.x - self.x
        y_shift = self._head.y - self.y

        if [abs(x_shift), abs(y_shift)] in [[1, 2], [2, 1]]:
            self.x += -1 if x_shift < 0 else 1
            self.y += -1 if y_shift < 0 else 1
        elif [abs(x_shift), abs(y_shift)] == [2, 2]:
            self.x += -1 if x_shift < 0 else 1
            self.y += -1 if y_shift < 0 else 1
        elif abs(x_shift) == 2:
            self.x += -1 if x_shift < 0 else 1
        elif abs(y_shift) == 2:
            self.y += -1 if y_shift < 0 else 1
        else:
            raise Exception('The knot broke')


def solve(head: Pos, tails: list[Pos]) -> int:
    history = {tails[-1].copy()}

    for d in data:
        for _ in range(int(d[2:])):
            match d[0]:
                case 'R':
                    head.x += 1
                case 'U':
                    head.y += 1
                case 'L':
                    head.x -= 1
                case 'D':
                    head.y -= 1

            for tail in tails:
                tail.move()
            history.add(tails[-1].copy())

    return len(history)


# part 1
head = Pos(0, 0)
tails = [Pos(0, 0, head)]
print(solve(head, tails))

# part 2
head = Pos(0, 0)
tails = [Pos(0, 0, head)]
for i in range(8):
    tails.append(Pos(0, 0, tails[-1]))
print(solve(head, tails))
