use std::io;

fn main() {
    let mut first_line = String::new();
    let mut second_line = String::new();
    let mut third_line = String::new();

    io::stdin().read_line(&mut first_line).expect("first out!!!!");
    io::stdin().read_line(&mut second_line).expect("second out!!!!");
    io::stdin().read_line(&mut third_line).expect("third out!!!!");

    let mut total: i32 = 0;
    let first: i32 = first_line.trim().parse().expect("out");
    let list: Vec<i32> = second_line.split_whitespace().map(|s| s.parse().expect("aaaa")).collect();

    total += first;
    for num in list {
        total += num
    }

    println!("{} {}", total, third_line);
}