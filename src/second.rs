/** Notes
* A type has a destructor if it implements a trait called Drop.
*/

pub struct List<T> {
    head: Link<T>,
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

// Link in the first.rs is an enum exactly the same as the Options type,
// it either retuns a value or nothing
type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

// a tuple struct
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

// implement the iterator trait
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access tuple struct and pop the value
        self.0.pop()
    }
}

// static method to create a new list
impl<T> List<T> {
    pub fn new() -> Self {
        // a new list is an empty list
        List { head: None }
    }
    // push will mutate the list
    pub fn push(&mut self, elem: T) {
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
    pub fn pop(&mut self) -> Option<T> {
        // match option { None => None, Some(x) => Some(y) } is the same as option.map(|x| y)
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
        // result
    }
    // returns a pointer
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
    // mutable peek function
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
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
    #[test]
    fn peek() {
        use super::List;
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&3));
        list.pop();
        assert_eq!(list.peek(), Some(&2));

        // test mutable peek
        // &mut value does not make the value mutable, map is in fact passing the &mut i32 to the closure
        // so there is no need to use &mut value
        list.peek_mut().map(|value| {
            *value = 42;
        });
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        use super::List;
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
