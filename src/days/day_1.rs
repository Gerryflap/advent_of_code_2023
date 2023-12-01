pub fn run(input_1: String, input_2: String) -> String {
    format!("Q1: {}\nQ2: {}", q1(input_1), q1(input_2))
}

fn q1(input: String) -> String {
    let result: i64 = input
    .lines()
    .map(|s| s.to_owned())
    .map(combine_last_and_first_num)
    .sum();

    format!("{}", result)
}

fn q2(input: String) -> String {
    "TODO".to_owned()
}

fn combine_last_and_first_num(line: String) -> i64 {
    let line_nums: &Vec<char> = &line.chars().filter(|c| c.is_ascii_digit()).collect();

    let number = format!(
        "{}{}",
        line_nums.first().expect("No numbers in line!"),
        line_nums.last().expect("No numbers in line!")
    );

    number
        .parse()
        .expect("Number should be parsable, but is not")
}


fn combine_last_and_first_num_q2(line: String) -> i64 {
    let line_nums: &Vec<char> = &line.chars().filter(|c| c.is_ascii_digit()).collect();

    let number = format!(
        "{}{}",
        line_nums.first().expect("No numbers in line!"),
        line_nums.last().expect("No numbers in line!")
    );

    number
        .parse()
        .expect("Number should be parsable, but is not")
}