fn main() {
    let (diagnostic_data, number_of_bits, majority_threshold) =
        diagnostic_stuff_from_file("binary_diagnostic.data");

    let power_consumption = power_consumption(&diagnostic_data, number_of_bits, majority_threshold);
    let life_support_rating = life_support_rating(&diagnostic_data, number_of_bits);

    assert_eq!(power_consumption, 4138664);
    assert_eq!(life_support_rating, 4273224);

    println!("{:}", power_consumption);
    println!("{:}", life_support_rating);
}

fn life_support_rating(diagnostic_data: &Vec<u32>, number_of_bits: usize) -> u32 {
    return oxygen_generator_rating(diagnostic_data, number_of_bits) * co2_scrubber_rating(diagnostic_data, number_of_bits)
}

fn oxygen_generator_rating(diagnostic_data: &Vec<u32>, number_of_bits: usize) -> u32 {
    return system_rating(diagnostic_data, number_of_bits, 1)
}

fn co2_scrubber_rating(diagnostic_data: &Vec<u32>, number_of_bits: usize) -> u32 {
    return system_rating(diagnostic_data, number_of_bits, 0)
}

fn system_rating(diagnostic_data: &Vec<u32>, number_of_bits: usize, on_majority_1s: u32) -> u32 {
    assert!(on_majority_1s == 0 || on_majority_1s == 1);
    let on_majority_0s: u32 = if on_majority_1s == 0 { 1 } else { 0 };

    let mut data: Vec<u32> = diagnostic_data.clone();
    for i in 0..number_of_bits {
        let current_bit: u32 = (number_of_bits - i - 1) as u32;

        let data_length: usize = data.len();

        if data_length == 1 {
            break;
        }
        if data_length == 0 {
            panic!("All data got filtered out on bit {:}", i - 1);
        }

        // determine whether 0 or 1 is the least common ith bit
        let number_of_1s: usize = data
            .iter()
            .filter(|&&datum| datum & (1 << current_bit) != 0)
            .count();

        let least_common_bit: u32 = (
            if number_of_1s >= (data_length / 2) { on_majority_1s } else { on_majority_0s }
        ) << current_bit;

        // select those data which have the least common ith bit
        data = data
            .iter()
            .filter(|&&datum| datum & (1 << current_bit) == least_common_bit)
            .copied()
            .collect();
    }
    return data[0]
}

fn power_consumption(
    diagnostic_data: &Vec<u32>,
    number_of_bits: usize,
    majority_threshold: u32,
) -> u32 {
    let gamma_rate: u32 = bit_counts(diagnostic_data, number_of_bits)
        .iter()
        .enumerate()
        .fold(0, |acc, bit_count_enumerate| {
            if bit_count_enumerate.1 > &majority_threshold {
                acc | (1 << bit_count_enumerate.0)
            } else {
                acc
            }
        });

    let epsilon_rate: u32 = !gamma_rate & ((1 << number_of_bits) - 1);

    gamma_rate * epsilon_rate
}

fn bit_counts(diagnostic_data: &Vec<u32>, number_of_bits: usize) -> Vec<u32> {
    let mut bit_counts: Vec<u32> = vec![0; number_of_bits];
    for datum in diagnostic_data.iter() {
        for i in 0..(number_of_bits) {
            bit_counts[i] += (datum & (1 << i)) >> i
        }
    }

    bit_counts
}

fn diagnostic_stuff_from_file(path: &str) -> (Vec<u32>, usize, u32) {
    let diagnostic_data_raw: String = match std::fs::read_to_string(path) {
        Ok(string) => string,
        Err(error) => panic!("Failed to read the diagnostic data: {:?}", error),
    };

    let number_of_bits: usize = match diagnostic_data_raw.split_ascii_whitespace().nth(0) {
        Some(string) => string.len(),
        None => panic!("The diagnostic data file appears to be empty"),
    };

    let majority_threshold: u32 = diagnostic_data_raw.split_ascii_whitespace().count() as u32 / 2;

    let diagnostic_data: Vec<u32> = diagnostic_data_raw
        .split_ascii_whitespace()
        .map(|datum| match u32::from_str_radix(datum, 2) {
            Ok(int) => int,
            Err(error) => panic!("Failed to parse a binary string into a number: {:?}", error),
        })
        .collect::<Vec<u32>>();

    (diagnostic_data, number_of_bits, majority_threshold)
}
