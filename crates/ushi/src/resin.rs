pub fn get_exp_wit_resin(goal: i32, average: bool) -> i32 {
    let divisor = if average { 4.5 } else { 4.0 };
    let completion_times = (goal as f32 / divisor).ceil();
    (completion_times * 20.0) as i32
}

#[cfg(test)]
mod tests {
    use super::get_exp_wit_resin;

    #[test]
    fn exp_wit_resin() {
        let wits = vec![8, 9, 10];
        assert_eq!(get_exp_wit_resin(wits[0], false), 40);
        assert_eq!(get_exp_wit_resin(wits[1], false), 60);
        assert_eq!(get_exp_wit_resin(wits[2], true), 60);
    }
}
