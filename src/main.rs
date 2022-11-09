
//Functions
fn main() {
    variables();
    expressions();
    conditionals();
    loops();
}
    fn variables(){
    //Unmutable Variables
    let x = 27;
    println! ("The value of x is: {x}");
    let name = "Emma";
    let lastname = "Ward";
    let major = "Software Engineering";
    println! ("My name is {name} {lastname}.");
    println! ("I am a {major} major.");
    
    //Mutable Variables
    let mut a = 38;
    println! ("The value of a is: {a}");
    a = 65;
    println! ("The value of a is: {a}");
    let mut c = a - 20 + x;
    println! ("The value of c is: {c}");
    }

    fn expressions(){
    //Expressions
    let b = 2;
    //Binds x so that it cannot be changed
    b;
    println! ("The value of b is: {b}");
    }

    fn conditionals(){
    //Conditionals
    let y = 75;
    if y < 0 {
        println! ("{y} is negative");
    } else if y > 0 {
        println! ("{y} is positive");
    } else {
        println! ("{y} is zero");
    }
    }

    fn loops(){
    //Loops
    for n in 1..31{
        println! ("n is {n}");
    }
    }


