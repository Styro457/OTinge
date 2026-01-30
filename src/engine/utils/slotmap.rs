use std::ops::{Index, IndexMut};

pub struct SlotMap<T> {
    data: Vec<T>,
    free_slots: Vec<usize>
}

impl<T> SlotMap<T> {
    pub fn new() -> SlotMap<T> {
        SlotMap {
            data: Vec::new(),
            free_slots: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> usize {
        let free_index = self.free_slots.pop();

        if(free_index.is_none()) {
            self.data.push(value);
            self.data.len() - 1
        }
        else {
            self.data[free_index.unwrap()] = value;
            free_index.unwrap()
        }
    }

    pub fn remove(&mut self, index: usize) {
        self.free_slots.push(index);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }

    // TODO: implement iteration
}

impl<T> Index<usize> for SlotMap<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for SlotMap<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}