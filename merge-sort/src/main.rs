use std::env;

fn main() {

    let args: Vec<_> = env::args().collect();
    let mut sub_arrays: Vec<Vec<u8>> = Vec::new();
    let mut sub_array: Vec<u8> = Vec::new();

    for arg in args {

        let input = arg.trim().parse::<u8>();

        /* ignores additional input before digits */
        if input.is_err() {
            continue;
        }

        sub_array.push(input.unwrap());
    }

    sub_arrays.push(sub_array);
}
