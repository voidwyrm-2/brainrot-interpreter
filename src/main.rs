use std::env;

fn main() {
    //println!("WHAT");
    let mut args: Vec<String> = env::args().collect(); // grabs the launch arguments as a list of Strings
    if args.len() > 3 {
        println!("You must specify one argument.");
        return;
    }
    if args.len() > 1 {
        args.remove(0);
        println!("{:?}", args)
    } else {
        println!(
            "You must input a valid .oroproj file to run. (via --path (-p) )\nUse --help, -h, help, or h for help."
        );
        return;
    }
}
