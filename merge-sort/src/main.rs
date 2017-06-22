use std::env;

fn merge_sort(
    unordered_array: &mut Vec<u8>,
    working_array: &mut Vec<u8>,
    array_size: usize,
) {

    /* TODO: copy the unordered array into the working array */
}

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

    let mut working_array: Vec<u8> = Vec::new();
    let array_size = unordered_array.len();

    merge_sort(
        &mut unordered_array,
        &mut working_array,
        array_size,
    );
}
