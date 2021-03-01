mod input;

fn main() {
    fn advance(x: usize, y: usize, slope: &(usize, usize), t: u32) -> u32 {
        if y < input::DATA.len() {
            return advance(
                (x + slope.0) % input::DATA[0].chars().count(),
                y + slope.1,
                slope,
                t + input::DATA[y as usize].chars().nth(x).unwrap().eq(&'#') as u32,
            );
        } else {
            return t;
        }
    }
    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!(
        "The product is: {:?}",
        slopes
            .iter()
            .map(|x| advance(0, 0, x, 0))
            .collect::<Vec<u32>>()
            .iter()
            .product::<u32>()
    );
}
