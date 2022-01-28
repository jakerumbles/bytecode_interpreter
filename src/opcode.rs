use std::fmt;

#[derive(Debug)]
pub struct OpCode {
    instruction: String,
    operand: Option<String>,
}

impl OpCode {
    pub fn new(instruction: String, operand: Option<String>) -> OpCode {
        OpCode {
            instruction,
            operand,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.operand == None {
            return write!(f, "{}", self.instruction);
        } else {
            let operand = self.operand.as_ref().unwrap();
            return write!(f, "{} {}", self.instruction, operand);
        }
    }
}
