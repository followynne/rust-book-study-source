pub fn from_fahreneit_to_celsius(entry_grade: f32, is_celsius: bool) {
    let res = match is_celsius {
        true => (entry_grade * 1.8000) + 32.0,
        false => (entry_grade - 32.0) / 1.8000,
    };

    println!("{res}");
}
