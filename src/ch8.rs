mod task1;
mod task2;
mod task3;
mod mod_struct;

#[test]
pub fn test(){
    mod_struct::test1();
    task1::test1();
    println!("{}", task2::pig_latin("filter"));
    task3::test3();
}