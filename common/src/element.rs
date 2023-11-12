use serde::Deserialize;
use serde::Serialize;
pub type PageTitle = String;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Element {
    pub data: ElementData,
    pub childs: Vec<Element>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ElementData {
    Page(PageElement),
    Title(Title),
    Text(Text),
    ListElement(ListElement),
    Comment(Comment),
    CommentSection,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Title {
    pub txt: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Text {
    pub txt: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListElement {
    pub txt: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageElement {
    pub title: Option<PageTitle>,
    pub comments: Option<Box<Element>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub line: Option<String>,
    pub user: String,
    pub text: Vec<String>,
}
