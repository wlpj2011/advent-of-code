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
            if crossword[i][j] == "X":
                if i >= 3 and j >= 3:
                    if (crossword[i-1][j-1] == "M") and (crossword[i-2][j-2] == "A") and (crossword[i-3][j-3] == "S"):
                        total_count+=1
                if i >= 3:
                    if (crossword[i-1][j] == "M") and (crossword[i-2][j] == "A") and (crossword[i-3][j] == "S"):
                        total_count+=1
                if j >= 3:
                    if (crossword[i][j-1] == "M") and (crossword[i][j-2] == "A") and (crossword[i][j-3] == "S"):
                        total_count+=1
                if i+3 < len(crossword) and j>=3:
                    if (crossword[i+1][j-1] == "M") and (crossword[i+2][j-2] == "A") and (crossword[i+3][j-3] == "S"):
                        total_count+=1
                if i+3 < len(crossword):
                    if (crossword[i+1][j] == "M") and (crossword[i+2][j] == "A") and (crossword[i+3][j] == "S"):
                        total_count+=1
                if j+3 < len(crossword) and i>=3:
                    if (crossword[i-1][j+1] == "M") and (crossword[i-2][j+2] == "A") and (crossword[i-3][j+3] == "S"):
                        total_count+=1
                if j+3 < len(crossword):
                    if (crossword[i][j+1] == "M") and (crossword[i][j+2] == "A") and (crossword[i][j+3] == "S"):
                        total_count+=1
                if i+3 < len(crossword) and j+3 < len(crossword):
                    if (crossword[i+1][j+1] == "M") and (crossword[i+2][j+2] == "A") and (crossword[i+3][j+3] == "S"):
                        total_count+=1

    return total_count

if __name__ == "__main__":
    #crossword = ["MMMSXXMASM", "MSAMXMSMSA", "AMXSXMAAMM", "MSAMASMSMX", "XMASAMXAMM", "XXAMMXXAMA", "SMSMSASXSS", "SAXAMASAAA", "MAMMMXMMMM", "MXMXAXMASX"]
    crossword = process_input('input-04.txt')

    total = count_crossword(crossword)
    print(f"Total number of XMAS's is {total}")
