from typing import Tuple

def process_input(filename: str) -> Tuple[list[Tuple[int, int]], list[list[int]]]:
    rules = []
    updates = []
    with open(filename, 'r') as file:
        for line in file.readlines():
            if "|" in line:
                rule = line.split("|")
                rules.append((int(rule[0]), int(rule[1])))
            elif "," in line:
                update = line.split(",")
                int_update = []
                for num in update:
                    int_update.append(int(num))
                updates.append(int_update)
    return rules, updates


if __name__ == "__main__":
    rules = [(47, 53), (97, 13), (97, 61), (97, 47), (75, 29), (61, 13), (75, 53), (29, 13), (97, 29), (53, 29), (61, 53), (97, 53), (61, 29), (47, 13), (75, 47), (97, 75), (47, 61), (75, 61), (47, 29), (75, 13), (53, 13)]
    updates = [[75, 47, 61, 53, 29], [97, 61, 53, 29, 13], [75, 29, 13], [75, 97, 47, 61, 53], [61, 13, 29], [97, 13, 75, 29, 47]]
    #rules, updates = process_input('input-05.txt')

    #total = count_crossword(crossword)
    print(len(rules), len(updates))
    total = 0
    print(f"Total sum of the middle of correct updates is {total}")
