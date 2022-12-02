# part 1
print(max([sum(map(int, i.split('\n')))for i in open('input.txt', 'r').read().split('\n\n')]))
# part 2
print(sum(sorted([sum(map(int, i.split('\n'))) for i in open('input.txt', 'r').read().split('\n\n')], reverse=True)[:3]))
