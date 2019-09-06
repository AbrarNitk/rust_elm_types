#[macro_use]
extern crate rust_elm_types_derive;

mod temp;

#[derive(Elm)]
struct User {
    #[elm(rename = "asdas")]
    name: Option<Vec<i32>>,
    id: temp::User,
    vector: Vec<i32>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {
        println!("asfdasf");
        super::test_check();
    }
}
