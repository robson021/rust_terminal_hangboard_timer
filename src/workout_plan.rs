use crate::{audio_player, sleep};
use std::fmt::Display;

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
        println!("Get ready, start in {}s.", start_in);
        sleep(5);

        let number_of_sets = self.number_of_sets;
        for set in 1..number_of_sets + 1 {
            println!("Set: {} of {}", set, number_of_sets);
            hang(self.hang_time, self.rest_time, self.number_of_hang_repeats);
            let is_last_set = set == number_of_sets;
            if !is_last_set {
                rest_between_sets(self.rest_time_between_sets);
            }
        }
        audio_player::finish();
        println!("Workout done!");
        sleep(2);
    }
}

impl Display for WorkoutPlan {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter,
               "Hang: {}\nRest: {}\nNumber of hang repeats: {}\nRest between sets: {}\nNumber of  sets: {}",
               self.hang_time, self.rest_time, self.number_of_hang_repeats, self.rest_time_between_sets, self.number_of_sets,
        )
    }
}

#[inline(always)]
fn countdown_hang(time: u32) {
    println!("Hang for {}s", time);
    audio_player::ding();
    for n in (1..time + 1).rev() {
        println!("{}...", n);
        sleep(1);
    }
    println!("Stop hanging!");
    audio_player::bell();
}

#[inline(always)]
fn countdown_rest(time: u32) {
    println!("Rest for: {}s", time);
    sleep(time);
}

fn hang(hang_time: u32, rest_time: u32, number_of_hang_repeats: u32) {
    for _ in 0..number_of_hang_repeats - 1 {
        countdown_hang(hang_time);
        countdown_rest(rest_time);
    }
    countdown_hang(hang_time);
}

fn rest_between_sets(rest_time: u32) {
    println!("Rest time before next set: {}s", rest_time);
    sleep(rest_time);
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
