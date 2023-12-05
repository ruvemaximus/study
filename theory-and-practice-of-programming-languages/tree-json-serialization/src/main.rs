#![allow(dead_code)]
mod widget;
pub use widget::Widget;

#[derive(Debug)]
enum Alignment {
    Horizontal, 
    Vertical
}

widget!(Window, title: String);
widget!(Layout, alignment: Alignment);
widget!(LineEdit, max_length: u8);
widget!(ComboBox, items: Vec<String>);

fn main() {
    let mut app = Window::new("Hello World!".to_string());

    let mut v_layout = Layout::new(Alignment::Vertical);
    let h_layout = Layout::new(Alignment::Horizontal);

    let mut edit = LineEdit::new(20);
    let combo_box = ComboBox::new(Vec::from(["a".to_string(), "b".to_string()]));

    edit.add_child(Box::new(combo_box));
    v_layout.add_child(Box::new(edit));

    app.add_child(Box::new(v_layout));
    app.add_child(Box::new(h_layout));

    dbg!(app.to_binary());
}
