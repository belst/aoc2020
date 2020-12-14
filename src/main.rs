mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

aoc_main::main! {
    year 2020;
    day1 : input_gen => solve_part1, solve_part2;
    day2 : input_gen => part1, part2;
    day3 : generate  => part1, part2;
    day4 : generate  => part1, part2;
    day5 : generate  => part1, part2;
    day6 : generate  => part1, part2;
    day7             => part1, part2;
    day8 : generate  => part1, part2;
    day9 : generate  => part1, part2;
    day10: generate  => part1, part2;
    day11: generate  => part1, part2;
    day12: generate  => part1, part2;
    day13: generate  => part1, part2;
}
