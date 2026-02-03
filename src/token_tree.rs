struct Where {}

struct Generics {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IdentDecl(String);

struct IdentPath{
    last: String,
    path: Vec<String>,
}

// function
//
// let
//
// struct 
struct Struct {
    generics: Generics,
    ident: IdentDecl,
    where_clause: Where,
    
}

enum StructBody {
    Struct{

    },
    Tuple,
    Unit,
}

// enum 
//
// type
//
// trait  
//
// use  
//
// splat 
//
// mod
//
// impl 
