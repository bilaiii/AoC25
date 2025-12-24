use std::fs;

fn main() {
    let mut viable: i32 = 0;
    fn sum_all(args: &[i32]) -> i32 {
        let sum: i32 = args.iter().sum();
        if sum < 4 {
            return 1;
        } else {
            return 0;
        }
    }
    let input = fs::read_to_string("input").expect("err");
    let mut input_array: Vec<char> = input.chars().collect();
    input_array.retain(|&x| x != '\n');
    let mut input_array_int: Vec<u32> = Vec::new();
    for e in input_array.iter() {
        if e == &'@' {
            input_array_int.push(1);
        } else if e == &'.' {
            input_array_int.push(0);
        }
    }
    let length: u32 = input_array_int.len() as u32;
    let row_col: u32 = length.isqrt();
    println!("{input_array_int:?} of lenght {length}");
    for (usize_i, e) in input_array_int.iter().enumerate() {
        let i = usize_i as u32;
        if e == &1 {
            if i == 0 {
                viable += sum_all(&[
                    input_array_int[usize_i + 1] as i32,
                    input_array_int[usize_i + row_col as usize] as i32,
                    input_array_int[usize_i + row_col as usize + 1] as i32,
                ]);
            } else if i == row_col - 1 {
                viable += sum_all(&[
                    input_array_int[usize_i - 1] as i32,
                    input_array_int[usize_i + row_col as usize - 1] as i32,
                    input_array_int[usize_i + row_col as usize] as i32,
                ]);
            } else if i == length - row_col {
                viable += sum_all(&[
                    input_array_int[usize_i + 1] as i32,
                    input_array_int[usize_i - 140] as i32,
                    input_array_int[usize_i - 139] as i32,
                ]);
            } else if i == length - 1 {
                viable += sum_all(&[
                    input_array_int[usize_i - 1] as i32,
                    input_array_int[usize_i - 140] as i32,
                    input_array_int[usize_i - 141] as i32,
                ]);
                println!("index {i} is in the corner")
            } else {
                if i % row_col == 0 {
                    viable += sum_all(&[
                        input_array_int[usize_i - row_col as usize] as i32,
                        input_array_int[usize_i - row_col as usize + 1] as i32,
                        input_array_int[usize_i + 1] as i32,
                        input_array_int[usize_i + row_col as usize] as i32,
                        input_array_int[usize_i + row_col as usize + 1] as i32,
                    ]);
                    println!("index {i} is on the right side")
                } else if (i + 1) % row_col == 0 {
                    viable += sum_all(&[
                        input_array_int[usize_i - row_col as usize - 1] as i32,
                        input_array_int[usize_i - row_col as usize] as i32,
                        input_array_int[usize_i - 1] as i32,
                        input_array_int[usize_i + row_col as usize - 1] as i32,
                        input_array_int[usize_i + row_col as usize] as i32,
                    ]);
                    println!("index {i} is on the left side")
                } else if i < row_col {
                    viable += sum_all(&[
                        input_array_int[usize_i - 1] as i32,
                        input_array_int[usize_i + 1] as i32,
                        input_array_int[usize_i + row_col as usize - 1] as i32,
                        input_array_int[usize_i + row_col as usize] as i32,
                        input_array_int[usize_i + row_col as usize + 1] as i32,
                    ]);
                    println!("index {i} is on the top")
                } else if i > (length - row_col) {
                    viable += sum_all(&[
                        input_array_int[usize_i - row_col as usize - 1] as i32,
                        input_array_int[usize_i - row_col as usize] as i32,
                        input_array_int[usize_i - row_col as usize + 1] as i32,
                        input_array_int[usize_i - 1] as i32,
                        input_array_int[usize_i + 1] as i32,
                    ]);
                    println!("index {i} is on the bottom")
                } else {
                    viable += sum_all(&[
                        input_array_int[usize_i - row_col as usize - 1] as i32,
                        input_array_int[usize_i - row_col as usize] as i32,
                        input_array_int[usize_i - row_col as usize + 1] as i32,
                        input_array_int[usize_i - 1] as i32,
                        input_array_int[usize_i + 1] as i32,
                        input_array_int[usize_i + row_col as usize - 1] as i32,
                        input_array_int[usize_i + row_col as usize] as i32,
                        input_array_int[usize_i + row_col as usize + 1] as i32,
                    ]);
                }
            }
        };
    }
    println!("{viable}")
}
