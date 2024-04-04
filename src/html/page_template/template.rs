use crate::html::HtmlNode;

use super::page_template_config::PageTemplateConfig;
use crate::html::page_template::{Link, Meta};

fn generate_scripts(scripts: Vec<String>) -> String {
    scripts
        .iter()
        .map(|src| format!("<script src=\"{}\"></script>", src.escape_default()))
        .collect()
}

fn generate_meta(meta_tags: Vec<Meta>) -> String {
    meta_tags.iter().map(|meta| meta.dump_html()).collect()
}

fn generate_links(links: Vec<Link>) -> String {
    links.iter().map(|link| link.dump_html()).collect()
}

pub fn render_page(config: PageTemplateConfig, node: HtmlNode) -> String {
    [
        "<!DOCTYPE html>".to_string(),
        match config.language {
            Some(lang) => format!("<html lang=\"{}\">", lang.escape_default()),
            None => "<html>".to_string(),
        },
        format!("<title>{}</title>", config.title.escape_default()),
        generate_meta(config.meta_vec),
        generate_links(config.links),
        "</head><body>".to_string(),
        node.dump_html(),
        generate_scripts(config.scripts),
        "</body></html>".to_string(),
    ].join("")
}
