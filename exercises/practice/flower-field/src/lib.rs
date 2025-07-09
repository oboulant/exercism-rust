fn get_local_char(garden: &[&str], i: usize, j: usize, vicinity: usize) -> char {
    let nb_rows = garden.len();
    let nb_cols = garden.get(0).unwrap().len();

    match (i.checked_sub(vicinity).unwrap_or(0) ..= i.checked_add(vicinity).unwrap().min(nb_rows-1))
    .flat_map(|cline| 
        (j.checked_sub(vicinity).unwrap_or(0)
        ..= 
        j.checked_add(vicinity).unwrap().min(nb_cols-1))
        .map(move |ccol| {
            println!("cline {cline}, ccol {ccol}");
            (cline, ccol)
        }))
    .filter(|&(cline, ccol)| *garden.get(cline).unwrap().as_bytes().get(ccol).unwrap() == b'*' ) // keep only cells with a flower
    .count() {
        0 => ' ', // If 0 flower, do not display '0', rather leave a space
        count => char::from_digit(count as u32, 10).unwrap()
    }
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
    .iter().enumerate()
    .map(|(i, line)| {
        line
        .bytes()
        .enumerate()
        .map(|(j, c)| {
            match c {
                b'*' => '*', // If flower, leave it here
                _ => get_local_char(garden, i, j, 1) // Otherwise, compute surroundings flowers
            }
        })
        .collect()
    })
    .collect()
}
