pub fn actions(n: u8) -> Vec<&'static str> {
    let actions = ["wink", "double blink", "close your eyes", "jump"];
    let mut res: Vec<&'static str> = Vec::new();

    for (i, action) in actions.iter().enumerate() {
        if n & (0b000_0001 << i) != 0 {
            res.push(action);
        }
    }

    if n & 0b001_0000 != 0 {
        res.reverse()
    }
    res
}
