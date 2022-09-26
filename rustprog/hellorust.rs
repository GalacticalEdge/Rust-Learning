// Ran my very first Rust program! Can't wait for my journey
// in Rust. Worth more than C and C++

fn main() {
    println!("Hello World"); // This will print "Hello World"
    println!("{} is after 30", 31); // There is a variable set in there that "{}" will print
    println!("{name} {is} {what}", name="Andrew", is=" is", what=" sus"); // We have set variables for this line, but we explicitly tell what we will print which variables to print

    let age = 15;
    print!("I am {} years old ", age); // With "print!", we are gonna be printing stuff in the same line instead
    print!("Just one more year til I get my driver's license"); // and this one will print in the same line since they both use "print!"
    println!(" "); // Add this so the line below isn't in the same one as above
    println!("Rust is fun\nI could do it all day"); // At "\n", it will cut to the next line

    // Here are different variable types
    let amount = 2; // This stores an integer
    println!("I have {} monitors", amount);

    let decimal = 4.5; // This stores a decimal/float
    println!("9 divided by 2 is {}", decimal);

    let movie = "Interstellar"; // This stores a string
    println!("One of the most interesting movies ever is {}", movie);

    let mut year = 2022; // This is a value that is "mutable", which is a variable that can be changed later
    print!("This year is {}. ", year);
    year = 2023; // This will change the value to "2023"
    println!("Next year will be {}.", year);

    const PI: f32 = 3.14; // A "const" is a special kind of variable where the value cannot be changed.
    println!("The value of pi = {}", PI);
}