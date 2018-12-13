//! `mpi-test-macro`

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;

use proc_macro2::{Ident, Literal, TokenStream, TokenTree};

const E: &str = "expected \"#[mpi_test($number_of_processes)]\" where \
                 $number_of_processes is an integer";

#[proc_macro_attribute]
pub fn mpi_test(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tokens = TokenStream::from(args).into_iter().collect::<Vec<_>>();
    if tokens.len() != 1 {
        panic!(E);
    }

    let number_of_processes: usize = match &tokens[0] {
        TokenTree::Literal(tt) => tt.to_string().parse().expect(E),
        _ => panic!(E),
    };

    let item = TokenStream::from(input);
    let test_name = find_name(item.clone());

    let test_name = test_name.to_string();
    let mut test_ty_name = test_name.to_string();
    test_ty_name.push_str("_mpi_test");

    let test_name_str = Literal::string(&test_name);

    let test_name: TokenStream = test_name
        .parse()
        .unwrap_or_else(|_| panic!("failed to parse name: {}", test_name));

    let test_ty_name: TokenStream = test_ty_name
        .parse()
        .unwrap_or_else(|_| panic!("failed to parse name: {}", test_ty_name));

    quote_spanned!(
        proc_macro2::Span::call_site() =>
        #[test_case]
        const #test_ty_name: ampi_test_runner::Test = ampi_test_runner::Test {
            test: #test_name,
            name: & #test_name_str,
            np: #number_of_processes,
        };

        #item
    )
    .into()
}

fn find_name(item: TokenStream) -> Ident {
    let mut tokens = item.into_iter();
    while let Some(tok) = tokens.next() {
        if let TokenTree::Ident(word) = tok {
            if word == "fn" {
                break;
            }
        }
    }

    match tokens.next() {
        Some(TokenTree::Ident(word)) => word,
        _ => panic!("failed to find function name"),
    }
}
