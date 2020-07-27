use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed read line");
    
    let mut count: u32 = 0;
    for s in input.trim().chars() {
        if s == '1' {
            count += 1;
        }
    }

    println!("{}", count)
}