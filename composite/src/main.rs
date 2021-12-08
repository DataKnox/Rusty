fn main() {
    let x = (5, "hello", true);
    println!("the first element is {} {}", x.0, x.1);
    // can also do something with variables
    // let a = x.1
    // let b = x.2
    // println!("first value is {}, the second value is {}", a,b)

    // another method is called destructuring to turn a tuple into other tuples
    // why this matters, i dont know. but you can
    // let (a,b,c) = x //note, if x has 3 values, your new var must have 3 parts, too
    // interestingly, if you want to ignore one of the original values, use an __rust_force_expr!
    // let (a,b,_) = x

    //arrays, and accessing arrays
    let l = [1, 2, 3, 4, 5];
    println!("the first value in l is {}", l[0])
    //remember that this is still not mutable unless you specify mut
}
