use std::cmp::Ordering;

fn binary_search(haysack: &[i32], needle: i32) -> bool {
    let mut left = 0;
    let mut right = haysack.len(); // do not remove 1!

    while left < right {
        let middle = left + (right - left) / 2;

        match haysack[middle].cmp(&needle) {
            Ordering::Equal => return true,
            Ordering::Less => left = middle + 1,
            Ordering::Greater => right = middle,
        };
    }

    false
}

#[test]
fn it_does_find_valid_numbers() {
    let haysack = [1, 3, 5, 6, 7, 10, 24, 60, 77, 122, 300, 325, 452, 612];
    assert!(binary_search(&haysack, 612));
    assert!(binary_search(&haysack, 452));
    assert!(binary_search(&haysack, 325));
    assert!(binary_search(&haysack, 300));
    assert!(binary_search(&haysack, 122));
    assert!(binary_search(&haysack, 77));
    assert!(binary_search(&haysack, 60));
    assert!(binary_search(&haysack, 24));
    assert!(binary_search(&haysack, 10));
    assert!(binary_search(&haysack, 7));
    assert!(binary_search(&haysack, 6));
    assert!(binary_search(&haysack, 5));
    assert!(binary_search(&haysack, 3));
    assert!(binary_search(&haysack, 1));
    assert!(!binary_search(&haysack, 500));
    assert!(!binary_search(&haysack, 0));
    assert!(!binary_search(&haysack, 2));
    assert!(!binary_search(&haysack, 4));
    assert!(!binary_search(&haysack, 11));
    assert!(!binary_search(&haysack, 66));
    assert!(!binary_search(&haysack, 88));
    assert!(!binary_search(&haysack, 144));
}
