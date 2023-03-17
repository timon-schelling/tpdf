pub(crate) fn calc_offset(str: &str) -> usize {
    str.lines()
        .filter(|line| !line.chars().all(char::is_whitespace))
        .map(|line| line.chars().take_while(|ch| ch.is_whitespace()).count())
        .min()
        .unwrap_or(0)
}

pub(crate) fn apply_offset(str: &str, offset: usize) -> String {
    str.lines()
        .enumerate()
        .fold(String::new(), |mut acc, (i, line)| {
            if i > 0 {
                acc.push('\n');
            }
            if line.len() > offset {
                acc.push_str(&line[offset..]);
            }
            acc
        })
}
