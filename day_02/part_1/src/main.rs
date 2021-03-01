use std::str::FromStr;
mod input;
const DATA: &[&str] = input::DATA;

pub fn main() {
    let mut valids: u32 = 0;
    for i in 0..DATA.len() {
        let vals: Vec<&str> = DATA[i].split(&['-', ' '][..]).collect();
        let min = FromStr::from_str(vals[0]).unwrap();
        let max = FromStr::from_str(vals[1]).unwrap();
        let letter = vals[2].chars().nth(0).unwrap();
        let pass = vals[3];
        let count = pass.matches(letter).count();
        valids += (count >= min && count <= max) as u32;
    }
    println!("{} passwords are valid.", valids);
}
