use std::mem;
use std::ptr;

pub struct LinkedList<T>(Option<Box<LinkedList<T>>>,Option<(T,Box<LinkedList<T>>)>);
impl<T> LinkedList<T>{
    pub fn new() -> Self{
        LinkedList(None,None)
    }
    pub fn push_front(&mut self, data:T){
        let t = self.0.take();
        self.0 = Some((data,Box::new(LinkedList(,t))));
    }
}
fn main() {

}





