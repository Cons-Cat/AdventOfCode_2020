mod input;
const INIT_X: u32 = 0;
const INIT_Y: u32 = 0;

fn main() {
    fn advance(x: u32, y: u32, t: u32) -> u32 {
        if y < input::DATA.len() as u32 {
            return advance(
                (x + 3) % input::DATA[0].chars().count() as u32,
                y + 1,
                input::DATA[y as usize]
                    .chars()
                    .nth(x as usize)
                    .unwrap()
                    .eq(&'#') as u32,
            );
        } else {
            return t;
        }
    }
    println!("The answer is {}", advance(INIT_X, INIT_Y, 0))
}
