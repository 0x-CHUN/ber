extern crate getopts;
extern crate image;

use std::fs::File;
use std::io::{BufWriter, Read};
use image::Rgba;

pub mod dom;
pub mod html;
pub mod css;
pub mod style;
pub mod layout;
pub mod painting;
pub mod pdf;

fn main() {
    let mut opts = getopts::Options::new();
    opts.optopt("h", "html", "HTML document", "FILENAME");
    opts.optopt("c", "css", "CSS stylesheet", "FILENAME");
    opts.optopt("o", "output", "Output file", "FILENAME");
    opts.optopt("f", "format", "Output file format", "png | pdf");

    let matches = opts.parse(std::env::args().skip(1)).unwrap();
    let str_arg = |flag: &str, default: &str| -> String {
        matches.opt_str(flag).unwrap_or(default.to_string())
    };

    let png = match &str_arg("f", "png")[..] {
        "png" => true,
        "pdf" => false,
        x => panic!("Unknown output format: {}", x),
    };

    let html = read(str_arg("h", "tests/perf-rainbow.html"));
    let css = read(str_arg("c", "tests/perf-rainbow.css"));

    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 8000.0;
    viewport.content.height = 6000.0;

    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);

    let filename = str_arg("o", if png { "output.png" } else { "output.pdf" });
    let mut file = BufWriter::new(File::create(&filename).unwrap());

    let ok = if png {
        let canvas = painting::paint(&layout_root, viewport.content);
        let (w, h) = (canvas.width as u32, canvas.height as u32);
        let img = image::ImageBuffer::from_fn(w, h, move |x, y| {
            let color = canvas.pixels[(y * w + x) as usize];
            Rgba([color.r, color.g, color.b, color.a])
        });
        image::DynamicImage::ImageRgba8(img).save_with_format(filename.clone(), image::ImageFormat::Png).is_ok()
    } else {
        pdf::render(&layout_root, viewport.content, &mut file).is_ok()
    };
    if ok {
        println!("Saved output as {}", filename)
    } else {
        println!("Error saving output as {}", filename)
    }
}

fn read(filename: String) -> String {
    let mut content = String::new();
    File::open(filename).unwrap().read_to_string(&mut content).unwrap();
    content
}
