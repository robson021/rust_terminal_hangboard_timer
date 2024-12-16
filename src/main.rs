mod audio_player;
mod sound;
mod workout_plan;

use crate::workout_plan::WorkoutPlan;
use std::thread;

#[inline(always)]
fn print_separator() {
    println!("-------------------------------------")
}

#[inline(always)]
fn sleep_seconds(seconds: u32) {
    thread::sleep(std::time::Duration::from_secs(seconds as u64))
}

fn main() {
    let workout_plan = WorkoutPlan::from_stdin();

    print_separator();
    println!("Workout plan:");
    println!("{}", workout_plan);
    print_separator();

    workout_plan.start_session();
}
