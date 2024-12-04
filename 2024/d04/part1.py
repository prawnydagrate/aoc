from getinput import getlines

type Direction = tuple[int, int]
DIRECTIONS: dict[str, Direction] = {
"N": (-1, 0),
"NE": (-1, 1),
"E": (0, 1),
"SE": (1, 1),
"S": (1, 0),
"SW": (1, -1),
"W": (0, -1),
"NW": (-1, -1),
}

inp_lines = list(getlines())

def get_new_pos(line: int, col: int, direction: Direction) -> tuple[int, int]:
    return (line + direction[0], col + direction[1])

def check_direction(direction: Direction, search: str, lines: list[str], line: int, col: int) -> bool:
    if line not in range(0, len(lines)):
        return False
    l = lines[line]
    if col not in range(0, len(l)):
        return False
    if l[col] == search[0]:
        if len(search) == 1:
            return True
        return check_direction(direction, search[1:], lines, *get_new_pos(line, col, direction))
    return False

def count_occurrences(of: str, lines: list[str]) -> int:
    occs = 0
    for l, line in enumerate(lines):
        for c, char in enumerate(line):
            if of.startswith(char):
                for direction in DIRECTIONS.values():
                    occs += check_direction(direction, of, lines, l, c)
    return occs

print(count_occurrences("XMAS", inp_lines))
