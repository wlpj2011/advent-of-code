from typing import Tuple

def process_input(filename: str) -> list[str]:
    lines = []
    with open(filename, 'r') as file:
        for line in file.readlines():
            lines.append(line)
    return lines

def count_guard_squares(guard_map: list[str]) -> int:
    return 0

def locate_guard(guard_map: list[str]) -> Tuple[int, int, int]:
    for i in range(len(guard_map)):
        for j in range(len(guard_map[i])):
            if guard_map[i][j] == "^":
                new_guard_map_row = list(guard_map[i])
                new_guard_map_row[j] = "."
                guard_map[i] = str(new_guard_map_row)
                return (i, j, 0)
    return (0,0,0)
            
def move_guard(guard_map: list[str], guard_location: Tuple[int, int, int]) -> Tuple[int, int, int]:
    
    return guard_location

if __name__ == "__main__":
    guard_map = ["....#.....", ".........#", "..........", "..#.......", ".......#..", "..........", ".#..^.....", "........#.", "#.........", "......#..."]
    #guard_map = process_input('input-04.txt')

    total = count_guard_squares(guard_map)
    print(f"Total number of squares the guard visits is {total}.")
