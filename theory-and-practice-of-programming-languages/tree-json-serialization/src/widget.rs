pub trait Widget: std::fmt::Debug  {
    fn add_child(&mut self, child: Box<dyn Widget>);
    fn to_binary(&self) -> String;
}


#[macro_export]
macro_rules! widget {
    ($struct_name:ident, $($field:ident: $type:ty),*) => {
        #[derive(Debug)]
        struct $struct_name {
            $( $field: $type, )*
            children: Vec<Box<dyn Widget>>,
        }

        impl $struct_name {
            fn new($($field: $type),*) -> Self {
                $struct_name { 
                    children: Vec::new(),
                    $( $field ),*
                }
            }
            
        }

        impl Widget for $struct_name {
            fn add_child(&mut self, child: Box<dyn Widget>) {
                self.children.push(child);
            }
            fn to_binary(&self) -> String {
                let mut result = String::from("");
                for child in self.children.iter() {
                    result.push_str(&child.to_binary());
                }
                result
            }
        }
    };
}
