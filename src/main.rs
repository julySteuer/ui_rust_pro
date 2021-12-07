use serde;
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};
use ui::hello;

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

macro_rules! concat {
    ($new_struct:ident,$new_enum:ident, $($elem:tt)*) => {
        #[derive(Debug)]
        enum $new_enum {
            $($elem)*
        }
        struct $new_struct {
            vals:$new_enum,
            names: String
        }
    };
}

macro_rules! make {
    ($new_struct:ident,$new_enum:ident, $($elem:expr)*) => {
        #[derive(Debug)]
        enum $new_enum {
            $($elem)*
        }
        struct $new_struct {
            vals:$new_enum,
            names: String
        }
    };
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum subItemEnum {
    subItem(subItem),
    subItemTwo(subItemTwo)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct subItem {
    test:String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct subItemTwo {
    test:String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    #[serde(rename = "$value")]
    events: Vec<subItemEnum>
}

fn main() {
    let src = r#"<Item><subItem><test>Hello</test></subItem></Item>"#;
    let item: Item = from_str(src).unwrap();
    concat!(test, test1,Foo(subItem));
    make!(world, ):
    let new_sub = subItem{test:String::from("test")};
    println!("{:?}", test1::Foo(new_sub));
    cast_enum!(subItemEnum::subItem, &item.events[0]);
    println!("{:?}", cast_enum!(subItemEnum::subItem, &item.events[0]).unwrap());
}