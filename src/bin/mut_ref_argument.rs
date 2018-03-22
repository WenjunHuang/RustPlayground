
fn mut_borrow(str:&mut String)->usize{
    str.len()
}

fn main(){
    let mut s = "hello world".to_owned();
    println!("{}",mut_borrow(&mut s));

    let mut_b = &mut s;
    let imut_b = &s;
}