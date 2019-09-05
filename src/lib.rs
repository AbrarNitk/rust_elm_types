#[macro_use]
extern crate rust_elm_types_derive;

#[derive(Debug, Elm)]
struct User {
    #[elm(rename="asdas")]
    name: Option<Vec<i32>>,
    id: i32,
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