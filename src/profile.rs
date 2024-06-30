use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub description: String,
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.email)
    }
}
