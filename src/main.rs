use std::time::Instant;

mod day_eight;
mod day_eleven;
mod day_fifteen;
mod day_five;
mod day_four;
mod day_fourteen;
mod day_nine;
mod day_one;
mod day_seven;
mod day_six;
mod day_ten;
mod day_thirteen;
mod day_three;
mod day_twelve;
mod day_two;

mod read_input;

fn main() {
    println!("Advent of Code - 2021");
    let now = Instant::now();

    // day_one::day_one_main();
    // day_two::day_two_main();
    // day_three::day_three_main();
    // day_four::day_four_main();
    // day_five::day_five_main();
    // day_six::day_six_main();
    // day_seven::day_six_main();
    // day_eight::day_eight_main();
    // day_nine::day_nine_main();
    // day_ten::day_ten_main();
    // day_eleven::day_eleven_main();
    // day_twelve::day_twelve_main();
    // day_thirteen::day_thirteen_main();
    // day_fourteen::day_fourteen_main();
    day_fifteen::day_fifteen_main();

    println!("\nTotal Execution time: {}ms", now.elapsed().as_millis());
}
