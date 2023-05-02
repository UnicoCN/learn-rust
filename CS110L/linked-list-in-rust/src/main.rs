

struct Node {
    value : u32,
    next : Option<Box<Node>>,
}

pub struct LinkedList {
    head : Option<Box<Node>>,
    size : usize,
}

impl Node {
    fn new(value : u32, next : Option<Box<Node>>) -> Node{
        Node { value: value, next: next }
    }
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList { head: None, size: 0 }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, value : u32) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop(&mut self) -> Option<u32> {
        let node = self.head.take()?; // if self.head is None, return None, else continue
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }

    pub fn display(&self) {
        let mut current = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                }, 
                None => break,
            }
        }
        println!("{}", result);
    }
}


fn main() {
    let mut list = LinkedList::new();
    for i in 1..10 {
        list.push(i);
    }
    list.display();
    list.pop();
    list.display();
    assert!(list.get_size() == 8);
}
