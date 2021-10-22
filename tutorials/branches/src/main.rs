// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }

//     let condition = false;
//     // let number = if condition { 5 } else { five(number) };
//     let number = if condition { 5 } else { five(99) };

//     println!("The value of number is: {}", number);
// }

// fn main() {
//     let mut x = 0;
//     loop {
//         x = x + 1;
//         if x > 20 {
//             break
//         } else {
//             println!("again!");
//         }        
//     }
// }

fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    let a = 0..4;
    for val in a {
        println!("number = {}", val)
    }
    println!("LIFTOFF!!!");
}

// fn five(z: i32) -> i32 {
//     z*25
// }