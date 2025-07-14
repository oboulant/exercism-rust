use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut res: HashSet<[u32; 3]> = HashSet::new();

    // If you put your right triangle into a circle that encompass all three vertices
    // then the hypotenuse is the diameter.
    // To get a, you start from & (integer) and go up to the isosceles triangle (where a == b)
    // When you have the isosceles triangle, you can compute a, because
    //  * You know that a + b + c = sum
    //  * You know that c = diameter
    //  * You know that a = diameter / sqrt(2)
    // So, for a given sum, you know the maximum value of a that guarantees that a < b < c
    // --> max(a) = sum / (2 + sqrt(2))
    //
    // Then, once we iterate for each possible value of a, we then compute the value c
    // using the same facts that the right triangle a * a + b * b = c * c and
    // a + b + c = sum. The formula is
    // c = (1 + (sum - 1) * (sum - 1)) / (2 * (N - 1))
    // If the numerator can be divided by the denominator, then the right triangle exists
    // for integer length.
    //
    // Finaly, one just needs to compute b = sum - a - c

    (1..=(sum as f32 / (2.0_f32.sqrt() + 2.0)).floor() as u32).for_each(|a: u32| {
        let c_numerator: u32 = a * a + (sum - a) * (sum - a);
        let c_denominator: u32 = 2 * (sum - a);
        if c_numerator % c_denominator == 0 {
            // c exists. Add the right triangle to the results
            let c = c_numerator / c_denominator;
            res.insert([a, sum - a - c, c]);
        }
    });

    println!("{res:?}");

    res
}
