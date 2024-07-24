fn main() {
    println!("Hello World!");
    // variable can be typed implicitly or explicitly
    // implicit variable declaration
    let a = 5;

    //explicit variable declaration
    let b : i32 = 6;

    println!("a: {}", a);
    println!("b: {}", b);

    // also the variable declared like above are non mutable
    // to declare a mutable variable you must use
    let mut c : i32 = 55;
    println!("value of c: {}", c);
    c = 22;
    println!("value of c: {}", c);
    
    // in rust it is possible to redeclare a non mutable variable
    let x = 5;
    println!("x: {}", x);
    let x = 6;
    println!("x: {}", x);

    //it's cool that you can redeclare a non mutable variable
    // with the same name doing some operations with the old value
    let x = 22;
    println!("x: {}", x);
    let x = x + 2;
    println!("x: {}", x);

    //it's possible do shadowing by using same variable name
    // within the "{}"
    let y = 5;
    println!("y: {}", y);
    {
        //you can do operation with the value before this actual scope
        let y = y - 5;
        println!("y: {}", y);
        
        let y = 56;
        println!("y: {}", y);
    }
    let y = y + 2;
    println!("y: {}", y);
    let w = 5;
    println!("w: {}", w);

    // here you can not do
    // w:"hello";
    // because the type has already been inferred
    let w= "hello!";
    println!("w: {}", w);
}
