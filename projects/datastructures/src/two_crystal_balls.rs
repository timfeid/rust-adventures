fn two_crystal_balls(breaks: &[bool]) -> usize {
    let jump_amount = (breaks.len() as f32).sqrt().floor() as usize;

    let mut i = jump_amount;

    while i < breaks.len() {
        if breaks[i] {
            break;
        }

        i += jump_amount;
    }

    i -= jump_amount;

    for _ in 0..jump_amount + 1 {
        if breaks[i] {
            return i;
        }
        i += 1;
    }

    0
}

#[test]
fn does_it() {
    assert_eq!(
        two_crystal_balls(&[false, false, false, false, true, true, true, true]),
        4
    );

    assert_eq!(
        two_crystal_balls(&[true, true, true, true, true, true, true, true]),
        0
    );

    assert_eq!(
        two_crystal_balls(&[false, true, true, true, true, true, true, true]),
        1
    );

    assert_eq!(
        two_crystal_balls(&[false, false, true, true, true, true, true, true]),
        2
    );

    assert_eq!(
        two_crystal_balls(&[false, false, false, true, true, true, true, true]),
        3
    );

    assert_eq!(
        two_crystal_balls(&[false, false, false, false, false, true, true, true]),
        5
    );

    assert_eq!(
        two_crystal_balls(&[false, false, false, false, false, false, true, true]),
        6
    );

    assert_eq!(
        two_crystal_balls(&[false, false, false, false, false, false, false, true]),
        7
    );
}
