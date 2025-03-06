use std::fmt::Display;

pub struct HTMLTag {
    opening_tag: String,
    closing_tag: String,
    content: String,
}

impl HTMLTag {
    pub fn new(tag_name: &str, content: &str) -> HTMLTag {
        let opening_tag = generate_opening_tag(tag_name);
        let closing_tag = generate_closing_tag(tag_name);

        HTMLTag {
            opening_tag,
            closing_tag,
            content: content.to_string(),
        }
    }
}

impl Display for HTMLTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}", self.opening_tag, self.content, self.closing_tag)
    }
}

fn generate_opening_tag(tag_name: &str) -> String {
    format!("<{}>", tag_name)
}

fn generate_closing_tag(tag_name: &str) -> String {
    format!("</{}>", tag_name)
}
