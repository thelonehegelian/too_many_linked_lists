use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>), // Box is a smart pointer that allocates memory on the heap
}

pub struct Node {
    elem: i32,
    next: Link,
}

// static method to create a new list
impl List {
    pub fn new() -> Self {
        // a new list is an empty list
        List { head: Link::Empty }
    }
    // push will mutate the list
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            // This function takes a mutable reference to the head
            // and returns the current head, replacing it with Link::Empty.
            next: mem::replace(&mut self.head, Link::Empty),
        };

        self.head = Link::More(Box::new(new_node));
    }
    // pop the node, return the value, return can be empty
    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => {
                result = None;
            }
            Link::More(node) => {
                // remove the head
                result = Some(node.elem);
                // set head to the next node
                self.head = node.next;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        use super::List;
        // create a new list
        let mut list = List::new();
        // assert that the list is empty
        assert_eq!(list.pop(), None);
        // push new values
        list.push(1);
        list.push(2);
        list.push(3);

        // pop the values
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        // push more values
        list.push(4);
        list.push(5);
        // pop the values
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        // remove all the values
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
