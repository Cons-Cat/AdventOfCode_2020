mod input;
const DATA: &[&str] = input::DATA;

pub fn main() {
    for i in 0..DATA.len() {
        // let mut cur_str: std::str::Chars = DATA[i].chars();
        // let mut point_char: char = ' ';
        // let mut min_str: String = String::new();
        // while point_char != ('-') {
        //     point_char = cur_str.next().unwrap();
        //     min_str.push(point_char);
        // }
        // let min_val = min_str.parse::<i32>().unwrap();
        // // Skip past the '-'
        // cur_str.next();
        // let mut max_str: String = String::new();
        // while point_char != (':') {
        //     point_char = cur_str.next().unwrap();
        //     max_str.push(point_char);
        // }
        // let max_val = max_str.parse::<i32>().unwrap();
        // // Skip past the ': '
        // cur_str.next();
        // cur_str.next();
        // let password = String::new();
        // while (Some(cur_str.current().un)) {
        //     password.push(cur_str.next())
        // }
        // break;
        let vals: Vec<&str> = DATA[i].split(&['-', ' '][..]).collect();
        let min = vals[0];
        let max = vals[1];
        let pass = vals[3];
    }
}
