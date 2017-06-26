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
