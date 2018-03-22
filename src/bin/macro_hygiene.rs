macro_rules! using_a {
($a:ident,$e:expr) => {
    {
        let $a = 42;
        $e
    }
}
}

fn main(){
    let a = 10;
    let four = using_a!(a,a / 10);
    println!("{}",four);
}