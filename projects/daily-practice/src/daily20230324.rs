fn binary_search(needle: &[i32], haystack: &i32) -> bool {
    let mut left = 0;
    let mut right = needle.len();

    while left < right {
        let middle = left + (right - left) / 2;

        match needle[middle].cmp(haystack) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Less => left = middle + 1,
            std::cmp::Ordering::Greater => right = middle,
        };
    }

    false
}

#[test]
fn it_works() {
    assert!(binary_search(
        &[1, 2, 4, 6, 8, 10, 22, 40, 50, 56, 100, 102, 200, 300],
        &22,
    ));
    assert!(!binary_search(
        &[1, 2, 4, 6, 8, 10, 22, 40, 50, 56, 100, 102, 200, 300],
        &3
    ));
}
