fn main() {
    let mut x = 1;
    loop {
        x = x * 2;
        if x > 5000 {
            break; //break forces the loop to stop right here and breaks to the next line after loop
        }
        println!("the value of x is {}", x)
    }
    //while
    let mut y = 1;
    while y < 5000 {
        y = y * 2;
        println!("value of y is {}", y)
    }
    //for loop, notice the range syntax STOPS at the ending number
    for a in 0..10 {
        println!("the value of a is {}", a)
    }
    //need to use = to include the ending number
    for b in 0..=10 {
        println!("the value of b is {}", b)
    }
    //loop over an array
    let g = [1, 2, 3];
    for h in g {
        println!("the value of h is {}", h)
    }
    println!("code has ended")
}
