fn main(){
    let mut x = 0;
    let v = loop{
        x += 1;
        if x == 20{
            break "found the 20";
        }
    };
    println!("from loop: {}", v);
}