use derive_more::{From, FromStr};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_not, tag, take_while_m_n},
    character::complete::{alpha1, alphanumeric1, char, multispace1},
    combinator::{cut, not, recognize, value},
    multi::{fold, fold_many0, many0_count, many1_count},
    sequence::{delimited, pair, preceded},
};

#[derive(Debug)]
pub struct TokenStream {
    toks: Vec<Token>,
}

pub fn parse_token_stream(input: &str) -> IResult<&str, TokenStream> {
    fold_many0(
        alt((
            parse_comment_or_whitespace.map(|_| None),
            parse_token.map(Some),
        )),
        || TokenStream { toks: Vec::new() },
        |mut acc: TokenStream, e| {
            if let Some(e) = e {
                acc.toks.push(e);
            }
            acc
        },
    )
    .parse(input)
}

#[derive(Debug)]
enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Ident(Ident),
    StringLiteral(StringLiteral),
}

fn parse_token(input: &str) -> IResult<&str, Token> {
    alt((
        parse_keyword.map(Token::Keyword),
        parse_symbol.map(Token::Symbol),
        parse_ident.map(Token::Ident),
        parse_string_literal.map(Token::StringLiteral),
    ))
    .parse(input)
}

fn parse_comment<'t>(input: &'t str) -> IResult<&'t str, &'t str> {
    recognize(pair(tag("//"), many0_count(not(tag("\n"))))).parse(input)
}

fn parse_comment_or_whitespace<'t>(input: &'t str) -> IResult<&'t str, &'t str> {
    recognize(many1_count(alt((parse_comment, multispace1)))).parse(input)
}

#[derive(Debug)]
struct Ident(String);

fn recognize_ident<'t>(input: &'t str) -> IResult<&'t str, &'t str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .parse(input)
}

fn parse_ident<'t>(input: &'t str) -> IResult<&'t str, Ident> {
    recognize_ident.map(Into::into).map(Ident).parse(input)
}

#[derive(Debug)]
struct StringLiteral(Vec<u8>);

fn parse_string_literal(s: &str) -> IResult<&str, StringLiteral> {
    let build_string = fold(0.., parse_fragment, Vec::new, |mut acc, frag| {
        let mut store = [0; 4];
        acc.extend_from_slice(match &frag {
            StringFragment::Str(s) => s.as_bytes(),
            StringFragment::Byte(b) => std::slice::from_ref(b),
            StringFragment::Char(c) => c.encode_utf8(&mut store).as_bytes(),
        });
        acc
    });
    delimited(char('"'), build_string, char('"'))
        .map(StringLiteral)
        .parse(s)
}

#[derive(From, Debug)]
enum StringFragment<'t> {
    Str(&'t str),
    Byte(u8),
    Char(char),
}

fn parse_fragment<'t>(s: &'t str) -> IResult<&'t str, StringFragment<'t>> {
    alt((parse_nonescape, parse_escape)).parse(s)
}

fn parse_nonescape<'t>(s: &'t str) -> IResult<&'t str, StringFragment<'t>> {
    let not_quote_slash = is_not("\"\\");
    recognize(many1_count(not_quote_slash))
        .map(Into::into)
        .parse(s)
}

fn parse_escape<'t>(s: &'t str) -> IResult<&'t str, StringFragment<'t>> {
    preceded(
        char('\\'),
        cut(alt((
            parse_whitespace_escape,
            parse_char_escape,
            parse_byte_escape,
            parse_unicode_escape,
        ))),
    )
    .parse(s)
}

fn parse_whitespace_escape<'t>(s: &'t str) -> IResult<&'t str, StringFragment<'t>> {
    multispace1.map(|_| "".into()).parse(s)
}

fn parse_char_escape<'t>(s: &'t str) -> IResult<&'t str, StringFragment<'t>> {
    alt((
        value(b'\n', char('n')),
        value(b'\r', char('r')),
        value(b'\t', char('t')),
        value(b'\\', char('\\')),
        value(b'\0', char('0')),
        value(b'"', char('"')),
    ))
    .map(Into::into)
    .parse(s)
}

fn parse_byte_escape<'t>(s: &'t str) -> IResult<&'t str, StringFragment<'t>> {
    let parse_hex = take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit());
    let parse_prefixed_hex = preceded(char('x'), parse_hex);
    parse_prefixed_hex
        .map_res(move |hex| u8::from_str_radix(hex, 16))
        .map(Into::into)
        .parse(s)
}

fn parse_unicode_escape<'t>(s: &'t str) -> IResult<&'t str, StringFragment<'t>> {
    let parse_hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());
    let parse_delimited_hex = preceded(char('u'), delimited(char('{'), parse_hex, char('}')));
    let parse_u32 = parse_delimited_hex.map_res(move |hex| u32::from_str_radix(hex, 16));
    parse_u32
        .map_opt(std::char::from_u32)
        .map(Into::into)
        .parse(s)
}

#[derive(FromStr, Debug)]
#[from_str(rename_all = "lowercase")]
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

fn parse_keyword(input: &str) -> IResult<&str, Keyword> {
    recognize_ident.map_res(|i| i.parse()).parse(input)
}

#[derive(FromStr, Debug)]
enum Symbol {
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
    IdentBinding,
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

fn parse_symbol(input: &str) -> IResult<&str, Symbol> {
    alt((
        tag("=").map(|_| Symbol::Assign),
        tag("+").map(|_| Symbol::TraitPlus),
        tag("|").map(|_| Symbol::PatternOr),
        tag("*").map(|_| Symbol::Splat),
        tag("@").map(|_| Symbol::PatternBind),
        tag("..").map(|_| Symbol::ReverseSplat),
        tag(".").map(|_| Symbol::FieldAccessor),
        tag(",").map(|_| Symbol::FieldSeparator),
        tag(";").map(|_| Symbol::StatementSeparator),
        tag("::").map(|_| Symbol::NamespaceSeparator),
        tag(":").map(|_| Symbol::IdentBinding),
        tag("->").map(|_| Symbol::FunctionReturns),
        tag("=>").map(|_| Symbol::MatchArm),
        tag("<").map(|_| Symbol::GenericOpen),
        tag(">").map(|_| Symbol::GenericClose),
        tag("{").map(|_| Symbol::BlockOpen),
        tag("}").map(|_| Symbol::BlockClose),
        tag("[").map(|_| Symbol::ListOpen),
        tag("]").map(|_| Symbol::ListClose),
        tag("(").map(|_| Symbol::GroupOpen),
        tag(")").map(|_| Symbol::GroupClose),
    ))
    .parse(input)
}
