#![no_std]

use heapless::Vec;

#[derive(Debug)]
pub enum SchedulerError {
    LateStart,
    ScheduleFull,
}

impl core::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::error::Error for SchedulerError {}

pub trait Task {
    fn run(&mut self, now: i64);
    fn start_time(&self) -> i64;
    fn duration(&self) -> i64;
    fn is_running(&self, now: i64) -> bool;
}

struct TaskEntry<'a> {
    start_time: i64,
    task: &'a mut dyn Task,
}

impl<'a> TaskEntry<'a> {
    fn new(start_time: i64, task: &'a mut dyn Task) -> Self {
        Self { start_time, task }
    }
}

#[derive(Default)]
pub struct Scheduler<'a, const NQUEUE: usize> {
    tasks: Vec<TaskEntry<'a>, NQUEUE>,
    now: i64,
    running: Option<&'a dyn Task>,
}

impl<'a, const NQUEUE: usize> Scheduler<'a, NQUEUE> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_task(&mut self, t: &'a mut dyn Task) ->
        Result<(), SchedulerError>
    {
        let start_time = t.start_time();
        if start_time <= self.now {
            return Err(SchedulerError::LateStart);
        }
        if self.tasks.is_full() {
            return Err(SchedulerError::ScheduleFull);
        }
        self.tasks.push(TaskEntry::new(start_time, t))
            .map_err(|_| "task queue full")
            .unwrap();
        Ok(())
    }

    pub fn tick(&mut self) {
        self.now += 1;

        if let Some(t) = self.running {
            if !t.is_running(self.now) {
                self.running = None;
            }
        }

        if self.running.is_none() {
            let next_task = self.tasks
                .iter()
                .enumerate()
                .min_by_key(|(_, t)| {
                    (t.start_time, t.task.duration())
                });

            if let Some((i, t)) = next_task {
                if t.start_time <= self.now {
                    let t = self.tasks.swap_remove(i);
                    t.task.run(self.now);
                    self.running = Some(t.task);
                }
            }
        }
    }

    pub fn now(&self) -> i64 {
        self.now
    }
}
