/** Notes
* A type has a destructor if it implements a trait called Drop.
*/

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
            next: self.head.take(),
            // mem::replace(&mut option, None) is the same as option.take()
        };

        self.head = Some(Box::new(new_node));
    }
    // pop the node, return the value, return can be empty
    pub fn pop(&mut self) -> Option<i32> {
        // match option { None => None, Some(x) => Some(y) } is the same as option.map(|x| y)
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
        // result
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
