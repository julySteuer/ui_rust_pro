use serde;
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};
use ui::hello;


//macro_rules! cast_enum {
//    ($it:path, $ret_type:expr) => {
//        if let $it(e) = $ret_type {
//            Some(e)
//        }
//        else{
//            None
//        }
//    };
//}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Base {
    subItem(subItem),
    subItemTwo(subItemTwo)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Extend {
    subItemThree(subItemThree),
    Base(Base)
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
struct subItemThree{
    test:String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    #[serde(rename = "$value")]
    events: Vec<Extend>,
}

fn main() {
    let src = r#"<Item><subItem><test>Hello</test></subItem></Item>"#;
    let item: Item = from_str(src).unwrap();
    //println!("{:?}", cast_enum!(Base::subItem, &item.events[0]).unwrap());
}