use std::io;


fn main() {
    // Rust have two category of primitive data types
    // 1. scalar data types
    // A scalar data type is something that has a finite set of possible values
    // each value can be compared to any other value as either equal, greater or less
    
    let x: i32 = 2;
    let y: u64 = 5;
    println!("x: {}, y: {}", x, y);
    // there are multiple "sizes" of integers (default i32)
    
    // for floating we have only 32 and 64 bits (default f64)
    let x: f32 = 5.7;
    let y: f64 = 4.9;
    println!("x: {}, y: {}", x, y);

    // we also have boolean type
    let mut boolean_value: bool = true;
    println!("boolean_value: {}", boolean_value);
    boolean_value = false;
    println!("boolean_value: {}", boolean_value);

    // character
    let character: char = 'h';
    println!("character: {}", character);



    // 2. compound data types
    // A compound data types is any data type which can be constructed in a program
    // using the programming language's primitive and other composite types

    // a tuple is a fixed length sequence of elements
    let tup = (55, 'n', true);
    // to access a tuple we must use "." followed by the index of which elements we want
    println!("tup at index 0: {}, index 1: {}, index 2: {}", tup.0, tup.1, tup.2);

    // array
    let arr = [1,2,3,4,5,6,7,8,9];
    println!("element at index 5: {}", arr[4]);
    let arr1: [char; 2] = ['a', 'b'];
    println!("arr1: {}, {}", arr1[0], arr1[1]);

    // ################################################################################################
    // Prelude: is the list of things that rust automatically import into every rust program
    // i/o are not in the prelude so you must import manually with "use std::io;"

    let mut input = String::new();
    // "&mut <variable name>" because we must pass the reference of the variable to the function
    // and then add the expect statement that catch any errors
    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("you insert: {}", input);
}
