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

def verify_update(update: list[int], rules: list[Tuple[int, int]]) -> bool:
    for page in update:
        for rule in rules:
            if page == rule[0]:
                if rule[1] in update:
                    if update.index(rule[1]) < update.index(page):
                        return False
            if page == rule[1]:
                if rule[0] in update:
                    if update.index(rule[0]) > update.index(page):
                        return False
    return True

def remove_valid_updates(updates: list[list[int]], rules: list[Tuple[int, int]]) -> list[list[int]]:
    invalid_updates = []
    for update in updates:
        if not verify_update(update, rules):
            invalid_updates.append(update)
    return invalid_updates

if __name__ == "__main__":
    rules = [(47, 53), (97, 13), (97, 61), (97, 47), (75, 29), (61, 13), (75, 53), (29, 13), (97, 29), (53, 29), (61, 53), (97, 53), (61, 29), (47, 13), (75, 47), (97, 75), (47, 61), (75, 61), (47, 29), (75, 13), (53, 13)]
    updates = [[75, 47, 61, 53, 29], [97, 61, 53, 29, 13], [75, 29, 13], [75, 97, 47, 61, 53], [61, 13, 29], [97, 13, 75, 29, 47]]
    #rules, updates = process_input('input-05.txt')

    invalid_updates = remove_valid_updates(updates, rules)
    print(f"Total sum of the middle of correct updates is {invalid_updates}")
