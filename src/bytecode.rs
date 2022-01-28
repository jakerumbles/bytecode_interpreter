// use crate::opcode;
use std::fmt;

#[derive(Debug)]
pub enum ByteCode {
    LoadVal(usize),
    WriteVar(char),
    ReadVar(char),
    Add,
    Subtract,
    Multiply,
    Divide,
    DoWhileLt((char, char, usize)),
    ReturnValue,
}

// impl ByteCode {
//     pub fn new(instruction: String, operand: Option<String>) -> ByteCode {
//         ByteCode {
//             instruction,
//             operand,
//         }
//     }
// }

// impl fmt::Display for ByteCode {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         if self.operand == None {
//             return write!(f, "{}", self.instruction);
//         } else {
//             let operand = self.operand.as_ref().unwrap();
//             return write!(f, "{} {}", self.instruction, operand);
//         }
//     }
// }
