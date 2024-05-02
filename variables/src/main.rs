fn main() {
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of the constant is: {THREE_HOURS_IN_SECONDS}");

    {
        // Let's try this with inner scope.

        let x = 5;

        let x = x + 1;

        {
            let x  = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }

        println!("The value of x is: {x}")
    }

    let spaces = "      ";
    let spaces = spaces.len();
}
