// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

fn update(year: &mut u32, month: &mut u32, month_day: &mut u32, week_day: &mut u32) {
    // Including a nonexistant 0th month allows us to skip a subtraction operation each loop
    let month_length = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    *month_day += 1;
    *week_day += 1;
    if *week_day > 7 {
        *week_day = 1;
    }
    // Deal with leap years
    if *month == 2 {
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
    // All other methods are easy
    } else {
        if *month_day > month_length[*month as usize] {
            *month_day = 1;
            *month += 1;
        }
    }
    // If we pass December go back to january
    if *month == 13 {
        *month = 1;
        *year += 1;
    }
}

fn count_days() -> u64 {
    // Enclude Tuesday, January 1st, 1901 as oue starting date
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
            //println!("{},{},{},{}",year,month,month_day,week_day);
        }
    }
    return ctr;
}

pub fn euler19() -> u64 {
    return count_days();
}