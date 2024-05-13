fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1: String = String::from("foo");
    let s2: &str = "bar";
    s1.push_str(s2);
    // println!("s2 is: {s2}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;

    // This is much cleaner: It also doesn't take ownership of any values.
    let s = format!("{s1}-{s2}-{s3}");

    for c in "Зд".chars() {
        println!("{c}");
    }
}
