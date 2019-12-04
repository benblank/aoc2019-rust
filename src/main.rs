use std::env;

fn main() {
    for arg in env::args().skip(1) {
        match arg.as_ref() {
            "1" => println!("Hello, world."),
            _ => eprintln!("Day {} hasn't been written yet!", arg),
        }
    }
}
