fn main() {
    let x = 114514;

    match x{
        0 => {
            println!("found zero");
        }

        1 | 2 => {
            println!("found zero or two");
        }

        3..=9 =>{
            println!("found a nunmber 3 to 9");
        }

        matched_num @ 10 ..= 100 =>{
            println!("found {} number between 10 and 100", matched_num);
        }

        _ => {
            println!("found something else!");       // else みたいなやつ
        }
    }
}
