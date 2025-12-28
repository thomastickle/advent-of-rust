use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

pub trait SleighTask: Send {
    fn describe(&self) -> String;
}

pub struct SantaSleighQueue {
    records: Mutex<VecDeque<Box<dyn SleighTask>>>, // 1. Should store the tasks
}

impl SantaSleighQueue {
    // 2. Define the `new` constructor
    pub fn new() -> Self {
        Self {
            records: Mutex::new(VecDeque::new()),
        }
    }

    // 3. Define the `enqueue` method
    pub fn enqueue(&self, task: Box<dyn SleighTask>) {
        self.records.lock().unwrap().push_back(task);
    }

    // 4. Define the `get_task` method
    pub fn get_task(&self) -> Option<Box<dyn SleighTask>> {
        self.records.lock().unwrap().pop_front()
    }
}

pub struct ElfTask {
    // 5. Define the fields
    pub name: String,
    pub urgency: u32,
}

impl ElfTask {
    // 6. Define the `new` constructor
    pub fn new(name: &str, urgency: u32) -> ElfTask {
        ElfTask {
            name: name.to_string(),
            urgency,
        }
    }
}

impl SleighTask for ElfTask {
    fn describe(&self) -> String {
        format!("Elf task: {} (urgency {})", self.name, self.urgency)
    }
}

pub struct ReindeerTask {
    // 7. Define the fields
    pub name: String,
    pub weight: u32,
}

impl ReindeerTask {
    // 8. Define the `new` constructor
    pub fn new(name: &str, weight: u32) -> ReindeerTask {
        ReindeerTask {
            name: name.to_string(),
            weight,
        }
    }
}

impl SleighTask for ReindeerTask {
    fn describe(&self) -> String {
        format!("Reindeer task: {} ({} kg)", self.name, self.weight)
    }
}

pub fn main() {
    let queue = Arc::new(SantaSleighQueue::new());
    let producer_queue = Arc::clone(&queue);
    let producer = thread::spawn(move || {
        producer_queue.enqueue(Box::new(ReindeerTask::new("Deliver Toys", 100)));
        producer_queue.enqueue(Box::new(ElfTask::new("Wrap Gifts", 3)));
        producer_queue.enqueue(Box::new(ReindeerTask::new("Collect Reindeer Feed", 50)));
        producer_queue.enqueue(Box::new(ElfTask::new("Decorate Tree", 7)));
    });

    thread::sleep(std::time::Duration::from_millis(10));

    let consumer_queue = Arc::clone(&queue);
    let consumer = thread::spawn(move || {
        loop {
            if let Some(task) = consumer_queue.get_task() {
                println!("{}", task.describe());
            } else {
                break;
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_task_new_and_describe() {
        let task = ElfTask::new("Wrap Gifts", 5);
        assert_eq!(task.name, "Wrap Gifts");
        assert_eq!(task.urgency, 5);
        assert_eq!(task.describe(), "Elf task: Wrap Gifts (urgency 5)");
    }

    #[test]
    fn test_reindeer_task_new_and_describe() {
        let task = ReindeerTask::new("Deliver Toys", 100);
        assert_eq!(task.name, "Deliver Toys");
        assert_eq!(task.weight, 100);
        assert_eq!(task.describe(), "Reindeer task: Deliver Toys (100 kg)");
    }

    #[test]
    fn test_sleigh_queue_new() {
        let queue = SantaSleighQueue::new();
        assert!(queue.get_task().is_none());
    }

    #[test]
    fn test_sleigh_queue_enqueue_and_get() {
        let queue = SantaSleighQueue::new();
        queue.enqueue(Box::new(ElfTask::new("Task 1", 1)));
        queue.enqueue(Box::new(ReindeerTask::new("Task 2", 2)));

        let task1 = queue.get_task().unwrap();
        assert_eq!(task1.describe(), "Elf task: Task 1 (urgency 1)");

        let task2 = queue.get_task().unwrap();
        assert_eq!(task2.describe(), "Reindeer task: Task 2 (2 kg)");

        assert!(queue.get_task().is_none());
    }

    #[test]
    fn test_sleigh_queue_thread_safety() {
        let queue = Arc::new(SantaSleighQueue::new());
        let mut handles = vec![];

        // Producers
        for i in 0..10 {
            let q = Arc::clone(&queue);
            handles.push(thread::spawn(move || {
                q.enqueue(Box::new(ElfTask::new(&format!("Task {}", i), i)));
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Consumers
        let mut count = 0;
        while queue.get_task().is_some() {
            count += 1;
        }

        assert_eq!(count, 10);
    }
}
