use askama::Template;


mod manage_page;

pub use manage_page::ControlPage;
pub use manage_page::ProductList;

#[macro_export]
macro_rules! user_action_new {
    ( $(( $name:expr, $url:expr  )), * $(,)?) => {
        UserActionTemplate::new(
            vec![
                $(
                    ActionItem::new($url, $name)
                ), *
            ]
        )
    };
}


#[derive(Debug, Template)]
#[template(path="user_action.html")]
pub struct UserActionTemplate{
    items: Vec<ActionItem>
}

impl UserActionTemplate{
    pub fn new(items: Vec<ActionItem>) -> Self {
        Self { items }
    }
}

#[derive(Debug)]
pub struct ActionItem{
    url: &'static str,
    title: &'static str
}

impl ActionItem {
    pub fn new(url: &'static str, title: &'static str) -> Self{
        Self { url, title }
    }
}


mod filters{

    pub fn as_id<T: std::fmt::Display>(
        s: T,
        _: &dyn askama::Values,
    ) -> askama::Result<String> {
        let s = s.to_string();
        Ok(s.replace(" ", "-").to_lowercase())
    }

}
