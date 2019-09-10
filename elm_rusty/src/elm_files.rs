use crate::types::ElmTypes;
use std::collections::HashMap;
use std::io::Write;

pub fn generate_elm(
    path: &str,
    ident: &str,
    elm_fields: &mut HashMap<String, Vec<ElmTypes>>,
) -> std::io::Result<()> {
    let mut path: std::path::PathBuf = std::path::Path::new(path).into();
    path.push("elm_types");
    if !path.exists() {
        std::fs::create_dir(path.clone()).expect("problem creating \"elm_types\" directory");
    }
    path.push(format!("{}.elm", ident));
    let file = std::fs::File::create(path).expect("Cannot create file");
    let ref mut writer = std::io::BufWriter::new(file);
    write!(writer, "module {} exposing (..)\n\n", ident)?;
    write!(writer, "type alias {} =\n\t{{ ", ident)?;
    let mut first_elem = true;
    for (field, elm_type) in elm_fields.iter_mut() {
        elm_type.reverse();
        let elm_type = generate_elm_type(elm_type);
        if first_elem {
            write!(writer, "{}: {}\n", field, elm_type)?;
            first_elem = false;
        } else {
            write!(writer, "\t, {}: {}\n", field, elm_type)?;
        }
    }
    write!(writer, "\t}}\n")?;
    Ok(())
}

fn generate_elm_type(elm_type: &mut Vec<ElmTypes>) -> String {
    if elm_type.len() == 1 {
        return elm_type[0].elm_type();
    }
    let first = elm_type.pop().unwrap();
    let remain_len = elm_type.len();

    match first {
        ElmTypes::Maybe | ElmTypes::List => {
            if remain_len == 1 {
                first.elm_type() + " " + &elm_type.pop().unwrap().elm_type()
            } else {
                first.elm_type() + "(" + &generate_elm_type(elm_type) + ")"
            }
        }
        ElmTypes::Dict => {
            if remain_len == 2 {
                first.elm_type()
                    + " "
                    + &elm_type.pop().unwrap().elm_type()
                    + " "
                    + &elm_type.pop().unwrap().elm_type()
            } else {
                first.elm_type()
                    + " "
                    + &elm_type.pop().unwrap().elm_type()
                    + "("
                    + &generate_elm_type(elm_type)
                    + ")"
            }
        }
        _ => unimplemented!(),
    }
}
