extern crate proc_macro;
use quote::quote;
use proc_macro2::Span;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[proc_macro]
pub fn function_like(_inp: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(_inp);
    let output: proc_macro2::TokenStream = quote!{
            #[derive(Debug)]
            struct #input {
                x:usize
            };
            let l = #input {
                x:1
            };
            println!("{:?}", l);
    }.into();
    proc_macro::TokenStream::from(output)
}