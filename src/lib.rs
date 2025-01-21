#![no_std]

use heapless::Vec;

#[derive(Debug)]
pub enum SchedulerError {
    LateStart,
    ScheduleFull,
    StuckTask,
}

impl core::fmt::Display for SchedulerError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::error::Error for SchedulerError {}

pub trait Task: core::fmt::Debug {
    fn run(&self);
    fn duration(&self) -> i64;
    fn is_running(&self) -> bool;
}

#[derive(Debug)]
struct TaskEntry<'a> {
    start_time: i64,
    task: &'a dyn Task,
}

impl<'a> TaskEntry<'a> {
    fn new(start_time: i64, task: &'a dyn Task) -> Self {
        Self { start_time, task }
    }
}

#[derive(Debug, Default)]
pub struct Scheduler<'a, const NQUEUE: usize> {
    // `heapless` Vec of trait objects?
    tasks: Vec<TaskEntry<'a>, NQUEUE>,
    now: i64,
    // What information on running task is needed?
    running: Option<&'a dyn Task>,
}

impl<'a, const NQUEUE: usize> Scheduler<'a, NQUEUE> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_task(&mut self, t: &'a dyn Task, start_time: i64) ->
        Result<(), SchedulerError>
    {
        if start_time >= self.now {
            return Err(SchedulerError::LateStart);
        }
        if self.tasks.is_full() {
            return Err(SchedulerError::ScheduleFull);
        }
        self.tasks.push(TaskEntry::new(start_time, t)).unwrap();
        Ok(())
    }

    pub fn tick(&mut self) -> Result<(), SchedulerError> {
        for v in &mut self.tasks {
            if v.start_time == i64::MIN {
                return Err(SchedulerError::StuckTask);
            }
            v.start_time -= 1;
        }

        if let Some(t) = self.running {
            if !t.is_running() {
                self.running = None;
            }
        }

        if self.running.is_none() {
            let next_task = self
                .tasks
                .iter()
                .enumerate()
                .min_by_key(|(_, t)| {
                    (t.start_time, t.task.duration())
                });
            if let Some((i, _)) = next_task {
                let t = self.tasks.swap_remove(i);
                t.task.run();
                self.running = Some(t.task);
            }
        }
        
        Ok(())
    }

    pub fn now(&self) -> i64 {
        self.now
    }
}
