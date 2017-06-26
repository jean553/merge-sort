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
mod tests;
