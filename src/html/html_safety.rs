// Copyright 2024 Jakub Duda, Lech Mazur
//
// This file is part of Poietic.
//
// Poietic is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License, version 2,
// as published by the Free Software Foundation.
//
// Poietic is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Poietic. If not, see <https://www.gnu.org/licenses/>.

use std::{collections::HashSet, sync::OnceLock};

const SAFE_HTML_TAGS: &[&str] = &[
    "a",
    "abbr",
    "acronym",
    "address",
    "area",
    "article",
    "aside",
    "b",
    "bdi",
    "bdo",
    "big",
    "blockquote",
    "br",
    "button",
    "caption",
    "cite",
    "code",
    "col",
    "colgroup",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "div",
    "dl",
    "dt",
    "em",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hgroup",
    "hr",
    "i",
    "img",
    "input",
    "ins",
    "kbd",
    "label",
    "legend",
    "li",
    "main",
    "map",
    "mark",
    "meter",
    "nav",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "picture",
    "pre",
    "progress",
    "q",
    "rp",
    "rt",
    "ruby",
    "s",
    "samp",
    "section",
    "select",
    "small",
    "span",
    "strong",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "tr",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
];

pub const ATTRIBUTE_BLACKLIST: &[&str] = &[
    "async",
    "crossorigin",
    "formaction",
    "onabort",
    "onblur",
    "oncanplay",
    "oncanplaythrough",
    "onchange",
    "onclick",
    "oncontextmenu",
    "ondblclick",
    "ondrag",
    "ondragend",
    "ondragenter",
    "ondragleave",
    "ondragover",
    "ondragstart",
    "ondrop",
    "ondurationchange",
    "onemptied",
    "onended",
    "onerror",
    "onfocus",
    "onformchange",
    "onforminput",
    "oninput",
    "oninvalid",
    "onkeydown",
    "onkeypress",
    "onload",
    "onloadeddata",
    "onloadedmetadata",
    "onloadstart",
    "onmousedown",
    "onmouseenter",
    "onmouseleave",
    "onmousemove",
    "onmouseout",
    "onmouseover",
    "onmouseup",
    "onmousewheel",
    "onpause",
    "onplay",
    "onplaying",
    "onpointercancel",
    "onpointerdown",
    "onpointerenter",
    "onpointerleave",
    "onpointerlockchange",
    "onpointererror",
    "onpointermove",
    "onpointerout",
    "onpointerover",
    "onpointerup",
    "onprogress",
    "onratechange",
    "onreadystatechange",
    "onreset",
    "onresize",
    "onscroll",
    "onseeked",
    "onseeking",
    "onselect",
    "onshow",
    "onstalled",
    "onsubmit",
    "onsuspend",
    "ontimeupdate",
    "onvolumechange",
    "onwaiting",
];

static SAFE_HTML_TAG_SET: OnceLock<HashSet<&str>> = OnceLock::new();
static ATTRIBUTE_BLACKLIST_SET: OnceLock<HashSet<&str>> = OnceLock::new();

pub fn get_safe_html_tag_set() -> &'static HashSet<&'static str> {
    SAFE_HTML_TAG_SET.get_or_init(|| SAFE_HTML_TAGS.iter().cloned().collect())
}

pub fn get_attribute_blacklist() -> &'static HashSet<&'static str> {
    ATTRIBUTE_BLACKLIST_SET.get_or_init(|| ATTRIBUTE_BLACKLIST.iter().cloned().collect())
}

pub const ILLEGAL_HTML_ATTRIBUTE_NAME_CHARACTERS: &[char] = &['\0', '\'', '"', '<', '>', '/', '='];

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

#[cfg(test)]
mod test {
    use crate::html::html_safety::EscapeHtml;

    #[test]
    fn html_injection() {
        let text = "<div>\"Evil&Injection\'</div>";
        assert_eq!("&lt;div&gt;&quot;Evil&amp;Injection&#39;&lt;/div&gt;", text.escape_html());
    }
}
