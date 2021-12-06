use serde;
use serde_derive::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

trait subItemtrait {

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct subItem {
    test:String
}

impl subItemtrait for subItem {

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
    subItem: subItemtrait
}


#[derive(Debug, Deserialize)]
pub struct RowValue {
    pub value: i32,
}

fn main() {
    let src = r#"<Item><subItem><test>Hello</test></subItem><name>Banana</name><source>Store</source></Item>"#;

    let item: Item = from_str(src).unwrap();
    println!("{}", item.subItem.test);
}