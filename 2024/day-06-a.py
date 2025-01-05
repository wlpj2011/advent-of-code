def process_input(filename: str) -> list[str]:
    lines = []
    with open(filename, 'r') as file:
        for line in file.readlines():
            lines.append(line)
    return lines

def count_guard_squares(guard_map: list[str]) -> int:

    return 0

if __name__ == "__main__":
    guard_map = ["....#.....", ".........#", "..........", "..#.......", ".......#..", "..........", ".#..^.....", "........#.", "#.........", "......#..."]
    #guard_map = process_input('input-04.txt')

    total = count_guard_squares(guard_map)
    print(f"Total number of squares the guard visits is {total}.")
