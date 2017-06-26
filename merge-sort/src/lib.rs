extern crate array_merge;

mod ms {

    use array_merge::am;

    pub fn merge_sort(
        source: &mut [u8],
        destination: &mut [u8],
        start: usize,
        end: usize,
    ) {

        if end - start < 2 {
            return;
        }

        let middle = (end + start) / 2;

        merge_sort(
            destination,
            source,
            start,
            middle,
        );

        merge_sort(
            destination,
            source,
            middle,
            end,
        );

        am::merge(
            source,
            destination,
            start,
            end,
            middle,
        );
    }
}

#[cfg(test)]
mod tests {

    use ms;

    #[test]
    fn test_with_two_items_array() {

        let mut source = [7, 5];
        let mut destination = [0, 0];

        ms::merge_sort(
            &mut source,
            &mut destination,
            0,
            2,
        );

        assert_eq!(
            destination,
            [5, 7],
            "two items array cannot be sorted",
        );
    }

    #[test]
    fn test_with_four_items_array() {

        let mut source = [7, 5, 8, 2];
        let mut destination = [7, 5, 8, 2];

        ms::merge_sort(
            &mut source,
            &mut destination,
            0,
            4,
        );

        assert_eq!(
            destination,
            [2, 5, 7, 8],
            "four items array cannot be sorted",
        );
    }

    #[test]
    fn test_with_five_items_array() {

        let mut source = [7, 5, 8, 2, 3];
        let mut destination = [7, 5, 8, 2, 3];

        ms::merge_sort(
            &mut source,
            &mut destination,
            0,
            5,
        );

        assert_eq!(
            destination,
            [2, 3, 5, 7, 8],
            "five items array cannot be sorted",
        );
    }

    #[test]
    fn test_with_five_items_array_and_two_equal_items() {

        let mut source = [7, 5, 8, 2, 3, 5];
        let mut destination = [7, 5, 8, 2, 3, 5];

        ms::merge_sort(
            &mut source,
            &mut destination,
            0,
            6,
        );

        assert_eq!(
            destination,
            [2, 3, 5, 5, 7, 8],
            "five items with two equal items array cannot be sorted",
        );
    }
}
