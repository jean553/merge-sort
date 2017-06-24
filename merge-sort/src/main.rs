use std::env;

fn main() {

    let args: Vec<_> = env::args().collect();
    let mut unordered_array: Vec<u8> = Vec::new();

    for arg in args {

        let input = arg.trim().parse::<u8>();

        /* ignores additional input before digits */
        if input.is_err() {
            continue;
        }

        unordered_array.push(input.unwrap());
    }
}
