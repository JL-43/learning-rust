fn passing_a_reference(&a_string): String {
    println!("{a_string}");
    &a_string
}

fn main() {
    println!("Hello, world!");
    passing_a_reference("fuckity fuck");
}
