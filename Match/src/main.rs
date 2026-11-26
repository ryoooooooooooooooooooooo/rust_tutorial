fn main() {
    let x = 42;

    match x {
        0 => {
            println!("found zero");
        }
        1 | 2 =>{
            println!("found one or two");
        }
        3..=9 =>{
            println!("found number between 3 and 9");
        }

    }
}
