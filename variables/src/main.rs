fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //constants use const instead of let and their type must be annotated
    //can also be declared in any scope
    //must be set to a constant expression and not a result computed at run time
    const MAX_POINTS: u32 = 100_000;
    println!("The value of Max Points is: {}", MAX_POINTS);

    //It will not return an error despite being immutable, as we use the let keyword
    //this is known as variable shadowing, variable is immutable after transformations
    //also used to change the type of the variable
    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("The value of y is: {}", y);

    //The following will return an error as mut does not allow the type of a variable to be mutated
    //shadowing must be done to change the type of the variable
    // let mut spaces = "   ";
    // spaces = spaces.len();
    

}
