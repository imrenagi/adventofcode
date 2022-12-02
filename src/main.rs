use crate::year_2022::day01_calorie_counting::calorie_counting;

pub mod year_2022;

fn main() {
    let answer = calorie_counting("input/2022/day01_calorie_counting.txt");
    println!("{:?}", answer);
}
