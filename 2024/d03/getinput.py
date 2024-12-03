INPUT_FILE = None  # comment this line to use input
TEST_INPUT_INDEX = 0  # change this index to select example

TEST_INPUTS = [
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
]

INPUT_FILE = "input.txt"


def getlines():
    if INPUT_FILE is None:
        for line in TEST_INPUTS[TEST_INPUT_INDEX].splitlines():
            yield line
    with open(INPUT_FILE) as f:
        for line in f:
            yield line


def getall() -> str:
    return "\n".join(getlines())
