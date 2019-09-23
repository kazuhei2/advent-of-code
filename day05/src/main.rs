use std::fs;

fn react_polymer(chars: &mut Vec<char>) {
    let mut is_reacted = true;
    let mut start_idx = 1;

    while is_reacted {
        is_reacted = false;
        for i in start_idx..chars.len() {
            if chars[i-1].is_lowercase() == !chars[i].is_lowercase() {
                let c0 = chars[i-1].to_ascii_lowercase();
                let c1 = chars[i].to_ascii_lowercase();
                if c0 == c1 {
                    chars.remove(i-1);
                    chars.remove(i-1);
                    start_idx = if i > 1 { i-1 } else { 1 };
                    is_reacted = true;
                    break;
                }
            }
        }
    }
}

fn main() {
    const INPUT_DATA_PATH: &str = "../input/05.txt";

    let input_text = fs::read_to_string(INPUT_DATA_PATH)
        .expect("Error");
    let mut chars: Vec<char> = input_text.chars().collect();

    //println!("chars: {:?}", chars);
    println!("Input units are {}", chars.len());

    react_polymer(&mut chars);

    //println!("{:?}", chars);
    println!("Answer is {}", chars.len());
}
