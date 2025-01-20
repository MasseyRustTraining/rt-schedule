#![no_std]

pub trait Task {
    fn run(&self);
    fn duration(&self) -> u64;
}

pub struct Scheduler {
    // `heapless` Vec of trait objects?
    tasks: ???,
    now: u64,
    // What information on running task is needed?
    running: Option<???>,
}

impl Scheduler {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_task(t: ???) -> ??? {
        todo!()
    }

    pub fn tick() {
        todo!()
    }
}
