use crate::{audio_player, sleep_seconds};
use std::fmt::Display;
use std::io::Write;

pub struct WorkoutPlan {
    hang_time: u32,
    rest_time: u32,
    number_of_hang_repeats: u32,
    rest_time_between_sets: u32,
    number_of_sets: u32,
}

impl WorkoutPlan {
    pub fn from_stdin() -> WorkoutPlan {
        println!("Enter hang time (seconds):");
        let hang_time = read_input();

        println!("Enter reset time (seconds):");
        let rest_time = read_input();

        println!("Enter numer of hang repeats:");
        let number_of_hang_repeats = read_input();

        println!("Enter reset between sets (seconds):");
        let rest_time_between_sets = read_input();

        println!("Enter number of sets:");
        let number_of_sets = read_input();

        WorkoutPlan {
            hang_time,
            rest_time,
            number_of_hang_repeats,
            rest_time_between_sets,
            number_of_sets,
        }
    }
    pub fn start_session(&self) {
        let start_in = 5;
        println!("Get ready, start in {start_in}s.");
        sleep_seconds(5);

        let number_of_sets = self.number_of_sets;
        for set in 1..number_of_sets + 1 {
            println!("Set: {set} of {number_of_sets}");
            hang_round(self.hang_time, self.rest_time, self.number_of_hang_repeats);
            let is_last_set = set == number_of_sets;
            if !is_last_set {
                countdown_rest_between_sets(self.rest_time_between_sets);
            }
        }
        audio_player::finish();
        println!("Workout done!");
        sleep_seconds(2);
    }
}

impl Display for WorkoutPlan {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter,
               "Hang: {}\nRest: {}\nNumber of hang repeats: {}\nRest between sets: {}\nNumber of sets: {}",
               self.hang_time, self.rest_time, self.number_of_hang_repeats, self.rest_time_between_sets, self.number_of_sets,
        )
    }
}

#[inline(always)]
fn countdown_hang(time: u32) {
    println!("Hang for {time}s");
    for n in (1..time + 1).rev() {
        print!("{n}...");
        let _ = std::io::stdout().flush();
        sleep_seconds(1);
    }
    println!("\nStop hanging!");
}

#[inline(always)]
fn countdown_rest(time: u32) {
    println!("Rest for: {time}s");
    sleep_seconds(time);
}

fn hang_round(hang_time: u32, rest_time: u32, number_of_hang_repeats: u32) {
    if hang_time < 1 {
        return;
    }
    for _ in 0..number_of_hang_repeats - 1 {
        audio_player::ding();
        countdown_hang(hang_time);
        audio_player::bell();
        countdown_rest(rest_time);
    }
    audio_player::ding();
    countdown_hang(hang_time);
    audio_player::end_of_round();
}

fn countdown_rest_between_sets(rest_time: u32) {
    println!("Rest time before next set: {rest_time}s");
    sleep_seconds(rest_time);
}

fn read_input() -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can not read user input.");
    input
        .trim()
        .parse::<u32>()
        .expect("Invalid input. A number was expected.")
}
