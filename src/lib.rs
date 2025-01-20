#![no_std]

pub trait Task {
    fn run(&self);
    fn duration(&self) -> u64;
}

pub struct Scheduler {
    tasks: ???,
    now: u64,
    running: Option<???>,
}

impl Scheduler {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_task(t: ???) -> ???{
        todo!()
    }

    pub fn tick() {
    }
}
