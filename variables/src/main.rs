fn main() {
    // step 0 
    let x = 5;
    // step 1
    let x = x + 1;
    // step 1 is shadowed by what's inside here
    {   // step 2
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // back to step 1 val
    println!("The value of x is: {x}");
}