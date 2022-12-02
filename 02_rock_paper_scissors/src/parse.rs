pub fn parse_letter_pair(s: &str) -> Result<(char, char), String> {
    let mut letters = s.chars().filter(|c| c.is_alphabetic()).take(2);
    let a = letters.next().ok_or("no first letter")?;
    let b = letters.next().ok_or("no second letter")?;
    Ok((a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_letters() {
        assert_eq!(Ok(('A', 'Z')), parse_letter_pair("    A  \t Z  \n"));
    }
}
