pub trait EscapeHtml {
    fn escape_html(&self) -> String;
}

impl EscapeHtml for str {
    fn escape_html(&self) -> String {
        self.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }
}
