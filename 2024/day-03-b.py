from FiniteStateMachine import *

def process_input(filename: str) -> list[str]:
    lines = []
    with open(filename, 'r') as file:
        for line in file.readlines():
            lines.append(line)
    return lines

#for each instance of 'mul(' keep searching until you find a non-digit, if you find one, check to make sure it is ',', then continue searching for non-digits and make sure the next is ')'. Then continue at the next 'mul('



    

if __name__ == "__main__":
    corrupted_memory = ["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"]
    #corrupted_memory = process_input('input-03.txt')


    begin = FSM_State("begin")
    end = FSM_State("end")
    state0 = FSM_State("0")
    state1 = FSM_State("1")
    ruleb0 = FSM_Rule(begin, state0, "0")
    ruleb1 = FSM_Rule(begin, state1, "1")
    rule00 = FSM_Rule(state0, end, "0")
    rule11 = FSM_Rule(state1, end, "1")
    rule01 = FSM_Rule(state0, state1, "1")
    rule10 = FSM_Rule(state1, state0, "0")
    rules = [ruleb0, ruleb1, rule00, rule11, rule01, rule10]
    states = [begin, end, state0, state1]

    test_FSM = FiniteStateMachine(states, rules)
    input_str = "00"
    test_FSM.add_input(input_str)

    for _ in range(10):
        print(test_FSM.current_state)
        test_FSM.advance_FSM()
    


    
    #total_sum = process_memory(corrupted_memory)
    total_sum = 0
    print(f"Total of mul's is {total_sum}")
