use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct IdAllocator {
    current_id: usize,
    recycled_ids: BinaryHeap<Reverse<usize>>,
}

impl IdAllocator {
    pub fn new() -> Self {
        Self {
            current_id: 0,
            recycled_ids: BinaryHeap::new(),
        }
    }

    pub fn allocate(&mut self) -> usize {
        match self.recycled_ids.pop() {
            Some(Reverse(id)) => id,
            None => {
                let id = self.current_id;
                self.current_id += 1;
                id
            }
        }
    }

    pub fn free(&mut self, id: usize) {
        self.recycled_ids.push(Reverse(id));
    }
}