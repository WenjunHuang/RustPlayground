//@formatter:off
macro_rules! times_five {
    ($e:expr) => {5 * $e};
}

macro_rules! multiply_add {
    ($a:expr, $b:expr,$c:expr) => {$a * ($b + $c)};
}

macro_rules! vec_strs {
    ($($element:expr),*) => {
        {
            let mut v = Vec::new();
            $(v.push(format!("{}",$element));)*
            v
        }
    };
}
//@formatter:on

fn main() {
    let value = times_five!(2 * 3 + 4);
    assert_eq!(50, value);

    let value = multiply_add!(2,3,4);
    assert_eq!(14, value);

    let v = vec_strs!(1,2,3,4,5);
    println!("{:?}",v);
}