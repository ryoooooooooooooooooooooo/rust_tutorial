fn add(x: i32, y: i32) -> i32{
    x + y  // or just return x + y; 
}

fn subtract(x: i32, y: i32) -> i32{
    x - y 
}
fn main() {
    println!("1 + 1 = {}", add(1, 1));
    println!("1 - 1 = {}", subtract(1, 1));
}
