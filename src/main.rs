mod model;

fn main() {
    let permissions = model::Permission::from_bits(1 << 0 | 1 << 2);
    println!("{:?}", permissions);
    println!("Hello, world!");
}
