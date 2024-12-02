ll: list[int] = []
lr: list[int] = []

with open("input.txt") as inputf:
    tuple(
        tuple(
            l.append(n)
            for l, n in zip((ll, lr), (int(n.strip()) for n in line.split("   ")))
        )
        for line in inputf
    )

score = sum(n * lr.count(n) for n in ll)

print(score)
