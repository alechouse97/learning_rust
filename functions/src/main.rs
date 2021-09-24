fn main() {

    another_function(5, 6.99999);

    let x = five(88);

    println!("The value of x is: {}", x);
    
}

fn another_function(x: i32, y: f64) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five(z: i32) -> i32 {
    z*25
}