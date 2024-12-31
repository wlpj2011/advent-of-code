import string
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


    T = FSM_State("true")
    F = FSM_State("false")
    Tm = FSM_State("tm")
    Tmu = FSM_State("tmu")
    Tmul = FSM_State("tmul")
    Tmul0 = FSM_State("tmul0")
    Tmul0d1 = FSM_State("tmul0d1")
    Tmul0d2 = FSM_State("tmul0d2")
    Tmul0d3 = FSM_State("tmul0d3")
    Tmul0dc = FSM_State("tmul0dc")
    Tmul0dcd1 = FSM_State("tmul0dcd1")
    Tmul0dcd2 = FSM_State("tmul0dcd2")
    Tmul0dcd3 = FSM_State("tmul0dcd3")
    Tmul0dcd0 = FSM_State("tmul0dcd0")
    Td = FSM_State("td")
    Tdo = FSM_State("tdo")
    Tdon = FSM_State("tdon")
    Tdona = FSM_State("tdona")
    Tdonat = FSM_State("tdonat")
    Tdonat0 = FSM_State("tdonat0")
    Tdonat00 = FSM_State("tdonat00")
    Fd = FSM_State("fd")
    Fdo = FSM_State("fdo")
    Fdo0 = FSM_State("fdo0")
    Fdo00 = FSM_State("fdo00")

    ascii_chars = list(string.printable)
    digit_chars = list(string.digits)
    ascii_not_digits = ascii_chars.copy()
    for digit in digit_chars:
        ascii_not_digits.remove(digit)
    ascii_not_digits.remove(",")
    
    ruleTTm = FSM_Rule(T, Tm, "m")
    ruleTTd = FSM_Rule(T, Td, "d")
    TT_chars = ascii_chars.copy()
    TT_chars.remove("m")
    TT_chars.remove("d")
    ruleTT = FSM_Rule(T, Tm, TT_chars)

    ruleTmTmu = FSM_Rule(Tm, Tmu, "u")
    TmT_chars = ascii_chars.copy()
    TmT_chars.remove("u")
    ruleTmT = FSM_Rule(Tm, T, TmT_chars)

    ruleTmuTmul = FSM_Rule(Tmu, Tmul, "l")
    TmuT_chars = ascii_chars.copy()
    TmuT_chars.remove("l")
    ruleTmuT = FSM_Rule(Tmu, T, TmuT_chars)

    ruleTmulTmul0 = FSM_Rule(Tmul, Tmul0, "(")
    TmulT_chars = ascii_chars.copy()
    TmulT_chars.remove("(")
    ruleTmulT = FSM_Rule(Tmul, T, TmulT_chars)

    ruleTmul0Tmul0d1 = FSM_Rule(Tmul0, Tmul0d1, digit_chars)
    ruleTmul0T = FSM_Rule(Tmul0, T, ascii_not_digits)
    
    ruleTmul0d1Tmul0d2 = FSM_Rule(Tmul0d1, Tmul0d2, digit_chars)
    ruleTmul0d1T = FSM_Rule(Tmul0d1, T, ascii_not_digits)
    ruleTmul0d1Tmul0dc = FSM_Rule(Tmul0d1, Tmul0dc, ",")
    
    ruleTmul0d2Tmul0d3 = FSM_Rule(Tmul0d2, Tmul0d3, digit_chars)
    ruleTmul0d2T = FSM_Rule(Tmul0d2, T, ascii_not_digits)
    ruleTmul0d2Tmul0dc = FSM_Rule(Tmul0d2, Tmul0dc, ",")

    ruleTmul0d3Tmul0dc = FSM_Rule(Tmul0d3, Tmul0dc, ",")
    Tmul0d3T_chars = ascii_chars.copy()
    Tmul0d3T_chars.remove(",")
    ruleTmul0d3T = FSM_Rule(Tmul0d3, T, Tmul0d3T_chars)


    
    #rules = [ruleb0, ruleb1, rule00, rule11, rule01, rule10]
    #states = [begin, end, state0, state1]

    #test_FSM = FiniteStateMachine(states, rules)
    input_str = "00"
    #test_FSM.add_input(input_str)

    #for _ in range(10):
    #    print(test_FSM.current_state)
    #    test_FSM.advance_FSM()
    


    
    #total_sum = process_memory(corrupted_memory)
    total_sum = 0
    print(f"Total of mul's is {total_sum}")
