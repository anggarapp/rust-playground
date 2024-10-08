pub fn check_string() -> String {
    let a = String::from("test");
    a
}

pub fn debug_string_slice() -> String {
    let a = String::from("ASD");
    let b = &a[0..a.len()];
    format!("{}", b)
}

pub fn roman_to_int(s: String) -> i32 {
    let table: Vec<(i32, &'static str)> = vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut sum = 0;
    let mut idx = 0;
    for p in table.iter() {
        // jika panjang karakter di tabel kurang dari karakter yang di input
        // dan
        // karaktertabel sama dengan slice of string (index - index + panjang karakter di tabel)
        while idx + p.1.len() <= s.len() && p.1 == &s[idx..idx + p.1.len()] {
            idx += p.1.len();
            sum += p.0;
            if idx >= s.len() {
                return sum;
            }
        }
    }
    sum
}
