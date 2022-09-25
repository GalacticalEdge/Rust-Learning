// Ran my very first Rust program! Can't wait for my journey
// in Rust. Worth more than C and C++

fn main() {
    println!("Hello World"); // This will print "Hello World"
    println!("{} is after 30", 31); // There is a variable set in there that "{}" will print
    println!("{name} {is} {what}", name="Andrew", is=" is", what=" sus"); // We have set variables for this line, but we explicitly tell what we will print which variables to print

    let age = 15; // We set the variable "age" to 15
    println!("I am {} years old", age);

    print!("I am {} years old ", age); // With "print!", we are gonna be printing stuff in the same line instead
    print!("Just one more year til I get my driver's license"); // and this one will print in the same line since they both use "print!"
    println!(" "); // Add this so the line below isn't in the same one as above
    println!("Rust is fun\nI could do it all day"); // At "\n", it will cut to the next line
}