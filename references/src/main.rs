fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut test_string = String::from("hello");

    change(&mut test_string);

    // Attempt to produce dangling reference
    let reference_to_something = no_dangle();
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.


fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn no_dangle() -> String {
    String::from("hello")
}