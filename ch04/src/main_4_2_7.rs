fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

fn not_dangle() -> String {
    let s = String::from("hello");
    s
}