use std::{
    fs::File,
    io::{self, BufRead},
};

fn parse_input(input_file: File) -> Vec<i16> {
    let mut inp_vec: Vec<i16> = Vec::new();

    let reader = io::BufReader::new(input_file);

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        // println!("line is {}", line);
        let direction = line.chars().nth(0);
        let mut value: i16 = (&line[1..])
            .parse()
            .expect("could not convert value to i16");
        if direction == Some('L') {
            value *= -1;
        }
        // println!("direction is {:?}, value is {}", direction, value);
        inp_vec.push(value);
    }
    inp_vec
}

fn count_zero_crossings(val: i16) -> (i16, i16) {
    /// Counts number of zer crossings for the given value, returning
    /// (num_zero_crossing, remainder)
    let rem: i16 = (val % 100);
    let zc: i16 = (val - rem) / 100;
    println!("processed value:{}, got zc:{}, rem:{}", val, zc, rem);
    (zc.abs(), rem)
}

fn main() {
    println!("Hello, world!");
    let filepath = "./inp.txt";
    let input_file = File::open(filepath).expect("failed to open file");
    let parsed_input = parse_input(input_file);
    // println!("parsed input: {:?}", parsed_input);

    let mut zero_crossings: i16 = 0;
    let mut starting_value: i16 = 50;
    let mut num_zeroes: i16 = 0;
    for item in parsed_input.iter() {
        println!("processing sv: {}, item: {}", starting_value, item);
        if starting_value + item >= 100 {
            println!(
                "moved past 100 with starting_value: {}, item: {}, zc's so far: {}",
                starting_value, item, zero_crossings
            );
            let diff_from_0: i16 = item - (100 - starting_value);
            let (zc, rem) = count_zero_crossings(diff_from_0);
            zero_crossings += zc + 1;
            starting_value = rem;
        } else if starting_value + item <= 0 {
            println!(
                "moved past 0 with starting_value: {}, item: {}, zc's so far: {}",
                starting_value, item, zero_crossings
            );
            let diff_from_0: i16 = item + starting_value;
            let (zc, rem) = count_zero_crossings(diff_from_0);
            zero_crossings += zc + 1;
            starting_value = 100 + rem;
            if starting_value == 100 {
                starting_value = 0;
            }
        } else {
            starting_value += item;
        }
        println!(
            "value now: {}, added: {}, zc's: {}",
            starting_value, item, zero_crossings
        );
        if starting_value == 0 {
            num_zeroes += 1;
        }
    }
    println!(
        "got {} zero crossings, {} zeroes",
        zero_crossings, num_zeroes
    );
}
