import string
from FiniteStateMachine import *

def process_input(filename: str) -> str:
    lines = ''
    with open(filename, 'r') as file:
        for line in file.readlines():
            lines += line
    return lines

#for each instance of 'mul(' keep searching until you find a non-digit, if you find one, check to make sure it is ',', then continue searching for non-digits and make sure the next is ')'. Then continue at the next 'mul('


def process_memory(corrupted_memory: str) -> int:
    states = []
    
    T = FSM_State("begin")
    states.append(T)
    F = FSM_State("false")
    states.append(F)
    Tm = FSM_State("tm")
    states.append(Tm)
    Tmu = FSM_State("tmu")
    states.append(Tmu)
    Tmul = FSM_State("tmul")
    states.append(Tmul)
    Tmul0 = FSM_State("tmul0")
    states.append(Tmul0)
    Tmul0d1 = FSM_State("tmul0d1")
    states.append(Tmul0d1)
    Tmul0d2 = FSM_State("tmul0d2")
    states.append(Tmul0d2)
    Tmul0d3 = FSM_State("tmul0d3")
    states.append(Tmul0d3)
    Tmul0dc = FSM_State("tmul0dc")
    states.append(Tmul0dc)
    Tmul0dcd1 = FSM_State("tmul0dcd1")
    states.append(Tmul0dcd1)
    Tmul0dcd2 = FSM_State("tmul0dcd2")
    states.append(Tmul0dcd2)
    Tmul0dcd3 = FSM_State("tmul0dcd3")
    states.append(Tmul0dcd3)
    Tmul0dcd0 = FSM_State("tmul0dcd0")
    states.append(Tmul0dcd0)
    Td = FSM_State("td")
    states.append(Td)
    Tdo = FSM_State("tdo")
    states.append(Tdo)
    Tdon = FSM_State("tdon")
    states.append(Tdon)
    Tdona = FSM_State("tdona")
    states.append(Tdona)
    Tdonat = FSM_State("tdonat")
    states.append(Tdonat)
    Tdonat0 = FSM_State("tdonat0")
    states.append(Tdonat0)
    Fd = FSM_State("fd")
    states.append(Fd)
    Fdo = FSM_State("fdo")
    states.append(Fdo)
    Fdo0 = FSM_State("fdo0")
    states.append(Fdo0)


    ascii_chars = list(string.printable)
    digit_chars = list(string.digits)
    ascii_not_digits = ascii_chars.copy()
    for digit in digit_chars:
        ascii_not_digits.remove(digit)
    ascii_not_digits.remove(",")
    ascii_not_digits.remove(")")


    rules = []
    
    ruleTTm = FSM_Rule(T, Tm, "m")
    rules.append(ruleTTm)
    ruleTTd = FSM_Rule(T, Td, "d")
    rules.append(ruleTTd)
    TT_chars = ascii_chars.copy()
    TT_chars.remove("m")
    TT_chars.remove("d")
    ruleTT = FSM_Rule(T, T, TT_chars)
    rules.append(ruleTT)

    ruleTmTmu = FSM_Rule(Tm, Tmu, "u")
    rules.append(ruleTmTmu)
    TmT_chars = ascii_chars.copy()
    TmT_chars.remove("u")
    ruleTmT = FSM_Rule(Tm, T, TmT_chars)
    rules.append(ruleTmT)

    ruleTmuTmul = FSM_Rule(Tmu, Tmul, "l")
    rules.append(ruleTmuTmul)
    TmuT_chars = ascii_chars.copy()
    TmuT_chars.remove("l")
    ruleTmuT = FSM_Rule(Tmu, T, TmuT_chars)
    rules.append(ruleTmuT)

    ruleTmulTmul0 = FSM_Rule(Tmul, Tmul0, "(")
    rules.append(ruleTmulTmul0)
    TmulT_chars = ascii_chars.copy()
    TmulT_chars.remove("(")
    ruleTmulT = FSM_Rule(Tmul, T, TmulT_chars)
    rules.append(ruleTmulT)

    ruleTmul0Tmul0d1 = FSM_Rule(Tmul0, Tmul0d1, digit_chars)
    rules.append(ruleTmul0Tmul0d1)
    ruleTmul0T = FSM_Rule(Tmul0, T, ascii_not_digits + [","])
    rules.append(ruleTmul0T)
    
    ruleTmul0d1Tmul0d2 = FSM_Rule(Tmul0d1, Tmul0d2, digit_chars)
    rules.append(ruleTmul0d1Tmul0d2)
    ruleTmul0d1T = FSM_Rule(Tmul0d1, T, ascii_not_digits + [")"])
    rules.append(ruleTmul0d1T)
    ruleTmul0d1Tmul0dc = FSM_Rule(Tmul0d1, Tmul0dc, ",")
    rules.append(ruleTmul0d1Tmul0dc)
    
    ruleTmul0d2Tmul0d3 = FSM_Rule(Tmul0d2, Tmul0d3, digit_chars)
    rules.append(ruleTmul0d2Tmul0d3)
    ruleTmul0d2T = FSM_Rule(Tmul0d2, T, ascii_not_digits + [")"])
    rules.append(ruleTmul0d2T)
    ruleTmul0d2Tmul0dc = FSM_Rule(Tmul0d2, Tmul0dc, ",")
    rules.append(ruleTmul0d2Tmul0dc)

    ruleTmul0d3Tmul0dc = FSM_Rule(Tmul0d3, Tmul0dc, ",")
    rules.append(ruleTmul0d3Tmul0dc)
    Tmul0d3T_chars = ascii_chars.copy()
    Tmul0d3T_chars.remove(",")
    ruleTmul0d3T = FSM_Rule(Tmul0d3, T, Tmul0d3T_chars)
    rules.append(ruleTmul0d3T)

    ruleTmul0dcTmul0dcd1 = FSM_Rule(Tmul0dc, Tmul0dcd1, digit_chars)
    rules.append(ruleTmul0dcTmul0dcd1)
    ruleTmul0dcT = FSM_Rule(Tmul0dc, T, ascii_not_digits + [","])
    rules.append(ruleTmul0dcT)

    ruleTmul0dcd1Tmul0dcd2 = FSM_Rule(Tmul0dcd1, Tmul0dcd2, digit_chars)
    rules.append(ruleTmul0dcd1Tmul0dcd2)
    ruleTmul0dcd1T = FSM_Rule(Tmul0dcd1, T, ascii_not_digits + [","])
    rules.append(ruleTmul0dcd1T)
    ruleTmul0dcd1Tmul0dcd0 = FSM_Rule(Tmul0dcd1, Tmul0dcd0, ")")
    rules.append(ruleTmul0dcd1Tmul0dcd0)

    ruleTmul0dcd2Tmul0dcd3 = FSM_Rule(Tmul0dcd2, Tmul0dcd3, digit_chars)
    rules.append(ruleTmul0dcd2Tmul0dcd3)
    ruleTmul0dcd2T = FSM_Rule(Tmul0dcd2, T, ascii_not_digits + [","])
    rules.append(ruleTmul0dcd2T)
    ruleTmul0dcd2Tmul0dcd0 = FSM_Rule(Tmul0dcd2, Tmul0dcd0, ")")
    rules.append(ruleTmul0dcd2Tmul0dcd0)

    ruleTmul0dcd3Tmul0dcd0 = FSM_Rule(Tmul0dcd3, Tmul0dcd0, ")")
    rules.append(ruleTmul0dcd3Tmul0dcd0)
    Tmul0dcd3T_chars = ascii_chars.copy()
    Tmul0dcd3T_chars.remove(")")
    ruleTmul0dcd3T = FSM_Rule(Tmul0dcd3, T, Tmul0dcd3T_chars)
    rules.append(ruleTmul0dcd3T)

    ruleTmul0dcd0Tm = FSM_Rule(Tmul0dcd0, Tm, "m")
    rules.append(ruleTmul0dcd0Tm)
    Tmul0dcd0T_chars = ascii_chars.copy()
    Tmul0dcd0T_chars.remove("m")
    ruleTmul0dcd0T = FSM_Rule(Tmul0dcd0, T, Tmul0dcd0T_chars)
    rules.append(ruleTmul0dcd0T)
    
    ruleTTd = FSM_Rule(T, Td, "d")
    rules.append(ruleTTd)

    ruleTdTdo = FSM_Rule(Td, Tdo, "o")
    rules.append(ruleTdTdo)
    TdT_chars = ascii_chars.copy()
    TdT_chars.remove("o")
    ruleTdT = FSM_Rule(Td, T, TdT_chars)
    rules.append(ruleTdT)

    ruleTdoTdon = FSM_Rule(Tdo, Tdon, "n")
    rules.append(ruleTdoTdon)
    TdoT_chars = ascii_chars.copy()
    TdoT_chars.remove("n")
    ruleTdoT = FSM_Rule(Tdo, T, TdoT_chars)
    rules.append(ruleTdoT)

    ruleTdonTdona = FSM_Rule(Tdon, Tdona, "'")
    rules.append(ruleTdonTdona)
    TdonT_chars = ascii_chars.copy()
    TdonT_chars.remove("'")
    ruleTdonT = FSM_Rule(Tdon, T, TdonT_chars)
    rules.append(ruleTdonT)

    ruleTdonaTdonat = FSM_Rule(Tdona, Tdonat, "t")
    rules.append(ruleTdonaTdonat)
    TdonaT_chars = ascii_chars.copy()
    TdonaT_chars.remove("t")
    ruleTdonaT = FSM_Rule(Tdona, T, TdonaT_chars)
    rules.append(ruleTdonaT)

    ruleTdonatTdonat0 = FSM_Rule(Tdonat, Tdonat0, "(")
    rules.append(ruleTdonatTdonat0)
    TdonatT_chars = ascii_chars.copy()
    TdonatT_chars.remove("(")
    ruleTdonatT = FSM_Rule(Tdonat, T, TdonatT_chars)
    rules.append(ruleTdonatT)

    ruleTdonat0F = FSM_Rule(Tdonat0, F, ")")
    rules.append(ruleTdonat0F)
    Tdonat0T_chars = ascii_chars.copy()
    Tdonat0T_chars.remove(")")
    ruleTdonat0T = FSM_Rule(Tdonat0, T, Tdonat0T_chars)
    rules.append(ruleTdonat0T)

    ruleFFd = FSM_Rule(F, Fd, "d")
    rules.append(ruleFFd)
    FF_chars = ascii_chars.copy()
    FF_chars.remove("d")
    ruleFF = FSM_Rule(F, F, FF_chars)
    rules.append(ruleFF)

    ruleFdFdo = FSM_Rule(Fd, Fdo, "o")
    rules.append(ruleFdFdo)
    FdF_chars = ascii_chars.copy()
    FdF_chars.remove("o")
    ruleFdF = FSM_Rule(Fd, F, FdF_chars)
    rules.append(ruleFdF)

    ruleFdoFdo0 = FSM_Rule(Fdo, Fdo0, "(")
    rules.append(ruleFdoFdo0)
    FdoF_chars = ascii_chars.copy()
    FdoF_chars.remove("(")
    ruleFdoF = FSM_Rule(Fdo, F, FdoF_chars)
    rules.append(ruleFdoF)

    ruleFdo0T = FSM_Rule(Fdo0, T, ")")
    rules.append(ruleFdo0T)
    Fdo0F_chars = ascii_chars.copy()
    Fdo0F_chars.remove(")")
    ruleFdo0F = FSM_Rule(Fdo0, F, Fdo0F_chars)
    rules.append(ruleFdo0F)
    

    mul_FSM = FiniteStateMachine(states, rules)
    mul_FSM.add_input(corrupted_memory)
    #test_FSM.add_input(input_str)

    #for _ in range(10):
    #    print(test_FSM.current_state)
    #    test_FSM.advance_FSM()
    total_sum = 0
    digit1 = 0
    digit2 = 0
    digits = ''
    while len(mul_FSM.input_str) > 0:
        #print(mul_FSM.current_state, mul_FSM.input_str)
        mul_FSM.advance_FSM()
        if mul_FSM.current_state == FSM_State('tmul0'):
            digit1 = 0
            digit2 = 0
            digits = mul_FSM.input_str[:7]
        elif mul_FSM.current_state == FSM_State('tmul0d1'):
            digit1 = int(digits[:1])
        elif mul_FSM.current_state == FSM_State('tmul0d2'):
            digit1 = int(digits[:2])
        elif mul_FSM.current_state == FSM_State('tmul0d3'):
            digit1 = int(digits[:3])
        elif mul_FSM.current_state == FSM_State('tmul0dcd1'):
            digit2 = int(digits[len(str(digit1)) + 1:len(str(digit1)) + 2])
        elif mul_FSM.current_state == FSM_State('tmul0dcd2'):
            digit2 = int(digits[len(str(digit1)) + 1:len(str(digit1)) + 3])
        elif mul_FSM.current_state == FSM_State('tmul0dcd3'):
            digit2 = int(digits[len(str(digit1)) + 1:len(str(digit1)) + 4])
        elif mul_FSM.current_state == FSM_State('tmul0dcd0'):
            total_sum += digit1 * digit2
            digit1 = 0
            digit2 = 0
            digits = ''
    return total_sum

    

if __name__ == "__main__":
    #corrupted_memory = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    corrupted_memory = process_input('input-03.txt')

    
    total_sum = process_memory(corrupted_memory)

    print(f"Total of mul's is {total_sum}")
