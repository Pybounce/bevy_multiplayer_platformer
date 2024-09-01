use std::collections::VecDeque;


pub struct LimitedStack<T> {
    items: VecDeque<T>,
    max_size: usize
}

impl<T> LimitedStack<T> {

    pub fn new(max_size: usize) -> Self {
        Self {
            items: VecDeque::with_capacity(max_size),
            max_size
        }
    }

    pub fn push(&mut self, item: T) {
        if self.items.len() == self.max_size {
            self.items.pop_back();
        }
        self.items.push_front(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }
    
}