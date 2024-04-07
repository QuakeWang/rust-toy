mod person;

pub use self::person::person::{PhoneNumber, PhoneType};
pub use person::*;

impl Person {
    pub fn new(
        name: impl Into<String>,
        id: i32,
        email: impl Into<String>,
        phones: impl Into<Vec<PhoneNumber>>,
    ) -> Self {
        Self {
            name: name.into(),
            id,
            email: email.into(),
            phones: phones.into(),
            ..Default::default()
        }
    }
}
impl PhoneNumber {
    pub fn new(number: impl Into<String>, phone_type: PhoneType) -> Self {
        Self {
            number: number.into(),
            phone_type: phone_type.into(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_person() {
        let person = Person::new(
            "Sam".to_string(),
            01,
            "sam@google.com".to_string(),
            vec![PhoneNumber::new(
                "123-456-7890".to_string(),
                PhoneType::Home,
            )],
        );
        assert_eq!(person.name, "Sam");
        assert_eq!(person.id, 01);
        assert_eq!(person.email, "sam@google.com");
        assert_eq!(person.phones.len(), 1);
        assert_eq!(person.phones[0].number, "123-456-7890");
        assert_eq!(person.phones[0].phone_type, PhoneType::Home.into());
    }
}
