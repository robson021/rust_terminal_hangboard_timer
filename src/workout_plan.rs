use crate::{audio_player, sleep_seconds};
use std::fmt::Display;
use std::io::Write;

pub struct WorkoutPlan {
    pub hang_time: u32,
    pub rest_time: u32,
    pub number_of_hang_repeats: u32,
    pub rest_time_between_sets: u32,
    pub number_of_sets: u32,
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

fn countdown_hang(time: u32, current_rep: u32, num_of_reps: u32) {
    println!("Hang for {time}s. Repeat {current_rep} of {num_of_reps}.");
    for n in (1..time + 1).rev() {
        print!("{n}...");
        let _ = std::io::stdout().flush();
        sleep_seconds(1);
    }
    println!("\nStop hanging!");
}

fn countdown_rest(time: u32) {
    println!("Rest for: {time}s");
    sleep_seconds(time);
}

fn hang_round(hang_time: u32, rest_time: u32, number_of_hang_repeats: u32) {
    if hang_time < 1 {
        return;
    }
    for i in 0..number_of_hang_repeats - 1 {
        audio_player::ding();
        countdown_hang(hang_time, i + 1, number_of_hang_repeats);
        audio_player::bell();
        countdown_rest(rest_time);
    }
    audio_player::ding();
    countdown_hang(hang_time, number_of_hang_repeats, number_of_hang_repeats);
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
    parse_input(&input).expect("Invalid input. A number was expected.")
}

fn parse_input(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.trim().parse()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, WorkoutPlan};

    #[test]
    fn displays_all_workout_settings() {
        let plan = WorkoutPlan {
            hang_time: 7,
            rest_time: 3,
            number_of_hang_repeats: 6,
            rest_time_between_sets: 120,
            number_of_sets: 3,
        };

        assert_eq!(
            plan.to_string(),
            "Hang: 7\nRest: 3\nNumber of hang repeats: 6\nRest between sets: 120\nNumber of sets: 3"
        );
    }

    #[test]
    fn parses_unsigned_seconds_with_surrounding_whitespace() {
        assert_eq!(parse_input(" \t42\n"), Ok(42));
    }

    #[test]
    fn rejects_non_numeric_and_negative_input() {
        assert!(parse_input("not a number").is_err());
        assert!(parse_input("-1").is_err());
    }

    #[test]
    fn rejects_values_outside_the_u32_range() {
        assert!(parse_input("4294967296").is_err());
    }
}
