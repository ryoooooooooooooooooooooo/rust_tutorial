fn main(){
    let mut x = 0;
    let v = loop{
        x += 1;
        if x == 13{   // no need to write ()
            break "reached 13";
        }
    };
    println!("{}", v);
}