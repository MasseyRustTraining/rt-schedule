use rt_schedule::*;

struct DemoTask<T> {
    start_time: i64,
    duration: i64,
    runner: T,
}

fn new_task(
    start_time: i64,
    duration: i64,
    id: usize,
) -> DemoTask<impl FnMut(i64)> {
    let runner = move |now| {
        let ticks = now - start_time;
        if ticks < duration {
            println!("task {}: tick {}", id, ticks);
        } else {
            println!("task {}: tick overrun {} ({})", id, ticks, duration);
        }
    };
    DemoTask { start_time, duration, runner }
}

impl<T: FnMut(i64)> Task for DemoTask<T> {
    fn run(&mut self, now: i64) {
        (self.runner)(now);
    }

    fn start_time(&self) -> i64 {
        self.start_time
    }

    fn duration(&self) -> i64 {
        self.duration
    }

    fn is_running(&self, now: i64) -> bool {
        now - self.start_time < self.duration
    }
}

fn main() {
    let mut t0 = new_task(1, 2, 0);
    let mut t1 = new_task(1, 1, 1);
    let mut t2 = new_task(4, 1, 2);
    let tasks: [&mut dyn Task; 3] = [&mut t0, &mut t1, &mut t2];
    let mut scheduler: Scheduler<3> = Scheduler::new();
    for t in tasks {
        scheduler.add_task(t).unwrap();
    }
    for i in 0..7 {
        println!("tick {}", i);
        scheduler.tick().unwrap();
    }
}
