use std::io;

fn main() {
    let mut temp = String::new();
    let mut line = String::new();
    io::stdin().read_line(&mut temp).expect("aaaaaaaaaaaaa");
    io::stdin().read_line(&mut line).expect("aaaaaaaaaaaaa");

    let mut list: Vec<i32> = line.split_whitespace()
                    .map(|s| s.parse().expect("bbbbbbbbbb"))
                    .collect();

    let mut count: i32 = 0;
    loop {
        let mut temp_list: Vec<i32> = Vec::new();

        for value in list.iter() {
            if value % 2 == 1 {
                println!("{}", count);
                return;
            }

            temp_list.push(value / 2);
        }

        list = temp_list;
        count += 1;
    }
}
