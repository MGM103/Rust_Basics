//Loop is an infinite loop that is used with break
//can retrun values by adding them after the break keyword
//while loop is the same as any other language, no () for condition
//for operates a bit different, always e in range format, as seen below
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
