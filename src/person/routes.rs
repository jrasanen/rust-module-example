use super::fields::*;

pub fn get() -> String {
    let user = Person {
        username: "username".to_string(),
        email: "email".to_string(),
        address: Address {
            street: "street".to_string(),
            city: "city".to_string(),
            state: "state".to_string(),
            zip: "zip".to_string(),
        },
    };

    user.username
}
