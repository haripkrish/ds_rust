use std::fmt;
use log::{info};

#[derive(Debug)]
pub struct MinHeap {
    pub heap: Vec<i32>,
}

impl MinHeap {
    pub fn new() -> Self {
        MinHeap { heap: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.heap.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.heap.pop()
    }

    //pub fn bubble_up();
    pub fn print(&self) {
        info!("Min Heap: {:?}", self);
    }
}

// Implementing the Display trait for MinHeap
impl fmt::Display for MinHeap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MinHeap {{ heap: {:?} }}", self.heap)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut min_heap = MinHeap::new();
        min_heap.push(1);
        min_heap.push(2);
        assert_eq!(min_heap.to_string(), "MinHeap { heap: [1, 2] }");
        assert_eq!(min_heap.pop(), Some(2));
    }
}

pub fn min_heap() {
    let mut min_heap: MinHeap = MinHeap::new();

    info!("Min Heap implementation");
    min_heap.print();
    for i in 1..5 {
        min_heap.push(i);
    }
    min_heap.print();
    info!("Min Heap implementation");
}
