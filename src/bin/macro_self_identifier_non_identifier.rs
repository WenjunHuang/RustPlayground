macro_rules! what_is {
(self) => {"the keyword 'self'"};
($i:ident) => {concat!("the identifier '",stringify!($i),"'")};
}

macro_rules! call_with_ident {
($c:ident($i:ident)) => {$c!($i)};
}

macro_rules! double_method {
($self_:ident,$body:expr)=>{
fn double(mut $self_) -> Dummy {
$body
}
};
}

struct Dummy(i32);
impl Dummy{
    double_method!{self,{
        self.0 *= 2;
        self
    }}
//    double_method!{_,0}
}
fn main(){
    println!(what_is!(self));
    println!(call_with_ident!(what_is(self)));

}