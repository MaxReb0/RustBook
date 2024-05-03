// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
}

// Exercises:

// 1.) Create a function that generates the nth fibonacci number.

fn fibonacci(n: u32) {
    let mut fib_num = 1;
    let mut prev_num = 1;
    let mut temp;

    if n == 0 {
        println!("Fibonacci number: 0");
    } else if n <= 2 {
        println!("Fibonacci number: 1");
    } else {    
        for _ in 3..n {
            temp = fib_num;
            fib_num = fib_num + prev_num;
            prev_num = temp;
        }
    }
    println!("Fibonacci number: {fib_num}")
}