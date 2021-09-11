//functions parameters must be annotated
//if the function returns a value its type must be specified after the ->, after the parameters
//you don't have to use the keyword return, if it is an expressions
//expressions don't require semicolons, if a semi colon was added it would break the code
//it would become a statement and therefore not return a value which means the function call wouldn't work
fn increment(x: i32) -> i32 {
    let y = x;
    y + 1
}

fn main() {
    let x = increment(1);

    println!("The value of x is: {}", x);
}
