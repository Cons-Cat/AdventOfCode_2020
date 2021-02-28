mod input;

const DEPTHS: usize = 3;

pub(crate) fn main() {
    // println!("{:?}", input::DATA);
    // v should initially be 0.
    fn recurse(d: &[i32], n: usize, o: [i32; DEPTHS]) -> Option<[i32; DEPTHS]> {
        // v is the value we are adding to to try and reach 2020.
        // n is the number of remaining depthes we may recurse.
        // o is the set of values we are using to compare.
        for i in 0..n {
            if n == 0 {
                if o.iter().sum::<i32>() + d[i] == 2020 {
                    let mut arr: [i32; DEPTHS] = o;
                    arr[arr.len() - 1] = d[i];
                    return Some(arr);
                }
                continue;
            } else {
                let mut arr: [i32; DEPTHS] = o;
                arr[arr.len() - n] = d[i];
                return recurse(&d[i..], n - 1, arr);
            }
        }
        return None;
    }

    let solution = recurse(input::DATA, DEPTHS, [0; DEPTHS]);
    match solution {
        Some(solution) => {
            println!(
                "The values are {:?}, and the solution is {}",
                solution,
                solution.iter().product::<i32>()
            )
        }
        _ => {
            eprintln!("Found no solution.")
        }
    }
}
