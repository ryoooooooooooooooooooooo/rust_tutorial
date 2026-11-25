fn return_something1() -> (u32){
    return 114514;    
}

fn return_something2() -> (u32){
    return 810;
}
fn main() {
    let a = return_something1();
    let b = return_something2();
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
}
