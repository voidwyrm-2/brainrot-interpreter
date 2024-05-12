use std::env;

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

    let mut idx = 0;

    let mut bytes: [i8; 30000] = [0; 30000];
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

            _other => {
                idx += 1;
            }
        }
    }
}
