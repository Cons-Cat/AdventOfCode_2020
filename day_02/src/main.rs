mod input;

const DEPTHS: usize = 3;

pub(crate) fn main() {
    // println!("{:?}", input::DATA);
    // v should initially be 0.
    fn recurse(d: &[i32], n: usize, o: [i32; DEPTHS]) -> Option<[i32; DEPTHS]> {
        // d is the data set we compare to.
        // n is the number of remaining depthes we may recurse.
        // o is the set of values we are using to compare.
        for i in 0..d.len() {
            if n == 0 {
                if o.iter().sum::<i32>() + d[i] == 2020 {
                    // println!("found result");
                    let mut arr: [i32; DEPTHS] = o;
                    arr[arr.len() - 1] = d[i];
                    return Some(arr);
                }
                continue;
            } else {
                let mut arr: [i32; DEPTHS] = o;
                arr[arr.len() - n] = d[i];
                // println!("{}, {:?}", n, arr);
                let val = recurse(&d[i + 1..], n - 1, arr);
                if val != None {
                    return val;
                }
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
