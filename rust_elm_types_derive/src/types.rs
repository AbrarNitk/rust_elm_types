enum ElmTypes {
    Bool,
    String,
    Int,
    Float,
    Char,
    List,
    Map,
    Maybe,
    Result,
    Unknown,
}

impl ElmTypes {
    fn elm_type(&self) -> String {
        (match self {
            ElmTypes::Bool => "bool",
            ElmTypes::String => "String",
            ElmTypes::Int => "Int",
            ElmTypes::Float => "Float",
            ElmTypes::Char => "Char",
            ElmTypes::List => "List",
            ElmTypes::Map => "Map",
            ElmTypes::Maybe => "Maybe",
            ElmTypes::Result => "Result",
            ElmTypes::Unknown => "Unknown",
        })
        .to_string()
    }
}

enum RustType {
    Char,
    Bool,
    I8,
    I16,
    I32,
    I64,
    ISize,
    U8,
    U16,
    U32,
    U64,
    USize,
    F32,
    F64,
    String,
    Vec,
    HashMap,
    Option,
    Result,
    Unknown,
}

impl RustType {
    fn rust_type(&self) -> String {
        (match self {
            RustType::Char => "char",
            RustType::Bool => "bool",
            RustType::I8 => "i8",
            RustType::I16 => "i16",
            RustType::I32 => "i32",
            RustType::I64 => "i64",
            RustType::ISize => "isize",
            RustType::U8 => "u8",
            RustType::U16 => "u16",
            RustType::U32 => "u32",
            RustType::U64 => "u64",
            RustType::USize => "usize",
            RustType::F32 => "f32",
            RustType::F64 => "f64",
            RustType::String => "String",
            RustType::Vec => "Vec",
            RustType::HashMap => "HashMap",
            RustType::Option => "Option",
            RustType::Result => "Result",
            RustType::Unknown => "Unknown",
        })
        .to_string()
    }
}

impl<'a> From<&'a str> for RustType {
    fn from(input: &str) -> Self {
        match input {
            "char" => RustType::Char,
            "bool" => RustType::Bool,
            "i8" => RustType::I8,
            "i16" => RustType::I16,
            "i32" => RustType::I32,
            "i64" => RustType::I64,
            "isize" => RustType::ISize,
            "u8" => RustType::U8,
            "u16" => RustType::U16,
            "u32" => RustType::U32,
            "u64" => RustType::U64,
            "usize" => RustType::USize,
            "f32" => RustType::F32,
            "f64" => RustType::F64,
            "String" => RustType::String,
            "Vec" => RustType::Vec,
            "HashMap" => RustType::HashMap,
            "Option" => RustType::Option,
            "Result" => RustType::Result,
            _ => RustType::Unknown,
        }
    }
}

impl<'a> From<&'a RustType> for ElmTypes {
    fn from(input: &RustType) -> Self {
        match input {
            RustType::Char => ElmTypes::Char,
            RustType::Bool => ElmTypes::Bool,
            RustType::I8 => ElmTypes::Int,
            RustType::I16 => ElmTypes::Int,
            RustType::I32 => ElmTypes::Int,
            RustType::I64 => ElmTypes::Int,
            RustType::ISize => ElmTypes::Int,
            RustType::U8 => ElmTypes::Int,
            RustType::U16 => ElmTypes::Int,
            RustType::U32 => ElmTypes::Int,
            RustType::U64 => ElmTypes::Int,
            RustType::USize => ElmTypes::Int,
            RustType::F32 => ElmTypes::Float,
            RustType::F64 => ElmTypes::Float,
            RustType::String => ElmTypes::String,
            RustType::Vec => ElmTypes::List,
            RustType::HashMap => ElmTypes::Map,
            RustType::Option => ElmTypes::Maybe,
            RustType::Result => ElmTypes::Result,
            RustType::Unknown => ElmTypes::Unknown,
        }
    }
}
