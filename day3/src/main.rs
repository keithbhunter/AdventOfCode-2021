use input;
use std::collections::HashMap;

fn main() {
    let input = input::read_lines("input.txt");
    let (oxygen_rating, co2_rating) = calculate_oxygen_co2_ratings(&input);
    println!("{:?}", oxygen_rating * co2_rating);
}

const PART_1_EXAMPLE_INPUT: [&str; 12] = [
    "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
    "00010", "01010",
];

fn count_bytes(input: &[String]) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut zeroes: HashMap<usize, usize> = HashMap::new();
    let mut ones: HashMap<usize, usize> = HashMap::new();

    for i in 0..input.len() {
        let mut bytes = input[i].bytes();

        for j in 0..bytes.len() {
            zeroes.entry(j).or_insert(0);
            ones.entry(j).or_insert(0);
            let byte = bytes.nth(0).unwrap();
            // byte == 0
            if byte == 48 {
                zeroes.entry(j).and_modify(|e| *e += 1);
            }

            // byte == 1
            if byte == 49 {
                ones.entry(j).and_modify(|e| *e += 1);
            }
        }
    }

    (ones, zeroes)
}

fn calculate_gamma_epsilon_rate(input: &[String]) -> (usize, usize) {
    let (ones, zeroes) = count_bytes(input);
    let mut gamma: usize = 0;
    let mut mask: usize = 0;

    for i in 0..zeroes.len() {
        mask = (mask << 1) | 0b1;
        gamma = gamma << 1;

        if ones.get(&i).unwrap() > zeroes.get(&i).unwrap() {
            gamma = gamma | 0b1;
        }
    }

    // epsilon is binary opposite of gamma
    let epsilon = !gamma & mask;
    (gamma, epsilon)
}

fn calculate_oxygen_co2_ratings(input: &[String]) -> (usize, usize) {
    let mut oxygen = Vec::from(input);
    let mut carbon = Vec::from(input);

    for i in 0..input.len() {
        if oxygen.len() > 1 {
            let (ones_oxygen, zeroes_oxygen) = count_bytes(&oxygen);
            let oxygen_zero_count = zeroes_oxygen.get(&i).unwrap();
            let oxygen_one_count = ones_oxygen.get(&i).unwrap();

            let mut oxygen_filter_char = 49;
            if oxygen_zero_count > oxygen_one_count {
                oxygen_filter_char = 48;
            }

            oxygen = oxygen
                .iter()
                .filter(|o| o.bytes().nth(i).unwrap() == oxygen_filter_char)
                .map(|s| String::from(s))
                .collect();
        }

        if carbon.len() > 1 {
            let (ones_carbon, zeroes_carbon) = count_bytes(&carbon);
            let carbon_zero_count = zeroes_carbon.get(&i).unwrap();
            let carbon_one_count = ones_carbon.get(&i).unwrap();

            let mut carbon_filter_char = 48;
            if carbon_one_count < carbon_zero_count {
                carbon_filter_char = 49;
            }

            carbon = carbon
                .iter()
                .filter(|o| o.bytes().nth(i).unwrap() == carbon_filter_char)
                .map(|s| String::from(s))
                .collect();
        }
    }

    let oxygen_rating = usize::from_str_radix(oxygen.get(0).unwrap(), 2).unwrap();
    let carbon_rating = usize::from_str_radix(carbon.get(0).unwrap(), 2).unwrap();
    (oxygen_rating, carbon_rating)
}

#[test]
fn check_part_1_example() {
    let input = PART_1_EXAMPLE_INPUT.map(|s| s.to_string());
    let (gamma, epsilon) = calculate_gamma_epsilon_rate(&input);
    assert_eq!((gamma, epsilon), (22, 9));
    assert_eq!(gamma * epsilon, 198);
}

#[test]
fn check_part_1() {
    let input = input::read_lines("input.txt");
    let (gamma, epsilon) = calculate_gamma_epsilon_rate(&input);
    assert_eq!((gamma, epsilon), (941, 3154));
    assert_eq!(gamma * epsilon, 2_967_914);
}

#[test]
fn check_part_2_example() {
    let input = PART_1_EXAMPLE_INPUT.map(|s| s.to_string());
    let (oxygen_rating, co2_rating) = calculate_oxygen_co2_ratings(&input);
    assert_eq!((oxygen_rating, co2_rating), (23, 10));
    assert_eq!(oxygen_rating * co2_rating, 230);
}
