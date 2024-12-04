from getinput import getlines

type Direction = tuple[int, int]
DIRECTIONS: list[Direction] = [
    (-1, 0),  # N
    (-1, 1),  # NE
    (0, 1),  # E
    (1, 1),  # SE
    (1, 0),  # S
    (1, -1),  # SW
    (0, -1),  # W
    (-1, -1),  # NW
]


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


def count_occurrences(of: str, lines: list[str]) -> int:
    return sum(
        check_direction(direction, of, lines, l, c)
        for l, line in enumerate(lines)
        for c, char in enumerate(line)
        if of.startswith(char)
        for direction in DIRECTIONS
    )


inp_lines = list(getlines())
print(count_occurrences("XMAS", inp_lines))
