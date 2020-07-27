use std::io;
use std::char::from_u32;

fn main() {
    let (n, _) = read_first_line();
    let alphabet: Vec<char> = generate_alphabet(n).chars().collect();
    let sorted = marge_sort(&alphabet);
    let sorted = to_string(sorted);
    println!("! {}", sorted);
}

fn to_string(chars: Vec<char>) -> String {
    let mut result = String::new();

    for c in chars {
        result += &c.to_string();
    }

    result
}

fn read_first_line() -> (u32, u32) {
    let mut first_line = String::new();

    io::stdin().read_line(&mut first_line)
        .expect("panic read first line");

    let nq: Vec<u32> = first_line.split_whitespace()
                    .map(|s| s.parse().expect("panic parse"))
                    .collect();

    (nq[0], nq[1])
}

fn generate_alphabet(len: u32) -> String {
    let init_char: char = 'A';
    let mut result: String = init_char.to_string();

    let mut char_code: u32 = init_char as u32;
    for _ in 0..len {
        char_code += 1;
        let c: char = from_u32(char_code).unwrap();
        result += &c.to_string();
    }

    result
}

fn is_lt_query(a: char, b: char) -> bool {
    println!("? {} {}", a, b);

    let mut ans: String = String::new();
    io::stdin().read_line(&mut ans).unwrap();

    ans.trim() == "<"
}

fn marge_sort(list: &[char]) -> Vec<char> {
    if list.len() == 1 {
        return list.to_vec();
    }

    let mid = list.len() / 2;
    let left: &[char] = &list[..mid];
    let right: &[char] = &list[mid..];

    let left = marge_sort(left);
    let right = marge_sort(right);

    marge(&left, &right)
}

fn marge<'a>(left: &'a [char], right: &'a [char]) -> Vec<char> {
    let mut marged: Vec<char> = Vec::new();
    let mut l_i = 0;
    let mut r_i = 0;
    let l_len = left.len();
    let r_len = right.len();

    while l_i < l_len && r_i < r_len {
        let ll: char = left[l_i];
        let rl: char = right[r_i];

        if is_lt_query(ll, rl) {
            marged.push(ll);
            l_i += 1;
        } else {
            marged.push(rl);
            r_i += 1;
        }
    }

    if l_i < l_len {
        marged.extend_from_slice(&left[l_i..])
    } else {
        marged.extend_from_slice(&right[l_i..])
    }

    marged
}
