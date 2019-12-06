struct InstructionSet {
    opcode: Instruction,
    first_mode: ParameterMode,
    second_mode: ParameterMode,
    third_mode: ParameterMode,
}

#[derive(PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(PartialEq)]
enum Instruction {
    Add,
    Multiply,
    Set,
    Read,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

pub fn solve(input: String) {
    let opcodes: Vec<i32> = input.split(',').map(|s| s.parse().unwrap()).collect();

    let mut part1_opcodes = opcodes.to_vec();
    process_opcodes(&mut part1_opcodes, 5);
}

fn process_opcodes(opcodes: &mut Vec<i32>, input: i32) {
    let mut cur_index: usize = 0;

    loop {
        match opcodes.get(cur_index) {
            Some(opcode) => {
                let instruction: InstructionSet = get_instruction(*opcode);

                if instruction.opcode == Instruction::Halt {
                    break;
                }

                let mut first_param: i32 = if instruction.first_mode == ParameterMode::Immediate {
                    opcodes[cur_index + 1]
                } else {
                    opcodes[opcodes[cur_index + 1] as usize]
                };

                match instruction.opcode {
                    Instruction::Add | Instruction::Multiply => {
                        let second_param: i32 =
                            if instruction.second_mode == ParameterMode::Immediate {
                                opcodes[cur_index + 2]
                            } else {
                                opcodes[opcodes[cur_index + 2] as usize]
                            };
                        let third_param: usize = opcodes[cur_index + 3] as usize;

                        if instruction.opcode == Instruction::Add {
                            opcodes[third_param] = first_param + second_param;
                        } else {
                            opcodes[third_param] = first_param * second_param;
                        }

                        cur_index += 4;
                    }
                    Instruction::Set => {
                        first_param = opcodes[cur_index + 1];
                        opcodes[first_param as usize] = input;

                        cur_index += 2;
                    }
                    Instruction::Read => {
                        println!("{}", first_param);

                        cur_index += 2;
                    }
                    Instruction::JumpIfTrue | Instruction::JumpIfFalse => {
                        let second_param: i32 =
                            if instruction.second_mode == ParameterMode::Immediate {
                                opcodes[cur_index + 2]
                            } else {
                                opcodes[opcodes[cur_index + 2] as usize]
                            };

                        let should_jump: bool = first_param != 0
                            && instruction.opcode == Instruction::JumpIfTrue
                            || first_param == 0 && instruction.opcode == Instruction::JumpIfFalse;

                        if should_jump {
                            cur_index = second_param as usize;
                        } else {
                            cur_index += 3;
                        }
                    }
                    Instruction::LessThan | Instruction::Equals => {
                        let second_param: i32 =
                            if instruction.second_mode == ParameterMode::Immediate {
                                opcodes[cur_index + 2]
                            } else {
                                opcodes[opcodes[cur_index + 2] as usize]
                            };
                        let third_param: usize = opcodes[cur_index + 3] as usize;

                        let statement_truth: bool = (instruction.opcode == Instruction::LessThan
                            && first_param < second_param)
                            || (instruction.opcode == Instruction::Equals
                                && first_param == second_param);

                        opcodes[third_param] = if statement_truth { 1 } else { 0 };

                        cur_index += 4;
                    }
                    _ => {
                        break;
                    }
                }
            }
            None => {
                break;
            }
        }
    }
}

fn get_instruction(input_opcode: i32) -> InstructionSet {
    let formatted: String = format!("{:0>5}", input_opcode.to_string());

    unsafe {
        InstructionSet {
            opcode: get_opcode(formatted.get_unchecked(3..5).parse::<i32>().unwrap()),
            first_mode: get_parameter_mode(formatted.get_unchecked(2..3).parse::<i32>().unwrap()),
            second_mode: get_parameter_mode(formatted.get_unchecked(1..2).parse::<i32>().unwrap()),
            third_mode: get_parameter_mode(formatted.get_unchecked(0..1).parse::<i32>().unwrap()),
        }
    }
}

fn get_opcode(raw_opcode: i32) -> Instruction {
    match raw_opcode {
        1 => Instruction::Add,
        2 => Instruction::Multiply,
        3 => Instruction::Set,
        4 => Instruction::Read,
        5 => Instruction::JumpIfTrue,
        6 => Instruction::JumpIfFalse,
        7 => Instruction::LessThan,
        8 => Instruction::Equals,
        99 => Instruction::Halt,
        _ => panic!("Unknown opcode {}", raw_opcode),
    }
}

fn get_parameter_mode(raw_mode: i32) -> ParameterMode {
    match raw_mode {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        _ => panic!("Unknown mode {}", raw_mode),
    }
}
