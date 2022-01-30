// Author: Jake Edwards
// For: Composable Questionaire
// Problem #1 and #2

mod bytecode;

use bytecode::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // 1. Fetch - Read an opcode from `mem`
    // 2. Decode - Figure out which kind of ByteCode the instruction is
    // 3. Execute - Execute instruction

    // This functions as the hard-disk. Read from hard-disk into memory (mem)
    let opcodes_string = read_file("bytecode_loop.txt");

    // Stores function instructions
    let mut mem: Vec<ByteCode> = vec![];
    load_memory(&mut mem, opcodes_string);

    println!("{:?}", mem);

    // Program is loaded in memory
    // Now run program...

    // Stack for some operand values and computation
    let mut op_stack: Vec<usize> = vec![];

    // Stack for storing local variables
    let mut local_stack: Vec<usize> = vec![];

    // `var_map` only necessary because opcodes accept char names for variables instead of just using memory address (vector index)
    let mut var_map: HashMap<char, usize> = HashMap::new();
    let mut program_counter = 0;

    // Interpreter
    while program_counter < mem.len() {
        println!("Program Counter: {}", program_counter);
        println!("op_stack: {:?}", op_stack);
        println!("local_stack: {:?}\n", local_stack);

        let opcode = &mem[program_counter];
        match opcode {
            // Push `num` onto the `op_stack`
            ByteCode::LoadVal(num) => {
                op_stack.push(*num);
            }
            // Assumes value to be written is top item in `op_stack`
            ByteCode::WriteVar(var_name) => {
                // Pop top value from `op_stack` and store in `local_stack` with lookup index from `var_map`
                match op_stack.pop() {
                    Some(val) => {
                        if var_map.contains_key(&var_name) {
                            match var_map.get(&var_name) {
                                Some(index) => local_stack[*index] = val,
                                None => panic!("Shouldn't happen"),
                            }
                        } else {
                            local_stack.push(val);
                            var_map.insert(*var_name, local_stack.len() - 1);
                        }
                    }
                    None => panic!("If this runs, bytecode is incorrect"),
                }
            }
            // Reads value from `local_stack` and pushes to `op_stack`
            ByteCode::ReadVar(var_name) => match var_map.get(&var_name) {
                Some(index) => {
                    let value = local_stack[*index];
                    op_stack.push(value);
                }
                None => panic!("Cannot read non-existant variable!"),
            },
            ByteCode::Add => {
                let num1 = pop_from_stack(&mut op_stack);
                let num2 = pop_from_stack(&mut op_stack);

                op_stack.push(num1 + num2);
            }
            ByteCode::Subtract => {
                let num1 = pop_from_stack(&mut op_stack);
                let num2 = pop_from_stack(&mut op_stack);

                // Randomly chose ordering here. Would need to define a convention for order of pushing numbers to op_stack before MULTIPLY opcode.
                op_stack.push(num2 - num1);
            }
            ByteCode::Multiply => {
                let num1 = pop_from_stack(&mut op_stack);
                let num2 = pop_from_stack(&mut op_stack);

                op_stack.push(num2 * num1);
            }
            ByteCode::Divide => {
                let num1 = pop_from_stack(&mut op_stack);
                let num2 = pop_from_stack(&mut op_stack);

                // Randomly chose ordering here. Would need to define a convention for order of pushing numbers to op_stack before DIVIDE opcode.
                op_stack.push(num2 / num1);
            }
            ByteCode::ReturnValue => {
                let final_num = pop_from_stack(&mut op_stack);
                println!("FINAL VALUE: {}", final_num);
            }
            // If `var1` is less than `var2` decrement `program_counter` to index of desired instruction in `mem`
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

                let i: usize = local_stack[index1]; // counter
                let m: usize = local_stack[index2]; // max

                if i < m {
                    println!("i: {}, m: {}, loop_beginning: {}", i, m, loop_beginning);
                    program_counter = *loop_beginning - 1;
                }
            }
        }
        program_counter = program_counter + 1;
    }
    println!("op_stack: {:?}", op_stack);
}

/// Given a mutable reference to a Vec<usize>, pop the top element from `stack` and return it.
fn pop_from_stack(stack: &mut Vec<usize>) -> usize {
    match stack.pop() {
        Some(num) => return num,
        None => panic!("Nothing on the stack!"),
    }
}

/// Given a path to a text file, read the contents of the file, write them to String `s` and return `s`.
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

/// Given a String of instructions, for each opcode in `opcodes_string` create the corresponding `ByteCode` type and push it onto the `mem` stack.
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
