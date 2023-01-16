use std::rc::Rc;
pub struct List<T> {
    head: Link<T>,
}
type Link<T> = Option<Rc<Node<T>>>;
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

/************
 * ITERATORS
 ************/

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

///////////////////////////////

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }
    pub fn tail(&self) -> List<T> {
        // map expects us to return a Y, but here we're returning an Option<Y>.
        // Thankfully, this is another common Option pattern,
        // and we can just use and_then to let us return an Option
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
    //  returns a reference to the first element
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let list = List::new().prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));
        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}
