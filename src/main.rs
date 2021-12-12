use ui::hello;
use std::fs;
extern crate syn;
extern crate proc_macro2;
use ui_macros::function_like;
#[macro_use]
extern crate quote;

use syn::Ident;
use proc_macro2::Span;

extern crate minidom;

use minidom::Element;

macro_rules! cast_enum {
    ($it:path, $ret_type:expr) => {
        if let $it(e) = $ret_type {
            Some(e)
        }
        else{
            None
        }
    };
}

struct test {
    pub x: isize
}

macro_rules! parse_xml_struct {
    ($struct_name:ident) => {
        let elem = &$struct_name {
            x:1
        };
        println!("{}", elem.x);
    };
}

fn list_children(elem: minidom::node::Node){
    if let minidom::node::Node::Element(val) = elem {
        for iterator in val.children() {
            let text = &iterator.text();
            let text = text.trim();
            let splitted = text.split("\r\n");
            let mut end_strings: Vec<String> = Vec::new();
            for split in splitted {
                end_strings.push(String::from(split.trim()));
            }
            for split in end_strings {
                println!("{}", split);
            }
            list_children(minidom::Node::Element(iterator.clone())); // has to include Node
        }
    }
}

fn main() {
    let data:String = fs::read_to_string("assets/index.xml").unwrap();
    let root: Element = data.parse().unwrap();
    let def_ident = Ident::new("test", Span::call_site());
    let var = "abc";
    function_like!(var);
    let exe = quote! {
        parse_xml_struct!(#def_ident);
    };
    let t = proc_macro2::TokenStream::from(exe);
    list_children(minidom::node::Node::Element(root));
}