fn main() {
    // using let to create an immutable variable
    /* let number_of_bugs: i32 = 0;
    println!(
        "Hello there, there are {} bugs in this code...",
        number_of_bugs
    ); */

    // create a mutable variable
    /* let mut number_of_lines: i32 = 12;
    println!("There are {} lines of codes in this file", number_of_lines);

    // now we can mutate the value of number_of_lines
    number_of_lines = 5;
    println!(
        "Now, there are {} lines of codes in this file",
        number_of_lines
    ); */

    // data types
    // there are two types: scalar and compound
    // scalar type: represents a single value
    // there are 4 primary scalar types
    // - integers, floating-point numbers, booleans, characters
    // int32, int64, f32, f64, bool, char
    // compound types
    // - tuples and arrays
    // tuple => (t1,t2,t3) = (v1 of t1, v2 of t2, v3 of t3);
    /* let desc_tup = ("Daniel", 27, 'A');
    let first_name = desc_tup.0;
    let age = desc_tup.1;
    let grade = desc_tup.2;
    println!("The values of tuple: name({first_name}), age({age}) and grade({grade})"); */

    // array => is a fixed collection of multiple values of the same type
    /* let even_numbers = [2, 4, 6, 8];
    println!(
        "The even numbers are: {}, {} and {}",
        even_numbers[0], even_numbers[1], even_numbers[2]
    ); */

    // function: is a blocked of organized and reusable code that performs a single action
    sample_function();
}

fn sample_function() {
    println!("This is a sample function");
}
