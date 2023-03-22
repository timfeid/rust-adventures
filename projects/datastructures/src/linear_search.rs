fn linear_search(haysack: &[i32; 1024], needle: i32) -> bool {
    for int in haysack {
        if &needle == int {
            return true;
        }
    }

    false
}

#[test]
fn it_does_not_find_5() {
    let haysack = [0; 1024];
    assert!(!linear_search(&haysack, 5));
}

#[test]
fn it_does_find_5() {
    let mut haysack = [0; 1024];
    haysack[120] = 5;
    assert!(linear_search(&haysack, 5));
}
