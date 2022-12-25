with open('input.txt', 'r') as f:
    lines = list(map(str.strip, f.readlines()))


r = 0
for line in lines:
    for i, char in enumerate(reversed(line)):
        match char:
            case '0' | '1' | '2':
                w = int(char)
            case '-':
                w = -1
            case '=':
                w = -2
            case _:
                raise ValueError('invaild char ' + char)
        r += w * (5 ** i)

s = ''
while r > 0:
    rr = (r + 2) % 5 - 2
    match rr:
        case 0 | 1 | 2:
            s += str(rr)
        case -1:
            r += 5
            s += '-'
        case -2:
            r += 5
            s += '='
    r //= 5

print(s[::-1])
