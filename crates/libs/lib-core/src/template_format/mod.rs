use askama::Template;

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

