import re
from dataclasses import dataclass


@dataclass(slots=True)
class Scanner:
    x: int
    y: int
    dist: int


if __name__ == '__main__':

    with open('input.txt') as f:
        lines = list(map(str.strip, f.readlines()))

    scanners: list[Scanner] = list()
    for line in lines:
        x, y, xx, yy = map(int, re.findall(r'-*\d+', line))
        scanners.append(Scanner(x, y, abs(xx-x) + abs(yy-y)))

    # part 1
    result = set()
    for scanner in scanners:
        if (dx := scanner.dist - abs(2_000_000 - scanner.y)) >= 0:
            result.update(range(scanner.x-dx, scanner.x+dx+1))
    print(len(result) - 1)

    # part 2
    for i in range(4_000_000):
        s: list[tuple[int, int]] = list()
        for scanner in scanners:
            if (dx := scanner.dist - abs(i - scanner.y)) >= 0:
                s.append((scanner.x-dx, scanner.x+dx))
        s.sort()
        dxl, dxr = s[0]
        for ss in s[1:]:
            dxl2, dxr2 = ss
            if dxl2 > dxr:
                print((dxl2 - 1) * 4_000_000 + i)
                quit()
            dxr = max(dxr, dxr2)
