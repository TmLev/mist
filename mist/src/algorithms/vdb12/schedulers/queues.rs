use std::hash::Hash;

use keyed_priority_queue::{Entry, KeyedPriorityQueue, KeyedPriorityQueueBorrowIter};

use crate::vdb12::{Application, Deadline};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait PriorityQueue<Element, Priority>
where
    Element: Hash + Eq,
    Priority: Ord,
{
    fn push(&mut self, element: Element, priority: Priority) -> Option<Priority>;
    fn pop(&mut self) -> Option<(Element, Priority)>;
    fn len(&self) -> usize;
    fn iter(&self) -> KeyedPriorityQueueBorrowIter<Element, Priority>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct EarliestDeadlineFirst {
    queue: KeyedPriorityQueue<Application, Deadline>,
}

impl EarliestDeadlineFirst {
    pub fn new() -> Self {
        Self {
            queue: KeyedPriorityQueue::new(),
        }
    }
}

impl PriorityQueue<Application, Deadline> for EarliestDeadlineFirst {
    fn push(&mut self, element: Application, priority: Deadline) -> Option<Deadline> {
        self.queue.push(element, priority)
    }

    fn pop(&mut self) -> Option<(Application, Deadline)> {
        self.queue.pop()
    }

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn iter(&self) -> KeyedPriorityQueueBorrowIter<Application, Deadline> {
        self.queue.iter()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct FirstComeFirstServed {
    queue: KeyedPriorityQueue<Application, usize>,
    counter: usize,
}

impl FirstComeFirstServed {
    pub fn new() -> Self {
        Self {
            queue: KeyedPriorityQueue::new(),
            counter: 0,
        }
    }
}

impl PriorityQueue<Application, usize> for FirstComeFirstServed {
    fn push(&mut self, element: Application, priority: usize) -> Option<usize> {
        self.counter += 1;
        self.queue.push(element, self.counter)
    }

    fn pop(&mut self) -> Option<(Application, usize)> {
        self.queue.pop()
    }

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn iter(&self) -> KeyedPriorityQueueBorrowIter<Application, usize> {
        self.queue.iter()
    }
}
