#![allow(dead_code)]
use std::fmt::Debug;
use regex::Regex;

mod widget;
pub use widget::Widget;



enum Alignment {
    Horizontal,
    Vertical
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


fn from_binary(source: &String) -> Box<dyn Widget> {
    let re = Regex::new(r#"\{"name":"(\w+)",((?:"\w+":[\w\s\[\]",]+,)+)"children":\[(\{.*\})?\]\}"#).unwrap();
    let capt = re.captures(source).unwrap();

    let name = &capt[1];
    let attrs = &capt[2];
    let mut widget: Box<dyn Widget> = match name {
        "Window" => {
            let re = Regex::new(r#"^"title":"([\w\s]+)",$"#).unwrap();
            let (_, [title]) = re.captures(attrs).unwrap().extract();

            Box::new(Window::new(title.to_string()))
        }
        "Layout" => {
            let re = Regex::new(r#"^"alignment":(\d+),$"#).unwrap();
            let (_, [alignment]) = re.captures(attrs).unwrap().extract();

            let alignment = match alignment {
                "1" => Alignment::Horizontal,
                "2" => Alignment::Vertical,
                _ => unreachable!()
            };
        
            Box::new(Layout::new(alignment))
        }
        "LineEdit" => {
            let re = Regex::new(r#"^"max_length":(\d+),$"#).unwrap();
            let (_, [max_length]) = re.captures(attrs).unwrap().extract(); 
        
            Box::new(LineEdit::new(max_length.parse().unwrap()))
        }
        "ComboBox" => {
            let re = Regex::new(r#"^"items":\[(.*)\],$"#).unwrap();
            let (_, [items]) = re.captures(attrs).unwrap().extract();

            let re_for_replace = Regex::new(r#"[" ]"#).unwrap();

            let items = items
                .split(',')
                .map(|s| { re_for_replace.replace_all(s, "").to_string() })
                .collect();
        
            Box::new(ComboBox::new(items))
        }
        _ => { panic!("Неизвестный виджет") }
    };

    if let Some(children) = capt.get(3) {
        for child in re.find_iter(&children.as_str()).map(|m| m.as_str()) {
            widget.add_child(from_binary(&child.to_string()));
        }
    }
    widget
}


fn main() {
    let mut app = Window::new("Example title".to_string());

    let mut v_layout = Layout::new(Alignment::Vertical);
    let h_layout = Layout::new(Alignment::Horizontal);

    let mut edit = LineEdit::new(20);
    let combo_box = ComboBox::new(Vec::from(["a".to_string(), "b".to_string()]));

    edit.add_child(Box::new(combo_box));
    v_layout.add_child(Box::new(edit));

    app.add_child(Box::new(v_layout));
    app.add_child(Box::new(h_layout));

    
    dbg!(&app);
    dbg!(from_binary(&mut app.to_binary()));

    assert_eq!(
        &app.to_binary(),
        &from_binary(&mut app.to_binary()).to_binary()
    )
}
