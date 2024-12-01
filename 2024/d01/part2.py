ll: list[int] = []
lr: list[int] = []

with open('input.txt') as inputf:
    for line in inputf:
        tuple(l.append(n) for l, n in zip((ll, lr), (int(n.strip()) for n in line.split('   '))))

score = 0

for n in ll:
    score += n * lr.count(n)

print(score)
