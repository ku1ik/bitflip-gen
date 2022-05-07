use std::env;

fn flips(input: &str) -> Vec<String> {
    let masks: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
    let mut outputs = vec![];

    for (i, c) in input.char_indices() {
        for m in masks {
            let b = c as u8;
            let b2 = b ^ m;
            let c2 = b2 as char;

            if c2.is_ascii() {
                let mut s = String::new();
                s.push_str(&input[0..i]);
                s.push(c2);
                s.push_str(&input[i+1..]);
                outputs.push(s);
            }
        }
    }

    outputs
}

fn main() -> Result<(), String> {
    let input = &env::args().nth(1).ok_or("missing input arg")?;

    for d in flips(input) {
        println!("{}", d);
    }

    Ok(())
}
