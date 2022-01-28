mod bytecode;
// mod opcode;

use bytecode::*;
// use opcode::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // 1. Fetch - Read an opcode from `opcodes_string`
    // 2. Decode - Create ByteCode object from opcode and load to memory (represented as mem vector)
    // 3. Execute - Execute instruction in interpreter loop

    // This functions as the hard-disk. Read from hard-disk into memory (mem)
    let opcodes_string = read_file("bytecode_loop.txt");

    let mut mem: Vec<ByteCode> = vec![];
    load_memory(&mut mem, opcodes_string);

    println!("{:?}", mem);

    // Program is loaded in memory
    // Now run program...

    let mut stack: Vec<usize> = vec![];

    // `var_map` only necessary because opcodes accept char names for variables instead of just using memory address (vector index)
    let mut var_map: HashMap<char, usize> = HashMap::new();
    let mut program_counter = 0;

    // Interpreter
    while program_counter < mem.len() {
        println!("Program Counter: {}", program_counter);
        println!("Stack: {:?}", stack);
        let opcode = &mem[program_counter];
        match opcode {
            ByteCode::LoadVal(num) => {
                stack.push(*num);
            }
            ByteCode::WriteVar(var_name) => {
                // Reference top value in stack as `x`
                if var_map.contains_key(&var_name) {
                    let val = stack.pop();
                    match var_map.get(&var_name) {
                        Some(index) => stack[*index] = val.unwrap(),
                        None => panic!("Shouldn't happen"),
                    }
                } else {
                    var_map.insert(*var_name, stack.len() - 1);
                }
            }
            ByteCode::ReadVar(var_name) => match var_map.get(&var_name) {
                Some(index) => {
                    let value = stack[*index];
                    stack.push(value);
                }
                None => panic!("Cannot read un-declared variable!"),
            },
            ByteCode::Add => {
                let num1 = pop_from_stack(&mut stack);
                let num2 = pop_from_stack(&mut stack);

                stack.push(num1 + num2);
            }
            ByteCode::Subtract => {
                let num1 = pop_from_stack(&mut stack);
                let num2 = pop_from_stack(&mut stack);

                // Randomly chose ordering here. Would need to define a convention for order of pushing numbers to stack before MULTIPLY opcode.
                stack.push(num1 - num2);
            }
            ByteCode::Multiply => {
                let num1 = pop_from_stack(&mut stack);
                let num2 = pop_from_stack(&mut stack);

                stack.push(num1 * num2);
            }
            ByteCode::Divide => {
                let num1 = pop_from_stack(&mut stack);
                let num2 = pop_from_stack(&mut stack);

                // Randomly chose ordering here. Would need to define a convention for order of pushing numbers to stack before DIVIDE opcode.
                stack.push(num1 / num2);
            }
            ByteCode::ReturnValue => {
                let final_num = pop_from_stack(&mut stack);
                println!("Final value is {}", final_num);
            }
            ByteCode::DoWhileLt((var1, var2, loop_beginning)) => {
                println!("DO_WHILE_LT called");
                let index1: usize;
                let index2: usize;

                match var_map.get(&var1) {
                    Some(index) => index1 = *index,
                    None => panic!("Couldn't get index!"),
                }
                match var_map.get(&var2) {
                    Some(index) => index2 = *index,
                    None => panic!("Couldn't get index!"),
                }

                let mut i: usize = 0; // counter
                let mut m: usize = 1; // max
                i = stack[index1]; // counter
                m = stack[index2]; // max

                if i < m {
                    println!("i: {}, m: {}, loop_beginning: {}", i, m, loop_beginning);
                    program_counter = *loop_beginning - 1;
                }
            }
        }
        program_counter = program_counter + 1;
    }
    println!("{:?}", stack);

    // Program end: clean up stack
    stack.clear();
    println!("{:?}", stack);
}

fn pop_from_stack(stack: &mut Vec<usize>) -> usize {
    match stack.pop() {
        Some(num) => return num,
        None => panic!("Nothing on the stack!"),
    }
}

fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Couldn't open {}: {}", display, e),
    };

    // Read file contents into a string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => return s,
        Err(e) => panic!("Couldn't read {}: {}", display, e),
    }
}

fn load_memory(mem: &mut Vec<ByteCode>, opcodes_string: String) {
    for opcode in opcodes_string.lines() {
        let s_arr: Vec<&str> = opcode.split_whitespace().collect();
        let instruction: ByteCode;

        match s_arr[0] {
            "LOAD_VAL" => {
                instruction = ByteCode::LoadVal(s_arr[1].parse().unwrap());
            }
            "WRITE_VAR" => {
                instruction = ByteCode::WriteVar(s_arr[1].chars().nth(1).unwrap());
            }
            "READ_VAR" => {
                instruction = ByteCode::ReadVar(s_arr[1].chars().nth(1).unwrap());
            }
            "ADD" => instruction = ByteCode::Add,
            "SUBTRACT" => instruction = ByteCode::Subtract,
            "MULTIPLY" => instruction = ByteCode::Multiply,
            "DIVIDE" => instruction = ByteCode::Divide,
            "RETURN_VALUE" => instruction = ByteCode::ReturnValue,
            "DO_WHILE_LT" => {
                instruction = ByteCode::DoWhileLt((
                    s_arr[1].chars().nth(1).unwrap(),
                    s_arr[2].chars().nth(1).unwrap(),
                    s_arr[3]
                        .chars()
                        .nth(0)
                        .unwrap()
                        .to_digit(10)
                        .unwrap()
                        .try_into()
                        .unwrap(),
                ))
            }
            &_ => panic!("INVALID OPCODE: {}", opcode),
        }
        mem.push(instruction);
    }
}
