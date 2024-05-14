use std::io::stdin;
use std::{collections::HashMap, env};

fn main() {
    //println!("WHAT");
    let mut args: Vec<String> = env::args().collect(); // grabs the launch arguments as a list of Strings
    if args.len() > 3 {
        println!("You must specify two arguments.");
        return;
    }
    if args.len() > 2 {
        args.remove(0);
        //println!("{:?}", args)
        process_args(args)
    } else {
        println!("Use --help, -h, help, or h for help.");
        return;
    }
}

fn process_args(args: Vec<String>) {
    let command = args[0].as_ref();
    match command {
        "run" => run_bf(args[1].to_owned()),
        _other => {
            println!("Unknown command '{}'", command)
        }
    }
}

fn run_bf(bfcode_raw: String) {
    let bfcode = bfcode_raw.chars();
    let mut bfcode_iter: Vec<char> = Vec::new();
    // can you tell I don't know what I'm doing?
    for c in bfcode {
        bfcode_iter.push(c)
    }

    // pre-interpretation
    let mut jumps: HashMap<u32, u32> = HashMap::new();
    let mut jumpstack: Vec<u32> = Vec::new();
    let mut ln: u32 = 0;
    for (i, c) in bfcode_iter.iter().enumerate() {
        if *c == '[' {
            jumpstack.push(i as u32)
        } else if *c == ']' {
            if jumpstack.len() > 0 {
                let opening: u32;
                match jumpstack.pop() {
                    Some(i_32) => opening = i_32,
                    None => {
                        println!("got None when expecting Some from 'opening'");
                        return;
                    }
                }
                jumps.insert(opening, i as u32);
                jumps.insert(i as u32, opening);
            } else {
                println!("error: unmatched ']' on line {}, col {}", ln + 1, i + 1);
                return;
            }
        } else if *c == '\n' {
            ln += 1;
        }
    }

    if jumpstack.len() > 0 {
        println!("error: unmatched '['");
        return;
    }

    let mut idx = 0;

    let mut bytes: [u8; 30000] = [0; 30000];
    let mut pointer: usize = 0;

    while idx < bfcode_iter.len() {
        //println!("{}: {}", idx, bfcode_iter[idx]);
        match bfcode_iter[idx] {
            '+' => {
                if bytes[pointer] == 127 {
                    bytes[pointer] = 0;
                } else {
                    bytes[pointer] += 1;
                }
                idx += 1;
            }

            '-' => {
                if bytes[pointer] == 0 {
                    bytes[pointer] = 127;
                } else {
                    bytes[pointer] -= 1;
                }
                idx += 1;
            }

            '>' => {
                if pointer == 29999 {
                    pointer = 0;
                } else {
                    pointer += 1;
                }
                idx += 1;
            }

            '<' => {
                if pointer == 0 {
                    pointer = 29999;
                } else {
                    pointer -= 1;
                }
                idx += 1;
            }

            '.' => {
                println!("i{}({})", pointer, bytes[pointer]);
                idx += 1;
            }

            '[' => {
                //println!("{:?}", jumps);
                if bytes[pointer] == 0 {
                    match jumps.get(&(idx as u32)) {
                        Some(out) => idx = 1 + (*out as usize),
                        None => {
                            println!(
                                "error at '[': tried to get value {}, but it didn't exist",
                                idx
                            );
                            return;
                        }
                    }
                } else {
                    idx += 1
                }
            }

            ']' => {
                //println!("{:?}", jumps);
                if bytes[pointer] != 0 {
                    match jumps.get(&(idx as u32)) {
                        Some(out) => idx = 1 + (*out as usize),
                        None => {
                            println!(
                                "error at ']': tried to get value {}, but it didn't exist",
                                idx
                            );
                            return;
                        }
                    }
                } else {
                    idx += 1
                }
            }

            ',' => {
                loop {
                    println!("please input a number in the range of 0-127, inclusive");

                    let mut buffer = String::new();

                    // `read_line` returns `Result` of bytes read
                    let _ = stdin().read_line(&mut buffer);
                    let given = buffer.trim_end().to_string();
                    let conversion_res = given.parse::<i16>();
                    match conversion_res {
                        Ok(o) => {
                            if o < 0 {
                                println!("that's outside of the valid range!");
                                continue;
                            }
                            bytes[pointer] = o as u8;
                            idx += 1;
                            break;
                        }
                        Err(e) => {
                            println!("that's not a number!(err: {})", e);
                            continue;
                        }
                    }
                }
            }

            _other => {
                idx += 1;
            }
        }
    }
}
