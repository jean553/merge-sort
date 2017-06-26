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
            source,
            destination,
            start,
            middle,
        );

        merge_sort(
            source,
            destination,
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
}
