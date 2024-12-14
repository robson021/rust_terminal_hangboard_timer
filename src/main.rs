#[inline(always)]
fn print_separator() {
    println!("-------------------------------------")
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

fn hang(hang_time: u32, rest_time: u32, number_of_hang_repeats: u32) {
    println!("Hang!");
    // todo!()
}

fn rest_between_sets(rest_time: u32) {
    println!("Rest time before next set: {}s", rest_time);
    // todo!()
}

fn main() {
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

    print_separator();
    println!("Workout setup:");
    println!("Hang: {:#?}", hang_time);
    println!("Rest: {:#?}", rest_time);
    println!("Number of hang repeats: {:#?}", number_of_hang_repeats);
    println!("Rest between sets: {:#?}", rest_time_between_sets);
    println!("Number of  sets: {:#?}", number_of_sets);
    print_separator();

    for set in 1..number_of_sets + 1 {
        println!("Set: {} of {}", set, number_of_sets);
        hang(hang_time, rest_time, number_of_hang_repeats);
        let is_last_set = set == number_of_sets;
        if !is_last_set {
            rest_between_sets(rest_time_between_sets);
        }
    }
}
