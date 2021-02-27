mod input;

struct MyError {
    details: &'static str,
}

const E: MyError = MyError {
    details: "No elements sum to 2020.",
};

pub(crate) fn main() {
    println!("{:?}", input::DATA);
    fn recurse(d: &[i32]) -> Result<(i32, i32), MyError> {
        if d.len() < 2 {
            return Err(E);
        }
        for i in d {
            if d[0] + i == 2020 {
                return Ok((d[0], *i));
            }
        }
        recurse(&d[1..])
    }
    let solution = recurse(input::DATA);
    match solution {
        Ok(solution) => {
            println!("The solution is {:?}", solution)
        }
        // Err(E) => {
        _ => {
            // This is probably a bad solution.
            eprintln!("Error")
        }
    }
}
