use std::fmt::Display;

fn main() {
    let mut list = List::new("1".to_string());
    list.append("2".to_string());
    list.append("3".to_string());
    println!("Without cero");
    list.print_list();

    list.insert_at_beginning("0".to_string());
    println!("With cero");
    list.print_list();
}




struct List<T: Clone> {
    value: T,
    next: Option<Box<List<T>>>,
}
impl <T> Clone for List<T> where T : Clone {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), next: self.next.clone() }
    }
}

impl <T:Clone> List<T> {
    fn new(value: T) -> Self {
        let next = None;
        List { value, next }
    }

    fn append(&mut self, value: T) {
        let new_node = List::new(value);
        let mut current = self;
        while let Some(_) = &current.next {
            current = current.next.as_mut().unwrap();
        }
        current.next = Some(Box::new(new_node));
    }

    fn insert_at_beginning(&mut self, value: T) {
        let mut moved_node = List::new(self.value.clone());
        if self.next.is_some() {
            moved_node.next = self.next.clone();
        }
        
        self.next = Some(Box::new(moved_node));
        self.value = value;
    }

    // ... otros métodos ...
}

impl <T:Clone> List<T> where T : Display{
    fn print_list(&self) {
        if let Some(ref next) = self.next {
            println!("{} -> ", self.value);
            next.print_list();
        } else {
            println!("{} ", self.value);
        }
    }
}
