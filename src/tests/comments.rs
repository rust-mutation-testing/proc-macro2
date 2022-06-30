use crate::{Delimiter, Literal, Spacing, TokenStream, TokenTree};

// #[doc = "..."] -> "..."
fn lit_of_outer_doc_comment(tokens: &TokenStream) -> Literal {
    lit_of_doc_comment(tokens, false)
}

// #![doc = "..."] -> "..."
fn lit_of_inner_doc_comment(tokens: &TokenStream) -> Literal {
    lit_of_doc_comment(tokens, true)
}

fn lit_of_doc_comment(tokens: &TokenStream, inner: bool) -> Literal {
    let mut iter = tokens.clone().into_iter();
    match iter.next().unwrap() {
        TokenTree::Punct(punct) => {
            assert_eq!(punct.as_char(), '#');
            assert_eq!(punct.spacing(), Spacing::Alone);
        }
        _ => panic!("wrong token {:?}", tokens),
    }
    if inner {
        match iter.next().unwrap() {
            TokenTree::Punct(punct) => {
                assert_eq!(punct.as_char(), '!');
                assert_eq!(punct.spacing(), Spacing::Alone);
            }
            _ => panic!("wrong token {:?}", tokens),
        }
    }
    iter = match iter.next().unwrap() {
        TokenTree::Group(group) => {
            assert_eq!(group.delimiter(), Delimiter::Bracket);
            assert!(iter.next().is_none(), "unexpected token {:?}", tokens);
            group.stream().into_iter()
        }
        _ => panic!("wrong token {:?}", tokens),
    };
    match iter.next().unwrap() {
        TokenTree::Ident(ident) => assert_eq!(ident.to_string(), "doc"),
        _ => panic!("wrong token {:?}", tokens),
    }
    match iter.next().unwrap() {
        TokenTree::Punct(punct) => {
            assert_eq!(punct.as_char(), '=');
            assert_eq!(punct.spacing(), Spacing::Alone);
        }
        _ => panic!("wrong token {:?}", tokens),
    }
    match iter.next().unwrap() {
        TokenTree::Literal(literal) => {
            assert!(iter.next().is_none(), "unexpected token {:?}", tokens);
            literal
        }
        _ => panic!("wrong token {:?}", tokens),
    }
}

#[test]
fn closed_immediately() {
    let stream = "/**/".parse::<TokenStream>().unwrap();
    let tokens = stream.into_iter().collect::<Vec<_>>();
    assert!(tokens.is_empty(), "not empty -- {:?}", tokens);
}

#[test]
fn incomplete() {
    assert!("/*/".parse::<TokenStream>().is_err());
}

// HACK(mutest): The rustc test harness generates a `lit` const descriptor, which clashes with the `lit` locals. We can
//               fix these for now by using a different name for the locals.
#[test]
fn lit() {
    let stream = "/// doc".parse::<TokenStream>().unwrap();
    let l_lit = lit_of_outer_doc_comment(&stream);
    assert_eq!(l_lit.to_string(), "\" doc\"");

    let stream = "//! doc".parse::<TokenStream>().unwrap();
    let l_lit = lit_of_inner_doc_comment(&stream);
    assert_eq!(l_lit.to_string(), "\" doc\"");

    let stream = "/** doc */".parse::<TokenStream>().unwrap();
    let l_lit = lit_of_outer_doc_comment(&stream);
    assert_eq!(l_lit.to_string(), "\" doc \"");

    let stream = "/*! doc */".parse::<TokenStream>().unwrap();
    let l_lit = lit_of_inner_doc_comment(&stream);
    assert_eq!(l_lit.to_string(), "\" doc \"");
}

// HACK(mutest): The rustc test harness generates a `lit` const descriptor, which clashes with the `lit` locals. We can
//               fix these for now by using a different name for the locals.
#[test]
fn carriage_return() {
    let stream = "///\r\n".parse::<TokenStream>().unwrap();
    let l_lit = lit_of_outer_doc_comment(&stream);
    assert_eq!(l_lit.to_string(), "\"\"");

    let stream = "/**\r\n*/".parse::<TokenStream>().unwrap();
    let l_lit = lit_of_outer_doc_comment(&stream);
    assert_eq!(l_lit.to_string(), "\"\\r\\n\"");

    "///\r".parse::<TokenStream>().unwrap_err();
    "///\r \n".parse::<TokenStream>().unwrap_err();
    "/**\r \n*/".parse::<TokenStream>().unwrap_err();
}
