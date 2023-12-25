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
                let mut result = String::new();

                result.push_str(&format!("{{\"name\":\"{}\",", stringify!($struct_name)));
                $( 
                    result.push_str(&format!("\"{}\":{:?},", stringify!($field), self.$field)); 
                )*
                result.push_str(&format!("\"children\":[{}]}}", 
                    &self.children.iter()
                        .map(|it| it.to_binary())
                        .collect::<Vec<String>>()
                        .join(",")
                ));
                result
            }
        }
    };
}
