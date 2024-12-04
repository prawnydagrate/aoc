TEST_INPUTS = [
]

INPUT_FILE = "input.txt"

INPUT_FILE = None  # comment this line to use input
TEST_INPUT_INDEX = 1  # change this index to select example


def getlines():
    if INPUT_FILE is None:
        yield from TEST_INPUTS[TEST_INPUT_INDEX].splitlines()
    with open(INPUT_FILE) as f:
        yield from f


def getall() -> str:
    return "\n".join(getlines())
