from typing import Tuple{

class FSM_State():
    # I could give each state a list of rule instad of holding a list of all rule for the FSM
    def __init__(self, state: str):
        # A FSM State is defined by a lower case ascii string of characters
        self.state = state.lower()
        return

    def __eq__(self, other: object) -> bool:
        # Two FSM States are equal if they have the same name
        if not isinstance(other, FSM_State):
            return NotImplemented
        return self.state == other.state

    def __repr__(self) -> str:
        return f"FSM_State('{self.state}')"

        
class FSM_Rule():
    def __init__(self, start_state: FSM_State, end_state: FSM_State, accepted_input: str):
        # A FSM Rule is defined by a starting and ending state and the input that must be provided to traverse the edge.
        self.start_state = start_state
        self.end_state = end_state
        self.accepted_input = accepted_input
        return

    def __eq__(self, other: object) -> bool:
        # TWo FSM Rules are equal if they have the same states and accepted input
        if not isinstance(other, FSM_Rule):
            return NotImplemented
        return (self.start_state == other.start_state) and (self.end_state == other.end_state) and (self.accepted_input == other.accepted_input)
        
class FiniteStateMachine():
    def __init__(self, states: list[FSM_State], rules: list[FSM_Rule]):
        self.states = states
        self.rules = rules
        self.current_state = FSM_State("begin")
        self.input_str = ""
        return

    def add_input(self, input_str: str) -> None:
        self.input_str += input_str
        return
    
    def add_state(self, state: FSM_State) -> None:
        self.states.append(state)
        return
    
    def add_rule(self, rule: FSM_Rule) -> None:
        self.rules.append(rule)
        return

    def advance_FSM(self) -> None:
        (new_char, remaining_input) = split_input(self.input_str)
        for rule in self.rules:
            if (rule.start_state == self.current_state) and (new_char == rule.accepted_input):
                self.current_state = rule.end_state
                self.input_str = remaining_input
                break
            
def split_input(input_str: str) -> Tuple[str, str]:
    return input_str[0], input_str[1:]

