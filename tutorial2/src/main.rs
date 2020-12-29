#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let i = 1;
    println!("Hello, world! {}", i);

    let mystr = String::from("hi");
    println!("Hello, world! {}", mystr);

    let p1 = Person {
        name: "Sim Kern Cheh".to_string(),
        age: 18,
    };
    println!("{:?}", p1);
}
