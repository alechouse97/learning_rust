fn main() {
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("x = {} , y = {} , z = {}", x, y, z);

    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    let mut a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];
    
    a[3] = 7;
}
