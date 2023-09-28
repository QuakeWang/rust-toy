use prost::Message;
use prost_example::pb::*;

fn main() {
    let phones = vec![PhoneNumber::new("111-222-3334", PhoneType::Mobile)];
    let person = Person::new("Quake Wang", 1, "quakewang@gmail.com", phones);
    let v1 = person.encode_to_vec();
    let person1 = Person::decode(v1.as_ref()).unwrap();
    assert_eq!(person, person1);

    let json = serde_json::to_string_pretty(&person1).unwrap();

    println!("{}", json);
}