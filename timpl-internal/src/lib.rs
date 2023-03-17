pub fn indent(indent: usize, str: String) -> String {
    let lines = str.split('\n').enumerate().peekable();
    let mut result = String::new();

    for (i, line) in lines {
        if i > 0 {
            result.push('\n');
        }

        if i == 0 || line.chars().all(char::is_whitespace) {
            result.push_str(line);
        } else {
            result.push_str(&" ".repeat(indent));
            result.push_str(line);
        }
    }

    result
}
