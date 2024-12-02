def check(nums: list[int], *, rm=False) -> bool:
    safe = True
    sign = None
    l, r = 0, 1
    while r < len(nums):
        diff = nums[r] - nums[l]
        if abs(diff) not in range(1, 4):
            safe = False
            break
        if sign is None:
            sign = diff < 0
        elif (diff < 0) != sign:
            safe = False
            break
        l += 1
        r += 1
    if not safe and not rm:
        for i in range(len(nums)):
            # i'm pretty sure it's supposed to be '[i+1:]', formatter bug ig
            if check(nums[:i] + nums[i + 1 :], rm=True):
                return True
    return safe


def is_safe(line: str) -> bool:
    nums = [int(n) for n in line.strip().split()]
    return check(nums)


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
