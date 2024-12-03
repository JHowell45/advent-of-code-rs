pub fn parse_report(report: &str) -> Vec<i32> {
    report
        .split(" ")
        .map(|v| v.parse::<i32>().unwrap())
        .collect()
}

pub fn is_safe(values: Vec<i32>) -> Option<usize> {
    let mut state: Option<bool> = None;

    for index in 0..values.len() - 1 {
        let a = values[index];
        let b = values[index + 1];
        if (a - b).abs() > 3 || (a - b).abs() < 1 {
            return Some(index + 1);
        }
        let local_state = if a < b { true } else { false };
        match state {
            None => state = Some(local_state),
            Some(state) => {
                if state != local_state {
                    return Some(index + 1);
                }
            }
        }
    }
    return None;
}
