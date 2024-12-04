TEST_INPUTS = [
    """\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX\
""",
]

INPUT_FILE = "input.txt"

# INPUT_FILE = None  # comment this line to use input
TEST_INPUT_INDEX = 0  # change this index to select example


def getlines():
    if INPUT_FILE is None:
        yield from TEST_INPUTS[TEST_INPUT_INDEX].splitlines()
        return
    with open(INPUT_FILE) as f:
        yield from f


def getall() -> str:
    return "\n".join(getlines())
