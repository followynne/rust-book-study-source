pub fn lesson_code() {
    let number = 3;

    if number < 5 {
        println!("condition true");
    } else {
        println!("condition false");
    }

    // tagging loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // for with Range array argument
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
