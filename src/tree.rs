//! Implementation of a 2-3-4 tree.

pub struct Tree234<'a, T> {
    pub children: Vec<Node234<'a, T>>
}

enum Node234<T> {
    Node2(Node2<T>),
    Node3(Node3<T>),
    Node4(Node4<T>),
}

pub struct Node2<'a, T> {
    pub children: [&'a Node234<T>; 2],
    pub elements: T
}