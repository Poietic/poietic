use crate::html::{html_safety::EscapeHtml, HtmlNode};

use super::page_template_config::PageTemplateConfig;
use crate::html::page_template::{Link, Meta};
use std::fmt::Write;

fn generate_scripts(scripts: Vec<String>) -> String {
    scripts.iter().fold(String::new(), |mut output, src| {
        let _ = write!(
            output,
            "<script src=\"{}\"></script>",
            src.escape_html()
        );
        output
    })
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
            Some(lang) => format!("<html lang=\"{}\">", (&lang as &str).escape_html()),
            None => "<html>".to_string(),
        },
        "<head>".to_string(),
        format!("<title>{}</title>", (&config.title as &str).escape_html()),
        generate_meta(config.meta_vec),
        generate_links(config.links),
        "</head><body>".to_string(),
        node.dump_html(),
        generate_scripts(config.scripts),
        "</body></html>".to_string(),
    ]
    .join("")
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use crate::html::page_template::page_template_config::PageTemplateConfigBuilder;

    use super::*;

    #[test]
    fn script_generation() {
        let scripts = vec![
            "foo.js".to_string(),
            "bar.js".to_string(),
            "poietic.js".to_string(),
        ];

        let result = generate_scripts(scripts);

        assert_eq!(
            &result,
            r#"<script src="foo.js"></script><script src="bar.js"></script><script src="poietic.js"></script>"#
        )
    }

    #[test]
    fn meta_generation() {
        let meta_tags = vec![Meta::new(BTreeMap::from([(
            "charset".to_string(),
            "UTF-8".to_string(),
        )]))
        .unwrap()];

        let result = generate_meta(meta_tags);

        assert_eq!(&result, r#"<meta charset="UTF-8"/>"#)
    }

    #[test]
    fn link_generation() {
        let links = vec![Link::new(BTreeMap::from([
            ("rel".to_string(), "stylesheet".to_string()),
            ("href".to_string(), "styles.css".to_string()),
        ]))
        .unwrap()];

        let result = generate_links(links);

        assert_eq!(&result, r#"<link href="styles.css" rel="stylesheet"/>"#)
    }

    #[test]
    fn page_render() {
        let config = PageTemplateConfigBuilder::new()
            .language("en".to_string())
            .title("Foo Bar".to_string())
            .charset()
            .add_stylesheet("styles.css".to_string())
            .add_script("main.js".to_string())
            .build();
        let node = HtmlNode::new("p".to_string(), [].into(), [].into()).unwrap();

        let render = render_page(config, node);

        assert_eq!(
            &render,
            r#"<!DOCTYPE html><html lang="en"><head><title>Foo Bar</title><meta charset="UTF-8"/><link href="styles.css" rel="stylesheet" type="text/css"/></head><body><p></p><script src="main.js"></script></body></html>"#
        );
    }
}
