fn gdaymate(word: &str) -> String{
    format!("G'day{}{}", "\t", word)
}

fn main() {
    let greeting = gdaymate("mate");
    println!("{}", greeting);
}
