// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    println!("blue");
    string("red".to_string()); // Calls `string` with `String`
    string(String::from("hi")); // Calls `string` with `String`
    string("rust is fun!".to_owned()); // Calls `string` with `String`
    string("nice weather".into()); // Calls `string` with `String`
    string(format!("Interpolation {}", "Station")); // Calls `string` with `String`

    string_slice(&String::from("abc")[0..1]); // Calls `string_slice` with `&str`
    string_slice("  hello there ".trim()); // Calls `string_slice` with `&str`
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // Calls `string_slice` with `&str`
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // Calls `string_slice` with `&str`
}
