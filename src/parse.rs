enum Keyword {
    Else,
    Enum,
    Fn,
    If,
    Impl,
    Let,
    Match,
    Mod,
    Pub,
    Struct,
    Trait,
    Type,
    Use,
    Where,
}

enum Symbols {
    // =
    Assign,
    // +
    TraitPlus,
    // |
    PatternOr,
    // *
    Splat,
    // @
    PatternBind,
    // .
    FieldAccessor,
    // ..
    ReverseSplat,
    // ,
    FieldSeparator,
    // ;
    StatementSeparator,
    // :
    TypeBinding,
    // ::
    NamespaceSeparator,
    // ->
    FunctionReturns,
    // =>
    MatchArm,
    // <
    GenericOpen,
    // >
    GenericClose,
    // {
    BlockOpen,
    // }
    BlockClose,
    // [
    ListOpen,
    // ]
    ListClose,
    // (
    GroupOpen,
    // )
    GroupClose,
}

struct Ident(String);

struct StringLiteral(Vec<u8>);

struct IntLiteral(u128);

struct FloatLiteral(f64);
