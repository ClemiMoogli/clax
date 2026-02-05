use nom::{IResult}

struct Where {}

struct Generics {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IdentDecl(String);

struct IdentPath {
    last: String,
    path: Vec<String>,
}

struct Arg {
    ident: IdentDecl,
    return_type: Type,
}

// function
struct Function {
    ident: IdentDecl,
    generics: Generics,
    args: vec![Arg],
    return_type: Type,
    where_clause: Where,
    body: StructBody,
}

// let
struct Let {
    ident: IdentDecl,
    return_type: Type,
    valeur: StructBody,
}

// struct
struct Struct {
    generics: Generics,
    ident: IdentDecl,
    where_clause: Where,
}

enum StructBody {
    Struct {},
    Tuple,
    Unit,
}

// enum
struct Enum {
    ident: IdentDecl,
    values: vec![String],
}

// type
enum Type {
    i32,
    String,
}

// trait
//
// use
//
// splat
//
// mod
//
// impl

fn parse_function(stream: TokenStream) -> Function {

}
