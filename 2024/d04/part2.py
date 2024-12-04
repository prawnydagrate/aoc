from os import walk

from getinput import getlines

type Direction = tuple[int, int]
X_DIRECTIONS: tuple[Direction, Direction] = (
    (1, 1),  # SE
    (1, -1),  # SW
)


def get_new_pos(line: int, col: int, direction: Direction) -> tuple[int, int]:
    return (line + direction[0], col + direction[1])


def check_direction(
    direction: Direction, search: str, lines: list[str], line: int, col: int
) -> bool:
    if line not in range(0, len(lines)):
        return False
    l = lines[line]
    if col not in range(0, len(l)):
        return False
    if l[col] == search[0]:
        if len(search) == 1:
            return True
        return check_direction(
            direction, search[1:], lines, *get_new_pos(line, col, direction)
        )
    return False


def count_x_occurrences(of: str, lines: list[str]) -> int:
    occs = 0
    for l, line in enumerate(lines):
        for c, char in enumerate(line):
            if of.startswith(char):
                if check_direction(
                    X_DIRECTIONS[0], of, lines, l, c
                ):
                    if check_direction(X_DIRECTIONS[1], of, lines, l, c + 2):
                        occs += 1
                    if check_direction(X_DIRECTIONS[1], of[::-1], lines, l, c + 2):
                        occs += 1
            elif of.endswith(char):
                if check_direction(
                    X_DIRECTIONS[0], of[::-1], lines, l, c
                ):
                    if check_direction(X_DIRECTIONS[1], of, lines, l, c + 2):
                        occs += 1
                    if check_direction(X_DIRECTIONS[1], of[::-1], lines, l, c + 2):
                        occs += 1
    return occs


inp_lines = list(getlines())
print(count_x_occurrences("MAS", inp_lines))
