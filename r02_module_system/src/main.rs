use r02_module_system::greet;
// use rand::Rng;
mod hello;
fn main() {
    // let mut rng=rand::thread_rng();
    // let mut _x=rng.gen_range(0..100);
    // println!("{}",_x);
    hello::hello();
    greet();
    println!("Hello, world!");
}
