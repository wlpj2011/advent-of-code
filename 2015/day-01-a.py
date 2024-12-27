def process_input(filename):
    lines = []
    with open(filename, 'r') as file:
        for line in file.readlines():
            lines.append(line)
    return lines[0]

def count_floors(input_str):
    floors_up = input_str.count("(")
    return floors_up - (len(input_str) - floors_up)

if __name__ == "__main__":
    test_input_0 = "(())"
    test_input_3 = "(()(()("
    test_input_neg3 = "))())()"

    real_input = process_input('input-01-1.txt')
    floor = count_floors(real_input)
    print(f"Santa gets taken to the {floor}th floor.")
