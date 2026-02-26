mod r00_variable; // for module 
use r03_basics::greet; // for libarary
fn main() {
    r00_variable::variable();
    greet();
    println!("Hello, world!");
}
