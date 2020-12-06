
pub fn parse_groups(input: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut streaming_buffer: Vec<String> = Vec::new();
    for line in input.lines() {
        if line != "" {
            streaming_buffer.push(String::from(line));
        } else {
            groups.push(streaming_buffer.clone());
            streaming_buffer.clear();
        }
    }
    if streaming_buffer.len() > 0 {
        groups.push(streaming_buffer.clone());
    }

    return Ok(groups);
}