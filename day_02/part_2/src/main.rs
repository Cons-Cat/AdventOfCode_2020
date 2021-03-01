mod input;
const DATA: &[&str] = input::DATA;

pub fn main() {
    let mut valids: u32 = 0;
    for i in 0..DATA.len() {
        let vals: Vec<&str> = DATA[i].split(&['-', ' '][..]).collect();
        let min: usize = vals[0].to_string().parse::<usize>().unwrap() - 1;
        let max: usize = vals[1].to_string().parse::<usize>().unwrap() - 1;
        let letter_compare = vals[2].chars().nth(0).unwrap();
        let letter_min = vals[3].chars().nth(min).unwrap();
        let letter_max = vals[3].chars().nth(max).unwrap();
        valids += (letter_compare.eq(&letter_min) ^ letter_compare.eq(&letter_max)) as u32;
    }
    println!("{} passwords are valid.", valids);
}
