use rt_schedule::*;

struct DemoTask {
    id: usize,
    start_time: i64,
    run_time: Option<i64>,
    duration: i64,
}

impl DemoTask {
    fn new ( start_time: i64, duration: i64, id: usize, ) -> Self {
        Self { id, start_time, run_time: None, duration }
    }

    fn run(&mut self, run_time: i64) {
        self.run_time = Some(run_time);
        if run_time > self.start_time {
            println!(
                "task {}: starting late ({} < {})",
                self.id,
                self.start_time,
                run_time,
            );
        }
        println!(
            "task {}: start {}, duration {}",
            self.id,
            run_time,
            self.duration,
        );
    }        
}

impl Task for DemoTask {
    fn run(&mut self, now: i64) {
        self.run(now);
    }

    fn start_time(&self) -> i64 {
        self.start_time
    }

    fn duration(&self) -> i64 {
        self.duration
    }

    fn is_running(&self, now: i64) -> bool {
        if let Some(run_time) = self.run_time {
            now - run_time < self.duration
        } else {
            panic!("is_running on not-yet-started task")
        }
    }
}

fn main() {
    let mut tasks = [
        DemoTask::new(1, 3, 0),
        DemoTask::new(1, 3, 1),
        DemoTask::new(4, 1, 2),
        DemoTask::new(9, 1, 3),
    ];
    let mut scheduler: Scheduler<4> = Scheduler::new();
    for t in &mut tasks {
        scheduler.add_task(t).unwrap();
    }
    for i in 0..10 {
        println!("tick {}", i);
        scheduler.tick();
    }
}
