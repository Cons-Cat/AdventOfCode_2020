mod input;

fn main() {
    fn advance(x: usize, y: usize, t: u32) -> u32 {
        if y < input::DATA.len() {
            return advance(
                (x + 3) % input::DATA[0].chars().count(),
                y + 1,
                t + input::DATA[y as usize].chars().nth(x).unwrap().eq(&'#') as u32,
            );
        } else {
            return t;
        }
    }
    println!("The answer is {}", advance(0, 0, 0))
}
