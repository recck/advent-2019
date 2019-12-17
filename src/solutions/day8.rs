use std::slice::ChunksExactMut;
use itertools::join;


pub fn solve(input: String) {
    let mut input_string: Vec<u32> = input
        .trim_end()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let width: usize = 25;
    let height: usize = 6;
    let chunk_size: usize = width * height;

    let mut chunks: ChunksExactMut<'_, u32> = input_string.chunks_exact_mut(chunk_size);
    let mut min_zero: usize = std::u32::MAX as usize;
    let mut cur_count: usize = 0;
    let mut working_layer = chunks.nth(0).unwrap();

    for chunk in chunks {
        let zeros = count_number(chunk, 0);
        
        if zeros < min_zero {
            min_zero = zeros;
            cur_count = count_number(chunk, 1) * count_number(chunk, 2);
        }

        for i in 0..chunk_size {
            let working_at: u32 = working_layer[i];
            let chunk_at: u32 = chunk[i];

            if chunk_at < working_at {
                working_layer[i] = chunk_at;
            }
        }
    }

    println!("Part 1: {}", cur_count);
    println!("{:?}", join(working_layer.into_iter(), ""));
}

fn count_number(chunk: &[u32], number: u32) -> usize {
    chunk.iter().filter(|&c| *c == number).count()
}