pub fn is_leap_year(year: i32) -> bool {
    match year % 4 == 0 {
        true => match year % 100 != 0 {
            true  => true,
            false => return year % 400 == 0
        },
        false => false
    }
}
