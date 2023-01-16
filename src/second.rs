/** Notes
* A type has a destructor if it implements a trait called Drop.
*/
use std::mem;

pub struct List {
    head: Link,
}
// Link in the first.rs is an enum exactly the same as the Options type,
// it either retuns a value or nothing
type Link = Option<Box<Node>>;

pub struct Node {
    elem: i32,
    next: Link,
}

// static method to create a new list
impl List {
    pub fn new() -> Self {
        // a new list is an empty list
        List { head: None }
    }
    // push will mutate the list
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            // This function takes a mutable reference to the head
            // and returns the current head, replacing it with Link::Empty.
            next: mem::replace(&mut self.head, None),
        };

        self.head = Some(Box::new(new_node));
    }
    // pop the node, return the value, return can be empty
    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match mem::replace(&mut self.head, None) {
            None => {
                result = None;
            }
            Some(node) => {
                // remove the head
                result = Some(node.elem);
                // set head to the next node
                self.head = node.next;
            }
        }
        result
    }
}
