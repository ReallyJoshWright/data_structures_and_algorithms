use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data,
            next: None,
        }
    }
}

struct List<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Display> List<T> {
    fn new() -> List<T> {
        List {
            head: None,
            tail: None,
        }
    }

    fn append(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node::new(data)));

        if self.head.is_none() {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(Rc::clone(&new_node));
        } else {
            if let Some(tail) = self.tail.as_ref() {
                tail.borrow_mut().next = Some(Rc::clone(&new_node));
            }
            self.tail = Some(Rc::clone(&new_node));
        }
    }

    fn print(&self) {
        let mut current = self.head.as_ref().map(Rc::clone);

        while let Some(node) = current {
            println!("Data: {}", node.borrow().data);
            current = node.borrow().next.as_ref().map(Rc::clone);
        }
    }
}

fn main() {
    let mut list: List<i32> = List::new();
    list.append(23);
    list.append(47);
    list.append(98);
    list.print();
}
