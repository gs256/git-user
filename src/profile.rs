#[derive(PartialEq, Debug)]
pub struct Profile {
    pub name: String,
    pub email: String,
    pub description: String,
}

impl Profile {
    pub fn to_string(self: &Self) -> String {
        return format!("{}:{}", self.name, self.email);
    }
}
