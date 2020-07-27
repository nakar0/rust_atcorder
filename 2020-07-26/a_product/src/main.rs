use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let splited: Vec<i32> = input.split_whitespace()
                            .map(|s| s.parse().expect("aaaaaaaaa"))
                            .collect();

    let total = splited[0] * splited[1];
    if total % 2 == 0 {
        println!("Even")
    } else {
        println!("Odd")
    }
}
