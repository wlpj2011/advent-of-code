from typing import Tuple

def process_input(filename: str) -> list[str]:
    lines = []
    with open(filename, 'r') as file:
        for line in file.readlines():
            lines.append(line)
    return lines

def count_guard_squares(guard_map: list[str]) -> int:
    guard_location = locate_guard(guard_map)
    while(guard_location[2] != -1):
        guard_location = move_guard(guard_map, guard_location)
    count_visited = 0
    for row in guard_map:
        count_visited += list(row).count("X")
    return count_visited

def locate_guard(guard_map: list[str]) -> Tuple[int, int, int]:
    for i in range(len(guard_map)):
        for j in range(len(guard_map[i])):
            if guard_map[i][j] == "^":
                new_guard_map_row = list(guard_map[i])
                new_guard_map_row[j] = "."
                guard_map[i] = ''.join(new_guard_map_row)
                return (i, j, 0)
    return (0,0,0)
            
def move_guard(guard_map: list[str], guard_location: Tuple[int, int, int]) -> Tuple[int, int, int]:
    if guard_location[2] == 0:
        next_location = (guard_location[0] - 1, guard_location[1])
    elif guard_location[2] == 1:
        next_location = (guard_location[0], guard_location[1] + 1)
    elif guard_location[2] == 2:
        next_location = (guard_location[0] + 1, guard_location[1])
    elif guard_location[2] == 3:
        next_location = (guard_location[0], guard_location[1] - 1)

    if next_location[0] >= len(guard_map) or next_location[1] >= len(guard_map) or next_location[0] < 0 or next_location[1] < 0:
        # If you go outside the map, return with direction -1
        new_guard_map_row = list(guard_map[guard_location[0]])
        new_guard_map_row[guard_location[1]] = "X"
        guard_map[guard_location[0]] = ''.join(new_guard_map_row)
        return (next_location[0], next_location[1], -1)
    elif guard_map[next_location[0]][next_location[1]] == "#":
        # If you run into an obstacle, just rotate
        return (guard_location[0], guard_location[1], (guard_location[2] + 1) % 4)
    else:
        new_guard_map_row = list(guard_map[guard_location[0]])
        new_guard_map_row[guard_location[1]] = "X"
        guard_map[guard_location[0]] = ''.join(new_guard_map_row)
        return (next_location[0], next_location[1], guard_location[2])


if __name__ == "__main__":
    #guard_map = ["....#.....", ".........#", "..........", "..#.......", ".......#..", "..........", ".#..^.....", "........#.", "#.........", "......#..."]
    guard_map = process_input('input-06.txt')

    total = count_guard_squares(guard_map)
    print(f"Total number of squares the guard visits is {total}.")
