# Rust to Elm Type Generator
### This repo contains simple rust struct to elm type generator.

## Examples
1. File Path may be set into env variable called (ELM_TYPES).
2. File path may be passed as elm options, this will override env path.

- ### Example 1 
- #### struct with option path(path is mandatory option)
```rust
#[macro_use]
extern crate realm_mate;

// Declare struct with Elm derive
// Here path is mandatory in elm opts or export in env variable as ELM_TYPES
#[derive(Elm)]
#[elm(opts(path = "/Users/abrarkhan/Documents/github/rust_elm_types"))]
struct Foo {
    id: i32,
    name: String,
}
```
After change into models, When ever run cargo build, check or test first time it will generated corresponding Elm code.

```elm
module Foo exposing (..)

type alias Foo =
	{ id: Int
	, name: String
	}

```
- ###Example 2
- #### struct with option path and rename
```rust
 #[macro_use]
extern crate realm_mate;

mod temp {
    pub struct User {}
}

#[derive(Elm)]
#[elm(opts(rename = "ElmUser", path = "/Users/abrarkhan/Documents/github/rust_elm_types"))]
struct User {
    name: Option<Vec<i32>>,
    id: Vec<std::collections::HashMap<String, Vec<temp::User>>>,
    vector: Vec<i32>,
}
```

```elm
module ElmUser exposing (..)

type alias ElmUser =
	{ vector: List Int
	, name: Maybe(List Int)
	, id: List(Dict String(List User))
	}

```
## Note Point
Here, I did not handle recursively custom type derive Elm, so it won't create corresponding User.elm

- ###Example 3
- #### struct with option path, elm type rename and field rename option
```rust
#[macro_use]
extern crate realm_mate;

mod temp {
    pub struct User {}
}

#[derive(Elm)]
#[elm(opts(rename = "ElmUser", path = "/Users/abrarkhan/Documents/github/rust_elm_types"))]
struct User {
    #[elm(rename = "foo")]
    name: Option<Vec<i32>>,
    id: Vec<std::collections::HashMap<String, Vec<temp::User>>>,
    vector: Vec<i32>,
}
```

```elm

module ElmUser exposing (..)

type alias ElmUser =
	{ id: List(Dict String(List User))
	, foo: Maybe(List Int)
	, vector: List Int
	}

```

- ###Example 4
- #### struct(reference types) with option path, elm type rename and field rename option
```rust

#[macro_use]
extern crate realm_mate;

mod temp {
    pub struct User {}
}

#[derive(Elm)]
#[elm(opts(rename = "ElmUser", path = "/Users/abrarkhan/Documents/github/rust_elm_types"))]
struct User<'a> {
    #[elm(rename = "foo")]
    name: Option<Vec<i32>>,
    id: &'a Vec<std::collections::HashMap<String, Vec<temp::User>>>,
    vector: Vec<i32>,
}
```

```elm

module ElmUser exposing (..)

type alias ElmUser =
	{ id: List(Dict String(List User))
	, foo: Maybe(List Int)
	, vector: List Int
	}

```


- ###Example 5
- #### without elm options and export path as ELM_TYPES="/Users/abrarkhan/Documents/github/rust_elm_types"
```rust

#[macro_use]
extern crate realm_mate;

mod temp {
    pub struct User {}
}

#[derive(Elm)]
struct User<'a> {
    #[elm(rename = "foo")]
    name: Option<Vec<i32>>,
    id: &'a Vec<std::collections::HashMap<String, Vec<temp::User>>>,
    vector: Vec<i32>,
}
```

```elm

module User exposing (..)

type alias User =
	{ id: List(Dict String(List User))
	, foo: Maybe(List Int)
	, vector: List Int
	}

```

# RoadMap
*[x] Generate Elm types alias from Rust struct.

*[ ] Make recursively type checks.

*[ ] Auto Import of Elm modules, if it exists inside same dir.

*[ ] Generate Elm types from Rust types.

*[ ] Generate Elm encoders from Rust types.

*[ ] Generate Elm decoders from Rust types.