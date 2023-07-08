fn send_message(s: &[char], a: &[i32], mut str: String, k: usize, to: usize) -> String {
    let letter = s[k];
    str.push(letter);

    if a[k] == 0 {
        return str;
    }

    send_message(s, a, str, a[k] as usize, to + 1)
}

fn decode(s: &[char], a: &[i32]) -> String {
    send_message(s, a, String::from(""), 0, 0)
}

#[test]
fn code() {
    let s = ['c', 'd', 'e', 'o'];
    let a = [3, 2, 0, 1];

    println!("{:#?}", decode(&s, &a));

    assert_eq!(decode(&s, &a), "code");
}

#[test]
fn centipede() {
    let s = ['c', 'd', 'e', 'e', 'n', 'e', 't', 'p', 'i'];
    let a = [5, 2, 0, 1, 6, 4, 8, 3, 7];

    assert_eq!(decode(&s, &a), "centipede");
}

#[test]
fn bat() {
    let s = ['b', 'y', 't', 'd', 'a', 'g'];
    let a = [4, 3, 0, 1, 2, 5];

    assert_eq!(decode(&s, &a), "bat");
}
