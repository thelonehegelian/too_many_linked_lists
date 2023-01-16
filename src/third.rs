use std::rc::Rc;
pub struct List {
    head: Link,
}
type Link = Option<Rc<Node>>;
pub struct Node {
    elem: i32,
    next: Link,
}
