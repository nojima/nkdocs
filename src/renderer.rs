pub trait Renderer {
    fn render_paragraph(&self, paragraph: &str) -> String;
}

pub struct HtmlRenderer {}

impl Renderer for HtmlRenderer {
    fn render_paragraph(&self, paragraph: &str) -> String {
        let mut ret = String::from("<p>");
        ret.push_str(&escape_html(paragraph));
        ret.push_str("</p>\n");
        ret
    }
}

fn escape_html(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("\"", "&quot;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}
