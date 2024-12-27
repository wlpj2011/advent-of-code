def process_input(filename):
    lines = []
    with open(filename, 'r') as file:
        for line in file.readlines():
            lines.append(line)
    return lines

#for each instance of 'mul(' keep searching until you find a non-digit, if you find one, check to make sure it is ',', then continue searching for non-digits and make sure the next is ')'. Then continue at the next 'mul('

def process_memory(corrupted_memory):
    total_sum = 0
    max_digits = 3
    for line in corrupted_memory:
        i = 0
        while i < len(line):
            next = line.find('mul(', i)
            if next == -1:
                break
            i = next + 4
            count_first_digits = 0
            count_second_digits = 0
            first_num = ''
            second_num = ''
            cond_satisfied = False
            while count_first_digits <= 3:
                if line[i].isdigit():
                    first_num += line[i]
                    i += 1
                    count_first_digits += 1
                elif line[i] == ',':
                    i += 1
                    while count_second_digits <= 3:
                        if line[i].isdigit():
                            second_num += line[i]
                            i += 1
                            count_second_digits += 1
                        elif line[i] == ')':
                            cond_satisfied = True
                            break
                        else:
                            break
                    break
                else:
                    break
            if cond_satisfied:
                total_sum += int(first_num) * int(second_num)
            #print(first_num, second_num)
            #print(line[i:i+7])
            #print(i)
    return total_sum
    

if __name__ == "__main__":
    #corrupted_memory = ["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"]
    corrupted_memory = process_input('input-03-1.txt')

    total_sum = process_memory(corrupted_memory)
    print(f"Total of mul's is {total_sum}")
