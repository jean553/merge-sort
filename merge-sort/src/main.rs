use std::env;

/// Invert the two arrays item if they are not in the correct order
fn merge(
    array: &mut [u8],
    start: usize,
    end: usize,
) {

    /* TODO: partially implemented */

    if array[end] < array[start] {
        array.swap(
            start,
            end,
        );
    }
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

    merge(
        &mut unordered_array,
        0,
        1,
    );

    merge(
        &mut unordered_array,
        2,
        3,
    );

    for value in unordered_array {
        println!("{}", value);
    }
}
