fn main(){
    assert_eq!(std::mem::size_of::<i64>(),8);
    assert_eq!(std::mem::align_of::<(i32,i32)>(),4);
}