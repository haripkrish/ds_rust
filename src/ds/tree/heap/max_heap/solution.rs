#[derive(Debug)]
pub struct MaxHeap {
    pub heap: Vec<i32>,
}

impl MaxHeap {
    pub fn new() -> Self {
        MaxHeap {
            heap: Vec::new()
        }
    }

    fn print_heap(&self) {
        println!("Max Heap: {:?}", self.heap);
    }

    fn swap_ele(&mut self, ele_1: usize, ele_2: usize) {
        let temp = self.heap[ele_1];
        self.heap[ele_1] = self.heap[ele_2];
        self.heap[ele_2] = temp;
    }

    fn push(&mut self, ele: i32) {
        self.heap.push(ele);
        let mut current_index = self.heap.len() - 1;

        while current_index > 0 {
            let mut parent_index = (current_index - 1) / 2;
            if self.heap[current_index] > self.heap[parent_index] {
                self.swap_ele(current_index, parent_index);
                current_index = parent_index;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.heap.is_empty() {
            println!("Heap is empty!");
            return None;
        }
        let max_value = self.heap[0]; // The root is the max value

        let last_element = self.heap.pop();

        if let Some(last) = last_element {
            if !self.heap.is_empty() {
                self.heap[0] = last; // Move the last element to the root
                self.bubble_down(0); // Restore the max-heap property
            }
        }
        Some(max_value)
    }

    fn bubble_down(&mut self, index: usize) {
        let heap_len = self.heap.len();
        let mut current_index = index;

        loop {
            let mut max_index = current_index;
            let left_child_index = (2 * current_index) + 1;
            let right_child_index = (2 * current_index) + 2;

            if left_child_index < heap_len && self.heap[left_child_index] > self.heap[max_index] {
                max_index = left_child_index
            }

            if right_child_index < heap_len && self.heap[right_child_index] > self.heap[max_index] {
                max_index = right_child_index
            }

            if current_index == max_index {
                break;
            }

            self.swap_ele(current_index, max_index);
            current_index = max_index
        }
    }
}


pub fn max_heap() {
    println!("Max heap implementation");
    let mut max_heap: MaxHeap = MaxHeap::new();
    max_heap.print_heap();
    for i in 1..10 {
        max_heap.push(i);
    }
    max_heap.print_heap();

    let Some(max) = max_heap.pop() else { todo! {} };
    while let Some(max) = max_heap.pop() {
        println!("Popped: {}", max);
        max_heap.print_heap();
    }
}