use rt_schedule::*;

struct DemoTask<T> {
    start_time: i64,
    run_time: i64,
    duration: i64,
    runner: T,
}

fn new_task(
    start_time: i64,
    duration: i64,
    id: usize,
) -> DemoTask<impl FnMut(i64)> {
    let runner = move |run_time| {
        println!("task {}: start {}, duration {}", id, run_time, duration);
    };
    DemoTask { start_time, run_time: 0, duration, runner }
}

impl<T: FnMut(i64)> Task for DemoTask<T> {
    fn run(&mut self, now: i64) {
        self.run_time = now;
        (self.runner)(now);
    }

    fn start_time(&self) -> i64 {
        self.start_time
    }

    fn duration(&self) -> i64 {
        self.duration
    }

    fn is_running(&self, now: i64) -> bool {
        now - self.run_time < self.duration
    }
}

fn main() {
    let mut t0 = new_task(1, 3, 0);
    let mut t1 = new_task(1, 3, 1);
    let mut t2 = new_task(4, 1, 2);
    let tasks: [&mut dyn Task; 3] = [&mut t0, &mut t1, &mut t2];
    let mut scheduler: Scheduler<3> = Scheduler::new();
    for t in tasks {
        scheduler.add_task(t).unwrap();
    }
    for i in 0..10 {
        println!("tick {}", i);
        scheduler.tick();
    }
}
