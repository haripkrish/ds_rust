use std::fmt;
use std::mem::swap;
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
        self.bubble_up();
    }

    pub fn pop(&mut self) {
        let last_ele = match self.heap.pop() {
            Some(ele) => ele,
            None => {
                println!("The heap is empty.");
                return;
            }
        };
        self.heap[0] = last_ele;
        self.bubble_down();
    }

    pub fn swap_ele(&mut self, pos_1: usize, pos_2: usize) {
        let temp_ele = self.heap[pos_1];
        self.heap[pos_1] = self.heap[pos_2];
        self.heap[pos_2] = temp_ele;
    }

    pub fn bubble_up(&mut self) {
        let mut current_index = self.heap.len() - 1;
        while current_index > 0 {
            let parent_index: usize = (current_index - 1) / 2;
            if self.heap[parent_index] > self.heap[current_index] {
                self.swap_ele(parent_index, current_index);
                current_index = parent_index;
            } else {
                break;
            }
        }
    }

    pub fn bubble_down(&mut self) {
        let mut current_index = 0;
        let heap_length = self.heap.len() - 1;

        loop {

            let mut left_child_index = (2 * current_index) + 1;
            let mut right_child_index = (2 * current_index) + 2;

            let mut smallest_index = current_index;

            if (left_child_index <= heap_length) && (self.heap[smallest_index] > self.heap[left_child_index]) {
                smallest_index = left_child_index;
            }

            if (right_child_index <= heap_length) && (self.heap[smallest_index] > self.heap[right_child_index]) {
                smallest_index = right_child_index;
            }

            if smallest_index == current_index {
                break
            }

            self.swap_ele(current_index, smallest_index);
            current_index = smallest_index;

        }

    }
    pub fn print(&self) {
        info!("Min Heap: {:?}", self);
    }

    pub fn print_level_wise(&self) {
        let heap_len = self.heap.len();
        let mut level = 0;

        while (2_usize.pow(level) - 1) < heap_len {
            let start_index = (2_usize.pow(level) - 1);

            let end_index = std::cmp::min(2_usize.pow(level + 1) - 1, heap_len);

            println!("Elements in level {}: {:?}", level, &self.heap[start_index..end_index]);

            level += 1;
        }

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
    }

    #[test]
    fn test_bubble_up() {
        let mut min_heap = MinHeap::new();
        min_heap.push(3);
        min_heap.push(2);
        min_heap.push(1);
        assert_eq!(min_heap.to_string(), "MinHeap { heap: [1, 3, 2] }");
    }

    #[test]
    fn test_bubble_down() {
        let mut min_heap = MinHeap::new();
        min_heap.push(1);
        min_heap.push(9);
        min_heap.push(13);
        min_heap.push(2);
        assert_eq!(min_heap.to_string(), "MinHeap { heap: [1, 2, 13, 9] }");
        min_heap.pop();
        assert_eq!(min_heap.to_string(), "MinHeap { heap: [2, 9, 13] }");
    }
}

pub fn min_heap() {
    let mut min_heap: MinHeap = MinHeap::new();

    info!("Min Heap implementation");
    info!("Before Inserting");
    min_heap.print();
    for i in [7, 1, 3, 10, 5, 2, 8, 6, 4, 9] {
        min_heap.push(i);
    }
    info!("After Inserting");
    min_heap.print();
    min_heap.pop();
    info!("After deleting the min val");
    min_heap.print();
    info!("Min Heap implementation");
    min_heap.print_level_wise();
}
