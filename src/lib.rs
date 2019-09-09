#[macro_use]
extern crate rust_elm_types_derive;

mod temp;

#[derive(Elm)]
struct User<'a> {
    // #[elm(rename = "asdas")]
    name: Option<Vec<i32>>,
    id: &'a Vec<std::collections::HashMap<String, Vec<String>>>,
    // asdsa: (i32, i32)
    // vector: Vec<i32>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {
        println!("asfdasf");
        super::test_check();
    }
}
