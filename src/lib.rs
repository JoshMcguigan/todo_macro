#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
#[macro_use]
extern crate syn;

extern crate chrono;

use proc_macro::TokenStream;
use chrono::prelude::Utc;
use chrono::NaiveDate;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;

struct ToDo {
    _description: String,
    date: String,
}

impl Parse for ToDo {
    fn parse(input: ParseStream) -> Result<Self> {
        let description: syn::LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let date: syn::LitStr = input.parse()?;
        Ok(ToDo {
            _description: description.value(),
            date: date.value(),
        })
    }
}

#[proc_macro]
pub fn todo(input: TokenStream) -> TokenStream {

    let input_clone = input.clone();
    let ToDo {
        date,
        ..
    } = parse_macro_input!(input_clone as ToDo);

    match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(parsed_date) => {
            if parsed_date.lt(&Utc::now().naive_utc().date()) {
                input.into_iter().next().unwrap().span().error("todo deadline has passed").emit();
            }
        },
        Err(_) => {
            input.into_iter().next().unwrap().span().error("Failed to parse second argument as date").emit();
        },
    }

    TokenStream::new()
}
