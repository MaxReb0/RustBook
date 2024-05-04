fn main() {
    // let s = "hello"; // String literal

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String.

    println!("{}", s);

    // Integer example portion.
    let x = 5;
    let y = x;

    // String counterpart.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("Here is the value of s1: {s1}");

    println!("x = {}, y = {}", x, y);

    // Ownership in functions example
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    println!("Here is x from the stack: {}", x);
}

fn takes_ownership(some_string: String) { //some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {// some_integer comes into scope
    println!("{}", some_integer)
} // Here, some_integer goes out of scope. Nothing special happens.