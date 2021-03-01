mod input;
const DATA: &[&str] = input::DATA;

pub fn main() {
    let mut valids: u32 = 0;
    for i in 0..DATA.len() {
        let vals: Vec<&str> = DATA[i].split(&['-', ' '][..]).collect();
        let min = (vals[0].chars().nth(0).unwrap().to_digit(10).unwrap() - 1) as usize;
        let max = (vals[1].chars().nth(0).unwrap().to_digit(10).unwrap() - 1) as usize;
        let letter = vals[2].chars().nth(0).unwrap();
        let letter_min = if vals[3].len() >= min {
            vals[3].chars().nth(min).unwrap()
        } else {
            ' '
        };
        let letter_max = if vals[3].len() >= max {
            vals[3].chars().nth(max).unwrap()
        } else {
            ' '
        };
        valids += (letter.eq(&letter_min) ^ letter.eq(&letter_max)) as u32;
    }
    println!("{} passwords are valid.", valids);
}
