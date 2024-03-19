fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let x2 = 5;

    let x2 = x2 + 1;

    {
        let x2 = x2 * 2;
        println!("The value of x in the inner scope is: {x2}");
    }

    println!("The value of x is: {x2}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The len of spaces is: {spaces}");
}
