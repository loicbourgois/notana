use crate::Comment;
// use crate::CommentSection;
use crate::Element;
use crate::ElementData;
use crate::ListElement;
use crate::PageElement;
use crate::Text;
use crate::Title;
impl ElementData {
    pub fn to_md(&self, level: usize, indent: usize) -> String {
        match self {
            ElementData::Page(x) => x.to_md(),
            ElementData::Title(x) => x.to_md(level),
            ElementData::Text(x) => x.to_md(indent),
            ElementData::ListElement(x) => x.to_md(indent),
            ElementData::Comment(x) => x.to_md(level),
            ElementData::CommentSection => "".to_string(),
        }
    }
}
impl Title {
    pub fn to_md(&self, level: usize) -> String {
        let mut level_str = String::from("#");
        for _ in 0..level {
            level_str = format!("#{level_str}");
        }
        return format!("\n{level_str} {}", self.txt.clone());
    }
}
impl Text {
    pub fn to_md(&self, indent: usize) -> String {
        let mut indent_str = String::from("");
        for _ in 0..indent {
            indent_str = format!("    {indent_str}");
        }
        return format!("{indent_str}{}", self.txt.clone());
    }
}
impl PageElement {
    pub fn to_md(&self) -> String {
        return format!("# {}", self.title.clone().unwrap());
    }
}
impl ListElement {
    pub fn to_md(&self, indent: usize) -> String {
        let mut indent_str = String::from("");
        for _ in 0..indent {
            indent_str = format!("    {indent_str}");
        }
        return format!("{indent_str}- {}", self.txt.clone());
    }
}
impl Comment {
    pub fn to_md(&self, level: usize) -> String {
        let mut level_str = String::from("");
        for _ in 1..level {
            level_str = format!("  {level_str}");
        }
        let text_str = self
            .text
            .iter()
            .map(|l| format!("  {level_str}{l}"))
            .collect::<Vec<_>>()
            .join("\n");
        let line_str = match &self.line {
            Some(x) => format!("l{x} "),
            None => "".to_string(),
        };
        return format!("{level_str}- {line_str}@{}\n{text_str}", self.user.clone());
    }
}
pub fn childs_to_md(childs: &Vec<Element>, level: usize, child_indent: usize) -> String {
    childs
        .iter()
        .map(|x| x.to_md(level + 1, child_indent))
        .collect::<Vec<_>>()
        .join("\n")
}
impl Element {
    pub fn to_md(&self, level: usize, indent: usize) -> String {
        let child_indent = match self.data {
            ElementData::Text(_) => indent + 1,
            ElementData::ListElement(_) => indent + 1,
            _ => indent,
        };
        let comments = match &self.data {
            ElementData::Page(page_data) => {
                let comments_md = childs_to_md(
                    &page_data.comments.clone().unwrap().childs,
                    level,
                    child_indent,
                );
                // let aa = format!("{:?}", comments);
                format!("\n\n# Comments\n{comments_md}").to_string()
            }
            _ => "".to_string(),
        };
        let child_md = childs_to_md(&self.childs, level, child_indent);
        if self.childs.len() > 0 {
            return format!("{}\n{}{comments}", self.data.to_md(level, indent), child_md);
        } else {
            return format!("{}{comments}", self.data.to_md(level, indent));
        }
    }
}
