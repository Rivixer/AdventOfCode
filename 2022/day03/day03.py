import re

with open('input.txt', 'r') as f:
    data = list(map(str.strip, f.readlines()))

# part 1
print(sum(map(lambda d: i-96 if (i := ord(re.findall(f'[{d[0:len(d)//2]}]', d[len(d)//2:])[0])) >= 97 else i-38, data)))
# part 2
print(sum(map(lambda k: i-96 if (i := ord(re.findall(f'[{"".join(re.findall(f"[{data[k]}]", data[k+1]))}]', data[k+2])[0])) >= 97 else i-38, range(0, len(data), 3))))