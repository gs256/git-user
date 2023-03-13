#[cfg(test)]
mod tests {
    use crate::core::{parse_line, Profile};

    #[test]
    fn parse_line_into_profile() {
        let correct_record = Profile {
            name: "name".to_string(),
            email: "email".to_string(),
            description: "description".to_string(),
        };

        assert_eq!(
            parse_line("name:email  description").unwrap(),
            correct_record
        );

        assert_eq!(
            parse_line("name:email description").unwrap(),
            correct_record
        );

        assert_eq!(
            parse_line("name:email description ").unwrap(),
            correct_record
        );

        assert_eq!(
            parse_line(" name:email description ").unwrap(),
            correct_record
        );
    }
}
