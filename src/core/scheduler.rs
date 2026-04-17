use std::collections::VecDeque;

pub struct Scheduler {
    queue: VecDeque<Vec<u8>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn submit(&mut self, task: Vec<u8>) {
        self.queue.push_back(task);
    }

    pub fn next(&mut self) -> Option<Vec<u8>> {
        self.queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
