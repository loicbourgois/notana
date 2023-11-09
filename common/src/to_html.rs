use crate::Element;
use crate::ElementData;
use crate::ListElement;
use crate::Page;
use crate::Text;
use crate::Title;
impl Title {
    pub fn to_html(&self, level: usize) -> String {
        let level_str = level + 1;
        return format!("<h{level_str}>{}</h{level_str}>", self.txt.clone());
    }
}
impl Text {
    pub fn to_html(&self, indent: usize) -> String {
        return format!("<p class='indent_{indent}'>{}</p>", self.txt.clone());
    }
}
impl Page {
    pub fn to_html(&self) -> String {
        return format!("<h1>{}</h1>", self.title.clone().unwrap());
    }
}
impl ListElement {
    pub fn to_html(&self, indent: usize) -> String {
        return format!("<p class='indent_{indent}'>{}</p>", self.txt.clone());
    }
}
impl ElementData {
    pub fn to_html(&self, level: usize, indent: usize) -> String {
        match self {
            ElementData::Page(x) => x.to_html(),
            ElementData::Title(x) => x.to_html(level),
            ElementData::Text(x) => x.to_html(indent),
            ElementData::ListElement(x) => x.to_html(indent),
            _ => String::from("-"),
        }
    }
}
// ▲▼◀▼
// ↓↑→←
impl Element {
    pub fn to_html(&self, level: usize, indent: usize) -> String {
        let child_indent = match self.data {
            ElementData::Text(_) => indent + 1,
            ElementData::ListElement(_) => indent + 1,
            _ => indent,
        };
        let child_md = self
            .childs
            .iter()
            .map(|x| x.to_html(level + 1, child_indent))
            .collect::<Vec<_>>()
            .join("\n");
        let button_show_hide = match &self.data {
            ElementData::Title(x) => String::from("<button>▼</button>"),
            _ => String::from("<button></button>"),
        };
        let button_left = match &self.data {
            ElementData::Text(x) => String::from("<button>←</button>"),
            ElementData::ListElement(x) => String::from("<button>←</button>"),
            _ => String::from("<button></button>"),
        };
        let button_right = match &self.data {
            ElementData::Text(x) => String::from("<button>→</button>"),
            ElementData::ListElement(x) => String::from("<button>→</button>"),
            _ => String::from("<button></button>"),
        };
        let element_holder = format!(
            "<div class='element'>{button_left}{button_right}{button_show_hide}{}</div>",
            self.data.to_html(level, indent)
        );
        let childs_class = match &self.data {
            _ => String::from(""),
        };
        let main_html = if self.childs.len() > 0 {
            format!("{element_holder}<div class='{childs_class}'>{child_md}</div>")
        } else {
            element_holder
        };
        match &self.data {
            ElementData::Page(data) => {
                return {
                    format!(
                        r#"
                    <div id='left'>
                        <p>Outline</p>
                    </div>
                    <div id='center'>
                        {main_html}
                    </div>
                    <div id='right'>
                        <p>Comments</p>
                    </div>
                "#
                    )
                }
            }
            // main_html,
            _ => main_html,
        }
    }
}
