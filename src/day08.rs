use crate::file_util::read_file_to_string;

struct Instruction {
    operation: String, //acc, jmp, or nop
    argument: i32,
    execution_count: u8,
}

fn get_instruction_from_line(line: &String) -> Instruction {
    let instruction_input: Vec<&str> = line.split_whitespace().collect();
    let operation = instruction_input[0].to_string();
    let argument = instruction_input[1].to_string().parse::<i32>().unwrap();
    return Instruction {
        operation,
        argument,
        execution_count: 0,
    };
}

fn reset_count(instructions: &mut Vec<Instruction>) {
    for ins in instructions {
        ins.execution_count = 0;
    }
}

fn run(instructions: &mut Vec<Instruction>, is_flip: bool, flip_index: usize) -> (i32, bool) {
    reset_count(instructions);

    if is_flip {
        flip(&mut instructions[flip_index]);
    }

    let mut accumulator: i32 = 0;
    let mut index: i32 = 0;

    loop {
        if index >= instructions.len() as i32 {
            return (accumulator, false);
        }

        let mut instruction = &mut instructions[index as usize];

        if instruction.execution_count > 0 {
            if is_flip {
                flip(&mut instructions[flip_index]);
            }
            return (accumulator, true);
        } else {
            instruction.execution_count += 1;
        }

        match instruction.operation.as_str() {
            "acc" => {
                accumulator += instruction.argument;
                index += 1;
            }
            "jmp" => {
                index += instruction.argument;
            }
            "nop" => {
                index += 1;
            }
            _ => {}
        }
    }
}

fn flip(instruction: &mut Instruction) {
    match instruction.operation.as_str() {
        "jmp" => {
            instruction.operation = "nop".to_string();
        }
        "nop" => {
            instruction.operation = "jmp".to_string();
        }
        "acc" => {}
        _ => {}
    }
}

pub fn solution() -> i32 {
    let input = read_file_to_string("input/day08");
    let mut instructions: Vec<Instruction> = input.iter().map(|l| get_instruction_from_line(&l)).collect();
    //part1 - 1859
    //return run(&mut instructions, false, 0).0;

    //part2 - 1235
    for i in 0..instructions.len() {
        let (acc, bad_code) = run(&mut instructions, true, i);
        if !bad_code {
            return acc;
        }
    }
    panic!("no result found :(")
}