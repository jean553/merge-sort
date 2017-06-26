mod ms {

    pub fn merge_sort(
        source: &mut [u8],
        destination: &mut [u8],
        start: usize,
        end: usize,
    ) {
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_with_two_items_array() {

        let source = [7, 5];
        let destination = [0, 0];

        merge_sort(
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
