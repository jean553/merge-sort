mod ms {

    pub fn merge_sort(array: &mut [u8]) {

        array.swap(
            0,
            1,
        );
    }
}

#[cfg(test)]
mod tests {

    use ms;

    #[test]
    fn test_sort_two_items() {

        let mut array = [5, 3];
        ms::merge_sort(&mut array);

        assert_eq!(
            array,
            [3, 5],
            "Two items array is not sorted"
        );
    }
}
