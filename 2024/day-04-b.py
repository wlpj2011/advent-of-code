def process_input(filename: str) -> list[str]:
    lines = []
    with open(filename, 'r') as file:
        for line in file.readlines():
            lines.append(line)
    return lines

def count_crossword(crossword: list[str]) -> int:
    total_count = 0
    for i in range(len(crossword)):
        for j in range(len(crossword[0])):
            if crossword[i][j] == "A":
                if i > 0 and j > 0 and i + 1 < len(crossword) and j + 1 < len(crossword):
                    if crossword[i-1][j-1] == "M" and crossword[i-1][j+1] == "S" and crossword[i+1][j-1] == "M" and crossword[i+1][j+1] == "S":
                        total_count += 1
                    elif crossword[i-1][j-1] == "M" and crossword[i-1][j+1] == "M" and crossword[i+1][j-1] == "S" and crossword[i+1][j+1] == "S":
                        total_count += 1
                    elif crossword[i-1][j-1] == "S" and crossword[i-1][j+1] == "M" and crossword[i+1][j-1] == "S" and crossword[i+1][j+1] == "M":
                        total_count += 1
                    elif crossword[i-1][j-1] == "S" and crossword[i-1][j+1] == "S" and crossword[i+1][j-1] == "M" and crossword[i+1][j+1] == "M":
                        total_count += 1
    return total_count

if __name__ == "__main__":
    #crossword = ["MMMSXXMASM", "MSAMXMSMSA", "AMXSXMAAMM", "MSAMASMSMX", "XMASAMXAMM", "XXAMMXXAMA", "SMSMSASXSS", "SAXAMASAAA", "MAMMMXMMMM", "MXMXAXMASX"]
    crossword = process_input('input-04.txt')

    total = count_crossword(crossword)
    print(f"Total number of XMAS's is {total}")
