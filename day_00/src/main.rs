mod input;

pub(crate) fn main() {
    println!("{:?}", input::DATA);
    fn recurse(d: &[i32]) -> Option<(i32, i32)> {
        if d.len() < 2 {
            return None;
        }
        for i in d {
            if d[0] + i == 2020 {
                return Some((d[0], *i));
            }
        }
        recurse(&d[1..])
    }
    println!("{:?}", recurse(input::DATA));
}
