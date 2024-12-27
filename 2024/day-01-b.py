from collections import Counter
from typing import Tuple

def process_input(filename: str) -> Tuple[list[int], list[int]]:
    list1 = []
    list2 = []
    with open(filename, 'r') as file:
        for line in file.readlines():
            nums = line.split('   ')
            list1.append(int(nums[0]))
            list2.append(int(nums[1][:-1]))
    return list1, list2

def compare_lists(list1: list[int], list2: list[int]) -> int:
    c2 = Counter(list2)

    #print(list1)
    #print(list2)
    total_similarity = 0
    for i in range(min(len(list1),len(list2))):
        #print(abs(list1[i] - list2[i]))
        total_similarity += list1[i] * c2[list1[i]]
    return total_similarity

if __name__ == "__main__":
    #list1 = [3, 4, 2, 1, 3, 3]
    #list2 = [4, 3, 5, 3, 9, 3]
    list1, list2 = process_input('input-01-1.txt')
    distance = compare_lists(list1, list2)
    print(f"Similarity between the lists in {distance}")
