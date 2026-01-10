use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("err");
    let answer = solve(input);
    println!("{}", answer)
}

fn solve(input: String) -> u32 {
    {
        let mut viable: u32 = 0;
        fn sum_all(args: &[u32]) -> u32 {
            let sum: u32 = args.iter().sum();
            if sum < 4 { 1 } else { 0 }
        }
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
        let length: usize = input_array_int.len();
        let row_col: usize = length.isqrt();
        println!("{input_array_int:?} of lenght {length}");
        for (i, e) in input_array_int.iter().enumerate() {
            // let i = usize_i as u32;
            if e == &1 {
                if i == 0 {
                    viable += sum_all(&[
                        input_array_int[i + 1],
                        input_array_int[i + row_col],
                        input_array_int[i + row_col + 1],
                    ]);
                } else if i == row_col - 1 {
                    viable += sum_all(&[
                        input_array_int[i - 1],
                        input_array_int[i + row_col - 1],
                        input_array_int[i + row_col],
                    ]);
                } else if i == length - row_col {
                    viable += sum_all(&[
                        input_array_int[i + 1],
                        input_array_int[i - row_col],
                        input_array_int[i - row_col + 1],
                    ]);
                } else if i == length - 1 {
                    viable += sum_all(&[
                        input_array_int[i - 1],
                        input_array_int[i - row_col],
                        input_array_int[i - row_col - 1],
                    ]);
                    println!("index {i} is in the corner")
                } else {
                    if i % row_col == 0 {
                        viable += sum_all(&[
                            input_array_int[i - row_col],
                            input_array_int[i - row_col + 1],
                            input_array_int[i + 1],
                            input_array_int[i + row_col],
                            input_array_int[i + row_col + 1],
                        ]);
                        println!("index {i} is on the right side")
                    } else if (i + 1) % row_col == 0 {
                        viable += sum_all(&[
                            input_array_int[i - row_col - 1],
                            input_array_int[i - row_col],
                            input_array_int[i - 1],
                            input_array_int[i + row_col - 1],
                            input_array_int[i + row_col],
                        ]);
                        println!("index {i} is on the left side")
                    } else if i < row_col {
                        viable += sum_all(&[
                            input_array_int[i - 1],
                            input_array_int[i + 1],
                            input_array_int[i + row_col - 1],
                            input_array_int[i + row_col],
                            input_array_int[i + row_col + 1],
                        ]);
                        println!("index {i} is on the top")
                    } else if i > (length - row_col) {
                        viable += sum_all(&[
                            input_array_int[i - row_col - 1],
                            input_array_int[i - row_col],
                            input_array_int[i - row_col + 1],
                            input_array_int[i - 1],
                            input_array_int[i + 1],
                        ]);
                        println!("index {i} is on the bottom")
                    } else {
                        viable += sum_all(&[
                            input_array_int[i - row_col - 1],
                            input_array_int[i - row_col],
                            input_array_int[i - row_col + 1],
                            input_array_int[i - 1],
                            input_array_int[i + 1],
                            input_array_int[i + row_col - 1],
                            input_array_int[i + row_col],
                            input_array_int[i + row_col + 1],
                        ]);
                    }
                }
            };
        }
        viable
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_4x4_full() {
        let matrix = "@@@@\n@@@@\n@@@@\n@@@@";

        let got = solve(matrix.into());
        let exp: u32 = 4;

        assert_eq!(got, exp, "expected {exp} got {got}");
    }
    #[test]
    fn test_4x4_no_corner() {
        let matrix = ".@@.\n@@@@\n@@@@\n.@@.";

        let got = solve(matrix.into());
        let exp: u32 = 0;

        assert_eq!(got, exp, "expected {exp} got {got}");
    }
}
