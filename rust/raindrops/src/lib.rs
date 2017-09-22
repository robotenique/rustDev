
pub fn raindrops(n: usize) -> String {
    let mut f = 1;
    let mut my_str = String::new();
    while f <= n {
        if n % f == 0 {
            my_str.push_str(match f {
                3 => "Pling",
                5 => "Plang",
                7 => "Plong",
                _ => ""
            });
        }
        f += 1;
    }
    match my_str == "" {
        true => n.to_string(),
        false => my_str
    }
}
