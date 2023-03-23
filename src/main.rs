use std::fs::File;
use std::io::Read;

pub mod dom;
pub mod html;
pub mod css;

fn main() {
    let html_content = read("./tests/test.html".to_string());
    let root_node = html::parse(html_content);
    assert_eq!(root_node.children.len(), 2);
    assert_eq!(root_node.children.get(0).unwrap().children.len(), 1);
    assert_eq!(root_node.children.get(1).unwrap().children.len(), 2);

    let css_content = read("./tests/test.css".to_string());
    let style = css::parse(css_content);
    assert_eq!(style.rules.len(), 8);
}

fn read(filename: String) -> String {
    let mut content = String::new();
    File::open(filename).unwrap().read_to_string(&mut content).unwrap();
    content
}
