mod audio_player;
mod sound;
mod workout_plan;

use crate::workout_plan::WorkoutPlan;
use std::thread;
use std::time::Duration;

#[inline(always)]
fn print_separator() {
    println!("-------------------------------------")
}

#[inline(always)]
fn sleep_seconds(seconds: u32) {
    match seconds > 15 {
        true => {
            let interval = 10;
            let seconds = seconds - interval;
            thread::sleep(Duration::from_secs(seconds as u64));
            // audio_player::bell();
            println!("Prepare. The next series starts in {}s", interval);
            thread::sleep(Duration::from_secs(interval as u64));
        }
        false => thread::sleep(Duration::from_secs(seconds as u64)),
    };
}

fn main() {
    let workout_plan = WorkoutPlan::from_stdin();

    print_separator();
    println!("Workout plan:");
    println!("{}", workout_plan);
    print_separator();

    workout_plan.start_session();
}
