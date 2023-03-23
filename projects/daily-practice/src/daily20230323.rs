fn bubble_sort(arr: &mut [i32]) -> &mut [i32] {
    let len = arr.len() - 1;

    for i in 0..len {
        for j in 0..len - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    arr
}

fn two_crystal_balls(breaks: &[bool]) -> Option<usize> {
    let jump_amount = (breaks.len() as f32).sqrt().floor() as usize;
    let mut position = 0;
    for i in (jump_amount..breaks.len()).step_by(jump_amount) {
        if breaks[i] {
            position = i - jump_amount;
            break;
        }
    }

    (position..position + jump_amount + 1).find(|&i| breaks[i])
}

#[test]
fn bubble_sort_works() {
    assert_eq!(
        bubble_sort(&mut [6, 2, 1, 8, 9, 4]),
        &mut [1, 2, 4, 6, 8, 9]
    );

    assert_eq!(
        bubble_sort(&mut [
            873, 669, 298, 285, 318, 717, 152, 5, 435, 789, 548, 531, 703, 139, 643, 179, 784, 398,
            376, 12, 561, 333, 536, 508, 753, 587, 82, 763, 620, 716, 858, 634, 702, 137, 631, 847,
            463, 945, 459, 444, 881, 94, 740, 864, 905, 524, 470, 366, 335, 165, 424, 108, 527,
            906, 53, 839, 927, 811, 995, 860, 38, 678, 743, 605, 883, 874, 676, 292, 470, 271, 305,
            712, 949, 401, 374, 540, 154, 297, 933, 381, 871, 729, 735, 344, 691, 502, 913, 42,
            288, 227, 613, 732, 347, 710, 217, 630, 411, 535, 952, 126, 429, 447, 849,
        ]),
        [
            5, 12, 38, 42, 53, 82, 94, 108, 126, 137, 139, 152, 154, 165, 179, 217, 227, 271, 285,
            288, 292, 297, 298, 305, 318, 333, 335, 344, 347, 366, 374, 376, 381, 398, 401, 411,
            424, 429, 435, 444, 447, 459, 463, 470, 470, 502, 508, 524, 527, 531, 535, 536, 540,
            548, 561, 587, 605, 613, 620, 630, 631, 634, 643, 669, 676, 678, 691, 702, 703, 710,
            712, 716, 717, 729, 732, 735, 740, 743, 753, 763, 784, 789, 811, 839, 847, 849, 858,
            860, 864, 871, 873, 874, 881, 883, 905, 906, 913, 927, 933, 945, 949, 952, 995,
        ]
    );
}

#[test]
fn two_crystal_balls_works() {
    assert_eq!(
        two_crystal_balls(&[false, false, false, false, false, false, false, false, false, false]),
        None
    );
    assert_eq!(
        two_crystal_balls(&[false, false, true, true, true, true, true, true, true, true]).unwrap(),
        2
    );
    assert_eq!(
        two_crystal_balls(&[false, false, false, true, true, true, true, true, true, true])
            .unwrap(),
        3
    );
    assert_eq!(
        two_crystal_balls(&[false, false, false, false, false, false, true, true, true, true])
            .unwrap(),
        6
    );
    assert_eq!(
        two_crystal_balls(&[false, false, false, false, false, false, false, false, true, true])
            .unwrap(),
        8
    );
}
