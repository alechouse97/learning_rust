use rand::Rng;

fn main() {

    let mut vec: Vec<i32> = Vec::new();

    for _ in 0..500 {
        let num = rand::thread_rng().gen_range(1..1000001);
        vec.push(num);
    }

    vec.sort();

    // let y = 888;
    let y = vec[17];
    println!("value to find: {}", y);

    let idx = simple_binary_sort(vec, y);

    println!("idx where vec = {} is {}", y, idx);

}

fn simple_binary_sort(x: Vec<i32>, val2find: i32) -> i32 {

    let mut a = 0;
    let mut b = x.len();
    
    let mut cnt = 0;

    loop {

        let middle = (a+b)/2;
        let val = x[middle as usize];
        println!("iteration {}: x[{}] = {}",cnt,middle,val);

        if val == val2find {
            break (middle as i32)
        } else if val > val2find {
            b = middle;
        } else if val < val2find{
            a = middle;
        }

        if cnt > 10 {
            break 999999999
        } else {
            cnt += 1;
        }        
    }
}
