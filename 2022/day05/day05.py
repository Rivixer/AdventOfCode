inputs, instrs = open('input.txt', 'r').read().split('\n\n')
stack1 = list(map(list, [[i for i in inp if i != ' '] for i, inp in enumerate(zip(*reversed(inputs.splitlines()[:-1]))) if i % 4 == 1]))

from copy import deepcopy
stack2 = deepcopy(stack1)

import re
for instr in instrs.split('\n'):
    for _ in range((k := list(map(int, re.findall(r'\d+', instr))))[0]):
        stack1[k[2]-1].append(stack1[k[1]-1].pop())
    stack2[k[2]-1].extend(stack2[k[1]-1][-k[0]:])
    stack2[k[1]-1] = stack2[k[1]-1][:-k[0]]

print(''.join((map(lambda i: i[-1], stack1))))
print(''.join((map(lambda i: i[-1], stack2))))