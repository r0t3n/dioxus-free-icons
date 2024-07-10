use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{ffi::OsStr, path::Path};

use heck::ToSnakeCase;
use heck::ToUpperCamelCase;
use regex::Regex;
use scraper::node::Element;
use scraper::ElementRef;
use scraper::Html;
use walkdir::WalkDir;

const ICON_TEMPLATE: &str = r#"#[derive(Copy, Clone, Debug, PartialEq)]
pub struct {ICON_NAME};
impl IconShape for {ICON_NAME} {
    fn view_box(&self) -> &str {
        "{VIEW_BOX}"
    }
    fn width(&self) -> &str {
        "{WIDTH}"
    }
    fn height(&self) -> &str {
        "{HEIGHT}"
    }
    fn xmlns(&self) -> &str {
        "{XMLNS}"
    }
    fn fill(&self) -> &str {
        "{FILL_COLOR}"
    }
    fn stroke(&self) -> &str {
        "{STROKE_COLOR}"
    }
    fn stroke_width(&self) -> &str {
        "{STROKE_WIDTH}"
    }
    fn stroke_linecap(&self) -> &str {
        "{STROKE_LINECAP}"
    }
    fn stroke_linejoin(&self) -> &str {
        "{STROKE_LINEJOIN}"
    }
    fn child_elements(&self) -> Element {
        rsx! {
{CHILD_ELEMENTS}
        }
    }
}
"#;

pub fn create_icon_file(svg_path: &str, output_path: &str, icon_prefix: &str) {
    let files = collect_svg_files(svg_path, icon_prefix);

    let icon_file = files
        .into_iter()
        .map(|file| {
            let svg_str = fs::read_to_string(&file).unwrap();
            let fragment = Html::parse_fragment(&svg_str);

            let elements = fragment
                .tree
                .nodes()
                .filter_map(|node| {
                    if node.value().is_element() {
                        let element = ElementRef::wrap(node).unwrap().value();
                        if !element.attrs.is_empty() {
                            return Some(element);
                        }
                    }
                    None
                })
                .collect::<Vec<_>>();

            let svg_element = &elements[0];

            let svg_child_elements = &elements[1..];
            let icon_name = icon_name(&file, icon_prefix);
            let (view_box, xmlns) = extract_svg_attrs(svg_element);
            let (width, height) = extract_svg_dimensions(svg_element);
            let child_elements = extract_svg_child_elements(svg_child_elements, icon_prefix);
            let (fill_color, stroke_color, stroke_width) = extract_svg_colors(svg_element);
            let stroke_linecap = extract_svg_stroke_linecap(svg_element);
            let stroke_linejoin = extract_svg_stroke_linejoin(svg_element);

            ICON_TEMPLATE
                .replace("{ICON_NAME}", &format!("{}{}", icon_prefix, &icon_name))
                .replace("{VIEW_BOX}", &view_box)
                .replace("{WIDTH}", &width)
                .replace("{HEIGHT}", &height)
                .replace("{XMLNS}", &xmlns)
                .replace("{CHILD_ELEMENTS}", &child_elements)
                .replace("{FILL_COLOR}", &fill_color)
                .replace("{STROKE_COLOR}", &stroke_color)
                .replace("{STROKE_WIDTH}", &stroke_width)
                .replace("{STROKE_LINECAP}", &stroke_linecap)
                .replace("{STROKE_LINEJOIN}", &stroke_linejoin)
        })
        .collect::<Vec<_>>()
        .join("\n");

    // write to file
    let mut file = File::create(output_path).unwrap();
    file.write_all(
        format!(
            "{}\n\n{}",
            "use super::super::IconShape;\nuse dioxus::prelude::*;", icon_file
        )
        .as_bytes(),
    )
    .unwrap();
    file.flush().unwrap();
}

fn collect_svg_files(svg_path: &str, icon_prefix: &str) -> Vec<PathBuf> {
    let dir_entries = WalkDir::new(svg_path)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .collect::<Vec<_>>();

    dir_entries
        .into_iter()
        .filter(|e| match icon_prefix {
            "Go" => {
                let re = Regex::new(r".*-16.svg$").unwrap();
                return re.is_match(e.path().to_str().unwrap());
            }
            "Md" => {
                let split_vec = e.path().components().collect::<Vec<_>>();
                return split_vec.iter().any(|c| c.as_os_str() == "materialicons")
                    && e.file_name().to_str().unwrap() == "24px.svg";
            }
            _ => return e.path().extension() == Some(OsStr::new("svg")),
        })
        .map(|dir| PathBuf::from(dir.path()))
        .collect::<Vec<_>>()
}

fn icon_name(path: &Path, icon_prefix: &str) -> String {
    match icon_prefix {
        "Go" => {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let name = filename.split('.').next().unwrap();
            name.replace("-16", "").to_upper_camel_case()
        }
        "Md" => {
            let split_vec = path.components().collect::<Vec<_>>();
            let name = split_vec[split_vec.len() - 3];
            name.as_os_str().to_str().unwrap().to_upper_camel_case()
        }
        _ => {
            let filename = path.file_name().unwrap().to_str().unwrap();
            let name = filename.split('.').next().unwrap();
            name.to_upper_camel_case()
        }
    }
}

fn extract_svg_attrs(element: &Element) -> (&str, &str) {
    (
        element.attr("viewBox").unwrap(),
        element.attr("xmlns").unwrap_or("http://www.w3.org/2000/svg")
    )
}

fn extract_svg_colors(element: &Element) -> (&str, &str, &str) {
    (
        element.attr("fill").unwrap_or("black"),
        element.attr("stroke").unwrap_or("none"),
        element.attr("stroke-width").unwrap_or("1"),
    )
}

fn extract_svg_dimensions(element: &Element) -> (&str, &str) {
    (
        element.attr("width").unwrap_or("300"),
        element.attr("height").unwrap_or("150"),
    )
}

fn extract_svg_stroke_linecap(element: &Element) -> &str {
    element.attr("stroke-linecap").unwrap_or("butt")
}

fn extract_svg_stroke_linejoin(element: &Element) -> &str {
    element.attr("stroke-linejoin").unwrap_or("miter")
}

fn extract_svg_child_elements(elements: &[&Element], icon_prefix: &str) -> String {
    let elements = match icon_prefix {
        "Md" => &elements[1..],
        _ => elements,
    };
    elements
        .iter()
        .map(|element| {
            let tag_name = element.name();
            let mut element_attrs = element
                .attrs()
                .filter_map(|(name, value)| {
                    let value = if icon_prefix == "Io" {
                        value.replace("fill:none;stroke:#000;", "")
                    } else {
                        value.to_string()
                    };
                    let re = Regex::new(r"^data-.*$").unwrap();
                    if !re.is_match(name) && name != "fill" {
                        Some(format!(
                            "                {}: \"{}\",",
                            name.to_snake_case(),
                            value
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            element_attrs.sort();
            let attrs_str = element_attrs.join("\n");
            "            {TAG_NAME} {\n{ATTRS}\n            }"
                .replace("{TAG_NAME}", tag_name)
                .replace("{ATTRS}", &attrs_str)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
