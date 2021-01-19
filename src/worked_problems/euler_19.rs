// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

fn update(year: &mut u32, month: &mut u32, month_day: &mut u32, week_day: &mut u32) {
    // Including a nonexistant 0th month allows us to skip a subtraction operation each loop
    let month_length = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    *month_day += 1;
    *week_day += 1;
    if *week_day > 7 {
        *week_day = 1;
    }
    if *month == 2 { // Deal with leap years
        if *year % 4 == 0 && *year % 400 != 0 {
            if *month_day > month_length[*month as usize]+1 {
                *month_day = 1;
                *month += 1;
            }
        } else {
            if *month_day > month_length[*month as usize] {
                *month_day = 1;
                *month += 1;
            }
        }
    } else {// All other months are easy
        if *month_day > month_length[*month as usize] {
            *month_day = 1;
            *month += 1;
        }
    }
    if *month == 13 { // If we pass December go back to january
        *month = 1;
        *year += 1;
    }
}

fn count_days() -> u64 {
    // Include Tuesday, January 1st, 1901 as the starting date
    let mut year: u32 = 1901;
    let mut month: u32 = 1;
    let mut month_day: u32 = 1;
    let mut week_day: u32 = 2;

    let mut ctr = 0;
    loop {
        // Pass mutable references here so that the update function can change the variables
        // we give to it
        update(&mut year, &mut month, &mut month_day, &mut week_day);
        if year == 2001 {
            break;
        }
        if month_day == 1 && week_day == 7 {
            ctr += 1;
        }
    }
    ctr
}

pub fn euler19() -> u64 {
    count_days()
}

pub fn euler19_example() {
    println!("\nProblem: How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?");
    println!("\n\nThis is somewhat easier than it appears. We need only track year, month, day of the month, and day of the week as well as keep in mind how many days in each month. Knowing that we allow for leap years and cycle through all the days in question.");
    let s = "
fn update(year: &mut u32, month: &mut u32, month_day: &mut u32, week_day: &mut u32) {
    // Including a nonexistant 0th month allows us to skip a subtraction operation each loop
    let month_length = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    *month_day += 1;
    *week_day += 1;
    if *week_day > 7 {
        *week_day = 1;
    }
    if *month == 2 { // Deal with leap years
        if *year % 4 == 0 && *year % 400 != 0 {
            if *month_day > month_length[*month as usize]+1 {
                *month_day = 1;
                *month += 1;
            }
        } else {
            if *month_day > month_length[*month as usize] {
                *month_day = 1;
                *month += 1;
            }
        }
    } else {// All other months are easy
        if *month_day > month_length[*month as usize] {
            *month_day = 1;
            *month += 1;
        }
    }
    if *month == 13 { // If we pass December go back to january
        *month = 1;
        *year += 1;
    }
}

fn count_days() -> u64 {
    // Include Tuesday, January 1st, 1901 as the starting date
    let mut year: u32 = 1901;
    let mut month: u32 = 1;
    let mut month_day: u32 = 1;
    let mut week_day: u32 = 2;

    let mut ctr = 0;
    loop {
        // Pass mutable references here so that the update function can change the variables
        // we give to it
        update(&mut year, &mut month, &mut month_day, &mut week_day);
        if year == 2001 {
            break;
        }
        if month_day == 1 && week_day == 7 {
            ctr += 1;
        }
    }
    ctr
}

pub fn euler19() -> u64 {
    count_days()
}";
    println!("\n{}\n",s);
    println!("The answer is: {}",euler19());
}

#[test]
fn test19() {
    assert_eq!(euler19(),171)
}