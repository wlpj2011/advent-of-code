class FSM_State():
    def __init__(self, state):
        self.state = state

        
class FSM_Rule():
    def __init__(self, rule):
        self.rule = rule

        
class FiniteStateMachine():
    def __init__(self, states, rules):
        self.states = states
        self.rules = rules

    def add_state(self, state):
        self.states.append(state)

    def add_rule(self, rule):
        self.rules.append(rule)


def process_input(filename):
    lines = []
    with open(filename, 'r') as file:
        for line in file.readlines():
            lines.append(line)
    return lines

#for each instance of 'mul(' keep searching until you find a non-digit, if you find one, check to make sure it is ',', then continue searching for non-digits and make sure the next is ')'. Then continue at the next 'mul('



    

if __name__ == "__main__":
    corrupted_memory = ["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"]
    #corrupted_memory = process_input('input-03.txt')

    #total_sum = process_memory(corrupted_memory)
    total_sum = 0
    print(f"Total of mul's is {total_sum}")
