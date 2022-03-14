
pub trait Domain{
    fn add(&self, a: i32, b: i32) -> i32{
        a + b
    }
}

pub struct Entity;
impl Domain for Entity {}

pub fn hello(){
    let dm = Entity{};
    dm.add(1,2);
    print!("Hello");
}
