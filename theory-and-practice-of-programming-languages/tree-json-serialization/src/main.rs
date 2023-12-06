#![allow(dead_code)]
mod widget;
use regex::Regex;
pub use widget::Widget;
pub use std::any::type_name;
use std::fmt::Debug;

enum Alignment {
    Horizontal = 1, 
    Vertical = 2
}

impl Debug for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Horizontal => write!(f, "1"),
            Self::Vertical => write!(f, "2")
        }
    }
}

widget!(Window, title: String);
widget!(Layout, alignment: Alignment);
widget!(LineEdit, max_length: u8);
widget!(ComboBox, items: Vec<String>);

fn parse_attrs(_: &String)  {
    todo!();
}

fn from_binary(source: &String) -> Box<dyn Widget> {
    let re = Regex::new(r#"\{"name":"(\w+)",((?:"(?:\w+)":(?:"?[\w !?.]+"?),)+)"children":\[(\{.*\})?\]\}"#).unwrap();
    if let Some(capt) = re.captures(source) {
        let struct_name = &capt[1];
        let attrs = &capt[2];
        if let Some(children) = capt.get(3) {
            println!("{}", &children.as_str());
            from_binary(&children.as_str().to_string());
        }

        println!("{}", struct_name);

    }


    Box::new(Window::new("Hello world!".to_string()))
}


fn main() {
    let mut app = Window::new("Hello World!".to_string());

    // /{"name":"(?<struct>\w+)",(?<attrs>(?:"(?:\w+)":(?:"?.+"?),)+)"children":\[(?<children>{.*})\]}/gm
    let v_layout = Layout::new(Alignment::Vertical);
    let h_layout = Layout::new(Alignment::Horizontal);

    // let mut edit = LineEdit::new(20);
    // let combo_box = ComboBox::new(Vec::from(["a".to_string(), "b".to_string()]));

    // edit.add_child(Box::new(combo_box));
    // v_layout.add_child(Box::new(edit));

    app.add_child(Box::new(v_layout));
    app.add_child(Box::new(h_layout));
    
    println!("{}", app.to_binary());
    from_binary(&mut app.to_binary());
}
