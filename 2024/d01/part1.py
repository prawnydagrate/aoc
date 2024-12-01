l1: list[int] = []
l2: list[int] = []

with open('input.txt') as inputf:
    for line in inputf:
        nums = (int(n.strip()) for n in line.split('   '))
        l1.append(next(nums))
        l2.append(next(nums))

def getmin(l: list[int]) -> int:
    minidx = 0
    for i in range(len(l)):
        if l[i] < l[minidx]:
            minidx = i
    n = l.pop(minidx)
    return n

tot = 0

for _ in range(len(l1)):
    tot += abs(getmin(l1) - getmin(l2))

print(tot)
