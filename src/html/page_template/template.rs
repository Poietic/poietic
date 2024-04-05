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
        "<head>".to_string(),
        format!("<title>{}</title>", config.title.escape_default()),
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
    use std::collections::HashMap;

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
        let meta_tags = vec![Meta::new(HashMap::from([(
            "charset".to_string(),
            "UTF-8".to_string(),
        )]))
        .unwrap()];

        let result = generate_meta(meta_tags);

        assert_eq!(
            &result,
            r#"<meta charset="UTF-8"/><meta content="width=device-width, initial-scale=1.0" name="viewport" />"#
        )
    }

    #[test]
    fn link_generation() {
        let links = vec![Link::new(HashMap::from([
            ("rel".to_string(), "stylesheet".to_string()),
            ("href".to_string(), "styles.css".to_string()),
        ]))
        .unwrap()];

        let result = generate_links(links);

        assert!(
            &result == r#"<link rel="stylesheet" href="styles.css"/>"#
                || &result == r#"<link href="styles.css" rel="stylesheet"/>"#
        )
    }

    #[test]
    fn page_render() {
        let config = PageTemplateConfigBuilder::new()
            .language("en".to_string())
            .title("Foo Bar".to_string())
            .charset()
            .add_stylesheet("styles.css".to_string())
            .add_script("main.js".to_string())
            .build()
            .unwrap();
        let node = HtmlNode::new("p".to_string(), [].into(), [].into()).unwrap();

        let render = render_page(config, node);

        assert_eq!(
            &render,
            r#"<!DOCTYPE html><html lang="en"><head><title>Foo Bar</title><meta charset="UTF-8"/><link type="text/css" rel="stylesheet" href="styles.css"/></head><body><p></p><script src="main.js"></script></body></html>"#
        );
    }

    #[test]
    fn script_html_injection() {
        let evil_script_src =
            r#""></script><div><h1>Wordpress is better than Poietic!</h1></div><script src=""#;

        let result = generate_scripts(vec![evil_script_src.to_string()]);

        assert_eq!(
            &result,
            r#"<script src="&quot;></script><div><h1>Wordpress is better than Poietic!</h1></div>< src=&quot;"></script>"#
        );
    }

    #[test]
    fn meta_script_injection() {
        let evil_meta_attribute =
            r#""/><title>Wordpress is better than Poietic!</title><meta name=""#;

        let result = generate_meta(vec![Meta::new(HashMap::from([(
            "name".to_string(),
            evil_meta_attribute.to_string(),
        )]))
        .unwrap()]);

        assert_eq!(
            &result,
            r#"<meta name="&quot/><title>Wordpress is better than Poietic!</title><meta name=&quot;"#
        );
    }

    #[test]
    fn link_script_injection() {
        let evil_link_attribute =
            r#""/><title>Wordpress is better than Poietic!</title><link rel=""#;

        let result = generate_links(vec![Link::new(HashMap::from([(
            "rel".to_string(),
            evil_link_attribute.to_string(),
        )]))
        .unwrap()]);

        assert_eq!(
            &result,
            r#"&quot;/><title>Wordpress is better than Poietic!</title><link rel=&quot;"#
        );
    }

    #[test]
    fn page_render_injection() {
        let config_with_injections = PageTemplateConfigBuilder::new()
            .language(r#"en"><body><h1>Evil injection in language</h1></body></html><!-- "#.to_string())
            .title(r#"Foo Bar</title><head><body><h1>Evil injection in title</h1>"#.to_string())
            .build()
            .unwrap();
        let node = HtmlNode::new("p".to_string(), [].into(), [].into()).unwrap();

        let render = render_page(config_with_injections, node);

        assert_eq!(
            &render,
            r#"<!DOCTYPE html><html lang="en&quot;><body><h1>Evil injection in language</h1></body></html><!--"><head><title>Foo Bar&lt;/title&gt;&lt;head&gt;&lt;body&gt;&lt;h1&gt;Evil injection in title&lt;/h1&gt;</title></head><body><p></p><script src="main.js"></script></body></html>"#
        );
    }
}
