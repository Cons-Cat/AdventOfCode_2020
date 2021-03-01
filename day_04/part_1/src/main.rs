// use Vec::<;

mod input;

fn main() {
    let data = input::DATA;
    fn is_field(x: &str) -> bool {
        return x == "eyr"
            || x == "iyr"
            || x == "byr"
            || x == "ecl"
            || x == "pid"
            || x == "hcl"
            || x == "hgt";
    }

    let tmp = data
        .iter()
        // Map passport strings into a vector of vectors of field strings.
        .map(|x| x.split(&[':', ' ', '\n'][..]).collect())
        .collect::<Vec<Vec<&str>>>()
        .into_iter()
        // Map the vector of vectors of field strings into a vector of
        // vectors of only the fields I care about.
        .map(|x| Vec::<&str>::filter(|y| is_field(y)))
        // .filter(|x| is_field(x))
        .collect::<Vec<Vec<&str>>>()
        .into_iter()
        // Map the string vector to its length.
        .map(|x| x.len())
        .collect::<Vec<_>>()
        .into_iter()
        // Filter the list to only elements mapped from vectors with all seven fields.
        .filter(|x| x == &7)
        .collect::<Vec<_>>()
        // Return the length of this filtered list.
        .len();
    println!("{}", tmp);
    println!(
        "{:?}",
        data.iter().flat_map(|x| x.split(':')).collect::<Vec<_>>()
    );
}
