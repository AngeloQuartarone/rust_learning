use std::io;
use std::env;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    
    // in rust, the value of a variable must match the type of the variable itself
    let x: i8 = 5; // OK
    // let y: i8 = 300; NOT OK because value 300 is out of bound in a 8 bit integer
    let y: u8 = 33;

    println!("x: {}, y: {}", x, y);

    // of course there is no way to do arithmetic between variable of different type
    // let z = x + y; it doesn't work

    let x: u8 = 255;
    let y: u8 = 1;

    // let z: u16 = x + y; doesn't work
    let z: u16 = (x as u16) + (y as u16);
    println!("result of {} + {} = {}", x , y, z);

    // this is the type casting
    let x = 257u32;
    println!("x: {}", x);

    let x = 300_u32;
    println!("x: {}", x);

    let x = 27 as u32;
    println!("x: {}", x);

    // note that if the result variable is truncated while the starting types was of
    // integer instead of float
    let x: i16 = 5;
    let y: i16 = 2;
    let z = x / y;
    println!("result of {} / {} = {}", x, y, z);

    // even if we do like this!
    let x = 5;
    let y = 2;
    let z = x / y;
    println!("result of {} / {} = {}", x, y, z);

    // correct way
    // with the keyword "as" we can cast a floating number without add the "." to the value
    let x/* : f64*/ = 5 as f64;
    let y/* : f64*/ = 2 as f64;
    let z = x / y;
    println!("result of {} / {} = {}", x, y, z);

    // let see the conversion from string of user input to other type
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("error while reading");

    let mut int_input: i32 = string.trim().parse().unwrap();

    println!("string converted to int + 10: {}", int_input + 10);

    // #########################################################################################
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("error while reading");

    let mut int_input: Vec<i32> = Vec::new(); // Inizializzazione di un vettore vuoto
    let mut x: i32 = string.trim().parse().unwrap();

    if x != 0 {
        int_input.push(x);
    }

    while x != 0 {
        string.clear();
        io::stdin().read_line(&mut string).expect("error while reading");
        x = string.trim().parse().unwrap();
        if x != 0 {
            int_input.push(x);
        }
    }

    let mut sum = 0;
    for num in &int_input {
        sum = sum + num;
    }
    println!("sum: {}", sum);

}
