extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

use quote::Tokens;

use syn::Lit;
use syn::MetaItem;
use syn::NestedMetaItem;
use syn::Body;
use syn::VariantData;

#[proc_macro_derive(LiteralLength, attributes(Literals))]
pub fn literal_length(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();

    let meta = ast.attrs
        .into_iter()
        .filter(|a| a.name() == "Literals")
        .next()
        .expect("unable to find literals");

    let literals = match meta.value {
        MetaItem::List(_, ref lits) => lits,
        _ => panic!("literals must be given in a list"),
    };

    let mut result = Tokens::new();
    for lit in literals {
        use NestedMetaItem::*;
        use MetaItem::*;
        use Lit::*;

        let (name, value) = match *lit {
            MetaItem(NameValue(ref id, ref lit)) => (id, lit),
            ref lit => panic!("expected name-value pair, found `{:?}`", lit),
        };

        let size = match *value {
            Str(ref v, _) => v.len(),
            ByteStr(ref b, _) => b.len(),
            ref e => panic!("expected a str literal or bytestr literal, found `{:?}`", e),
        };

        let name_len = quote::Ident::from(format!("{}_LEN", name));

        result.append(quote! {
                static #name : &str = #value;
                const #name_len : usize = #size;
            });
    }

    result.parse().unwrap()
}