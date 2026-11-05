#[derive(Debug)]
struct Foo {}

fn main() {
    let mut vector = vec![Foo {}, Foo {}, Foo {}];
    let last_foo = vector.iter_mut();

    vector.pop();
    // println!("last foo {:?}", last_foo);
}
