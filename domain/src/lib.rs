
pub trait Domain{
    fn add(a: i32, b:i32) -> i32;
}

pub struct Entity;

impl Entity {
    pub fn add(a: i32, b: i32) -> i32{
        return a + b
    }
}

pub fn hello(){
    print!("Hello");
}
