def process_input(filename: str) -> list[str]:
    lines = []
    with open(filename, 'r') as file:
        for line in file.readlines():
            lines.append(line)
    return lines

def count_crossword(crossword: list[str]) -> int:

    return 0

if __name__ == "__main__":
    crossword = ["MMMSXXMASM", "MSAMXMSMSA", "AMXSXMAAMM", "MSAMASMSMX", "XMASAMXAMM", "XXAMMXXAMA", "SMSMSASXSS", "SAXAMASAAA", "MAMMMXMMMM", "MXMXAXMASX"]
    #crossword = process_input('input-04.txt')

    total = count_crossword(crossword)
    print(f"Total number of XMAS's is {total}")
