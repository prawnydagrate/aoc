def is_safe(line: str) -> bool:
    nums = [int(n) for n in line.strip().split()]
    sign = None
    l, r = 0, 1
    while r < len(nums):
        diff = nums[r] - nums[l]
        if abs(diff) not in range(1, 4):
            return False
        if sign is None:
            sign = diff < 0
        elif (diff < 0) != sign:
            return False
        l += 1
        r += 1
    return True


def solution() -> int:
    safe = 0
    with open("input.txt", "r") as f:
        for line in f:
            safe += is_safe(line)
    return safe


def test() -> int:
    safe = 0
    for line in """\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9\
""".splitlines():
        safe += is_safe(line)
    return safe


if __name__ == "__main__":
    print(solution())
