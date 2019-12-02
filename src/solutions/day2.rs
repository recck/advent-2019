use itertools::iproduct;

pub fn solve(input: String) {
    let opcodes: Vec<i32> = input
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut part1_opcodes = opcodes.to_vec();
    process_opcodes(&mut part1_opcodes, 12, 2);

    println!("Part 1: {}", part1_opcodes[0]);

    for (noun, verb) in iproduct!(0..100, 0..100) {
        let mut cur_opcodes = opcodes.to_vec();
        process_opcodes(&mut cur_opcodes, noun, verb);

        if cur_opcodes[0] == 19690720 {
            println!("Part 2: {}", 100 * noun + verb);
            break;
        }
    }
}

fn process_opcodes(opcodes: &mut Vec<i32>, noun: i32, verb: i32) {
    let mut cur_index: usize = 0;

    opcodes[1] = noun;
    opcodes[2] = verb;

    loop {
        match opcodes.get(cur_index) {
            Some(99) | None => {
                break;
            },
            Some(1) | Some(2) => {
                let pos: usize = opcodes[cur_index + 3] as usize;
                let left: i32 = opcodes[opcodes[cur_index + 1] as usize];
                let right: i32 = opcodes[opcodes[cur_index + 2] as usize];

                if opcodes[cur_index] == 1 {
                    opcodes[pos] = left + right;
                } else {
                    opcodes[pos] = left * right;
                }

                cur_index += 4;
            },
            Some(unknown_opcode) => {
                println!("unknown opcode {} at index {}", unknown_opcode, cur_index);
                break;
            }
        }
    }
}