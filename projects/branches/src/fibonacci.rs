pub fn fibonacci(value: i32) -> i128 {
    if value < 0 {
        panic!();
    }

    let mut result: i128 = 1;
    let mut current_entry: i128 = 1;
    for _element in 1..value - 1 {
        result += current_entry;
        current_entry = result - current_entry;
    }

    result
}
