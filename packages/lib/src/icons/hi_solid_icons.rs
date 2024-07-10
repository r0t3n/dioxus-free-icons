use super::super::IconShape;
use dioxus::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiAcademicCap;
impl IconShape for HiAcademicCap {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.3939 2.08085C10.1424 1.97305 9.85763 1.97305 9.60608 2.08085L2.60608 5.08085C2.2384 5.23843 2 5.59997 2 6C2 6.40003 2.2384 6.76157 2.60608 6.91915L5.25037 8.05241C5.34653 7.94347 5.46706 7.85473 5.60608 7.79515L9.60608 6.08087C10.1137 5.86331 10.7016 6.09846 10.9191 6.60609C11.1367 7.11372 10.9015 7.7016 10.3939 7.91916L7.66668 9.08797L9.60608 9.91915C9.85763 10.027 10.1424 10.027 10.3939 9.91915L17.3939 6.91915C17.7616 6.76157 18 6.40003 18 6C18 5.59997 17.7616 5.23843 17.3939 5.08085L10.3939 2.08085Z",
            }
            path {
                d: "M3.31004 9.39673L5 10.121V14.2226C4.65723 14.1449 4.30705 14.0867 3.95071 14.0494C3.48094 14.0001 3.1097 13.6289 3.06044 13.1591C3.02046 12.7778 3 12.391 3 11.9998C3 11.1033 3.10741 10.2315 3.31004 9.39673Z",
            }
            path {
                d: "M9.29996 16.5725C8.62708 15.9129 7.85167 15.3584 7 14.9351V10.9781L8.81824 11.7574C9.57289 12.0808 10.4271 12.0808 11.1818 11.7574L16.69 9.39673C16.8926 10.2315 17 11.1033 17 11.9998C17 12.391 16.9795 12.7778 16.9396 13.1591C16.8903 13.6289 16.5191 14.0001 16.0493 14.0494C13.9765 14.2667 12.1124 15.188 10.7 16.5725C10.3112 16.9537 9.68881 16.9537 9.29996 16.5725Z",
            }
            path {
                d: "M6 18C6.55228 18 7 17.5523 7 17V14.9351C6.37136 14.6227 5.70117 14.3817 5 14.2226V17C5 17.5523 5.44772 18 6 18Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiAdjustments;
impl IconShape for HiAdjustments {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 4C5 3.44772 4.55228 3 4 3C3.44772 3 3 3.44772 3 4V11.2676C2.4022 11.6134 2 12.2597 2 13C2 13.7403 2.4022 14.3866 3 14.7324V16C3 16.5523 3.44772 17 4 17C4.55228 17 5 16.5523 5 16V14.7324C5.5978 14.3866 6 13.7403 6 13C6 12.2597 5.5978 11.6134 5 11.2676V4Z",
            }
            path {
                d: "M11 4C11 3.44772 10.5523 3 10 3C9.44772 3 9 3.44772 9 4V5.26756C8.4022 5.61337 8 6.25972 8 7C8 7.74028 8.4022 8.38663 9 8.73244V16C9 16.5523 9.44772 17 10 17C10.5523 17 11 16.5523 11 16V8.73244C11.5978 8.38663 12 7.74028 12 7C12 6.25972 11.5978 5.61337 11 5.26756V4Z",
            }
            path {
                d: "M16 3C16.5523 3 17 3.44772 17 4V11.2676C17.5978 11.6134 18 12.2597 18 13C18 13.7403 17.5978 14.3866 17 14.7324V16C17 16.5523 16.5523 17 16 17C15.4477 17 15 16.5523 15 16V14.7324C14.4022 14.3866 14 13.7403 14 13C14 12.2597 14.4022 11.6134 15 11.2676V4C15 3.44772 15.4477 3 16 3Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiAnnotation;
impl IconShape for HiAnnotation {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 13V5C18 3.89543 17.1046 3 16 3H4C2.89543 3 2 3.89543 2 5V13C2 14.1046 2.89543 15 4 15H7L10 18L13 15H16C17.1046 15 18 14.1046 18 13ZM5 7C5 6.44772 5.44772 6 6 6H14C14.5523 6 15 6.44772 15 7C15 7.55228 14.5523 8 14 8H6C5.44772 8 5 7.55228 5 7ZM6 10C5.44772 10 5 10.4477 5 11C5 11.5523 5.44772 12 6 12H9C9.55229 12 10 11.5523 10 11C10 10.4477 9.55229 10 9 10H6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArchive;
impl IconShape for HiArchive {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 3C2.89543 3 2 3.89543 2 5C2 6.10457 2.89543 7 4 7H16C17.1046 7 18 6.10457 18 5C18 3.89543 17.1046 3 16 3H4Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 8H17V15C17 16.1046 16.1046 17 15 17H5C3.89543 17 3 16.1046 3 15V8ZM8 11C8 10.4477 8.44772 10 9 10H11C11.5523 10 12 10.4477 12 11C12 11.5523 11.5523 12 11 12H9C8.44772 12 8 11.5523 8 11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowCircleDown;
impl IconShape for HiArrowCircleDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM11 7C11 6.44772 10.5523 6 10 6C9.44771 6 9 6.44772 9 7L9 10.5858L7.70711 9.29289C7.31658 8.90237 6.68342 8.90237 6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071L9.29289 13.7071C9.68342 14.0976 10.3166 14.0976 10.7071 13.7071L13.7071 10.7071C14.0976 10.3166 14.0976 9.68342 13.7071 9.29289C13.3166 8.90237 12.6834 8.90237 12.2929 9.29289L11 10.5858V7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowCircleLeft;
impl IconShape for HiArrowCircleLeft {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM10.7071 7.70711C11.0976 7.31658 11.0976 6.68342 10.7071 6.29289C10.3166 5.90237 9.68342 5.90237 9.29289 6.29289L6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071L9.29289 13.7071C9.68342 14.0976 10.3166 14.0976 10.7071 13.7071C11.0976 13.3166 11.0976 12.6834 10.7071 12.2929L9.41421 11H13C13.5523 11 14 10.5523 14 10C14 9.44772 13.5523 9 13 9L9.41421 9L10.7071 7.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowCircleRight;
impl IconShape for HiArrowCircleRight {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM13.7071 9.29289L10.7071 6.29289C10.3166 5.90237 9.68342 5.90237 9.29289 6.29289C8.90237 6.68342 8.90237 7.31658 9.29289 7.70711L10.5858 9L7 9C6.44772 9 6 9.44771 6 10C6 10.5523 6.44772 11 7 11H10.5858L9.29289 12.2929C8.90237 12.6834 8.90237 13.3166 9.29289 13.7071C9.68342 14.0976 10.3166 14.0976 10.7071 13.7071L13.7071 10.7071C14.0976 10.3166 14.0976 9.68342 13.7071 9.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowCircleUp;
impl IconShape for HiArrowCircleUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM13.7071 9.29289L10.7071 6.29289C10.3166 5.90237 9.68342 5.90237 9.29289 6.29289L6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071C6.68342 11.0976 7.31658 11.0976 7.70711 10.7071L9 9.41421L9 13C9 13.5523 9.44771 14 10 14C10.5523 14 11 13.5523 11 13V9.41421L12.2929 10.7071C12.6834 11.0976 13.3166 11.0976 13.7071 10.7071C14.0976 10.3166 14.0976 9.68342 13.7071 9.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowDown;
impl IconShape for HiArrowDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M16.7071 10.2929C17.0976 10.6834 17.0976 11.3166 16.7071 11.7071L10.7071 17.7071C10.3166 18.0976 9.68342 18.0976 9.29289 17.7071L3.29289 11.7071C2.90237 11.3166 2.90237 10.6834 3.29289 10.2929C3.68342 9.90237 4.31658 9.90237 4.70711 10.2929L9 14.5858L9 3C9 2.44772 9.44772 2 10 2C10.5523 2 11 2.44772 11 3L11 14.5858L15.2929 10.2929C15.6834 9.90237 16.3166 9.90237 16.7071 10.2929Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowLeft;
impl IconShape for HiArrowLeft {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.70711 16.7071C9.31658 17.0976 8.68342 17.0976 8.29289 16.7071L2.29289 10.7071C1.90237 10.3166 1.90237 9.68342 2.29289 9.29289L8.29289 3.29289C8.68342 2.90237 9.31658 2.90237 9.70711 3.29289C10.0976 3.68342 10.0976 4.31658 9.70711 4.70711L5.41421 9H17C17.5523 9 18 9.44772 18 10C18 10.5523 17.5523 11 17 11L5.41421 11L9.70711 15.2929C10.0976 15.6834 10.0976 16.3166 9.70711 16.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowNarrowDown;
impl IconShape for HiArrowNarrowDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.7071 12.2929C15.0976 12.6834 15.0976 13.3166 14.7071 13.7071L10.7071 17.7071C10.3166 18.0976 9.68342 18.0976 9.29289 17.7071L5.29289 13.7071C4.90237 13.3166 4.90237 12.6834 5.29289 12.2929C5.68342 11.9024 6.31658 11.9024 6.70711 12.2929L9 14.5858L9 3C9 2.44772 9.44772 2 10 2C10.5523 2 11 2.44772 11 3L11 14.5858L13.2929 12.2929C13.6834 11.9024 14.3166 11.9024 14.7071 12.2929Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowNarrowLeft;
impl IconShape for HiArrowNarrowLeft {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.70711 14.7071C7.31658 15.0976 6.68342 15.0976 6.29289 14.7071L2.29289 10.7071C1.90237 10.3166 1.90237 9.68342 2.29289 9.29289L6.29289 5.29289C6.68342 4.90237 7.31658 4.90237 7.70711 5.29289C8.09763 5.68342 8.09763 6.31658 7.70711 6.70711L5.41421 9L17 9C17.5523 9 18 9.44771 18 10C18 10.5523 17.5523 11 17 11L5.41421 11L7.70711 13.2929C8.09763 13.6834 8.09763 14.3166 7.70711 14.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowNarrowRight;
impl IconShape for HiArrowNarrowRight {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.2929 5.29289C12.6834 4.90237 13.3166 4.90237 13.7071 5.29289L17.7071 9.29289C18.0976 9.68342 18.0976 10.3166 17.7071 10.7071L13.7071 14.7071C13.3166 15.0976 12.6834 15.0976 12.2929 14.7071C11.9024 14.3166 11.9024 13.6834 12.2929 13.2929L14.5858 11H3C2.44772 11 2 10.5523 2 10C2 9.44772 2.44772 9 3 9H14.5858L12.2929 6.70711C11.9024 6.31658 11.9024 5.68342 12.2929 5.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowNarrowUp;
impl IconShape for HiArrowNarrowUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.29289 7.70711C4.90237 7.31658 4.90237 6.68342 5.29289 6.29289L9.29289 2.29289C9.68342 1.90237 10.3166 1.90237 10.7071 2.29289L14.7071 6.29289C15.0976 6.68342 15.0976 7.31658 14.7071 7.70711C14.3166 8.09763 13.6834 8.09763 13.2929 7.70711L11 5.41421L11 17C11 17.5523 10.5523 18 10 18C9.44772 18 9 17.5523 9 17L9 5.41421L6.70711 7.70711C6.31658 8.09763 5.68342 8.09763 5.29289 7.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowRight;
impl IconShape for HiArrowRight {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.2929 3.29289C10.6834 2.90237 11.3166 2.90237 11.7071 3.29289L17.7071 9.29289C18.0976 9.68342 18.0976 10.3166 17.7071 10.7071L11.7071 16.7071C11.3166 17.0976 10.6834 17.0976 10.2929 16.7071C9.90237 16.3166 9.90237 15.6834 10.2929 15.2929L14.5858 11L3 11C2.44772 11 2 10.5523 2 10C2 9.44772 2.44772 9 3 9H14.5858L10.2929 4.70711C9.90237 4.31658 9.90237 3.68342 10.2929 3.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowSmDown;
impl IconShape for HiArrowSmDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.7071 10.2929C15.0976 10.6834 15.0976 11.3166 14.7071 11.7071L10.7071 15.7071C10.3166 16.0976 9.68342 16.0976 9.29289 15.7071L5.29289 11.7071C4.90237 11.3166 4.90237 10.6834 5.29289 10.2929C5.68342 9.90237 6.31658 9.90237 6.70711 10.2929L9 12.5858V5C9 4.44772 9.44772 4 10 4C10.5523 4 11 4.44772 11 5L11 12.5858L13.2929 10.2929C13.6834 9.90237 14.3166 9.90237 14.7071 10.2929Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowSmLeft;
impl IconShape for HiArrowSmLeft {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.70711 14.7071C9.31658 15.0976 8.68342 15.0976 8.29289 14.7071L4.29289 10.7071C3.90237 10.3166 3.90237 9.68342 4.29289 9.29289L8.29289 5.29289C8.68342 4.90237 9.31658 4.90237 9.70711 5.29289C10.0976 5.68342 10.0976 6.31658 9.70711 6.70711L7.41421 9L15 9C15.5523 9 16 9.44772 16 10C16 10.5523 15.5523 11 15 11H7.41421L9.70711 13.2929C10.0976 13.6834 10.0976 14.3166 9.70711 14.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowSmRight;
impl IconShape for HiArrowSmRight {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.2929 5.29289C10.6834 4.90237 11.3166 4.90237 11.7071 5.29289L15.7071 9.29289C16.0976 9.68342 16.0976 10.3166 15.7071 10.7071L11.7071 14.7071C11.3166 15.0976 10.6834 15.0976 10.2929 14.7071C9.90237 14.3166 9.90237 13.6834 10.2929 13.2929L12.5858 11L5 11C4.44772 11 4 10.5523 4 10C4 9.44772 4.44772 9 5 9H12.5858L10.2929 6.70711C9.90237 6.31658 9.90237 5.68342 10.2929 5.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowSmUp;
impl IconShape for HiArrowSmUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.29289 9.70711C4.90237 9.31658 4.90237 8.68342 5.29289 8.29289L9.29289 4.29289C9.68342 3.90237 10.3166 3.90237 10.7071 4.29289L14.7071 8.29289C15.0976 8.68342 15.0976 9.31658 14.7071 9.70711C14.3166 10.0976 13.6834 10.0976 13.2929 9.70711L11 7.41421L11 15C11 15.5523 10.5523 16 10 16C9.44772 16 9 15.5523 9 15L9 7.41421L6.70711 9.70711C6.31658 10.0976 5.68342 10.0976 5.29289 9.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowUp;
impl IconShape for HiArrowUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.29289 9.70711C2.90237 9.31658 2.90237 8.68342 3.29289 8.29289L9.29289 2.29289C9.68342 1.90237 10.3166 1.90237 10.7071 2.29289L16.7071 8.29289C17.0976 8.68342 17.0976 9.31658 16.7071 9.70711C16.3166 10.0976 15.6834 10.0976 15.2929 9.70711L11 5.41421L11 17C11 17.5523 10.5523 18 10 18C9.44772 18 9 17.5523 9 17L9 5.41421L4.70711 9.70711C4.31658 10.0976 3.68342 10.0976 3.29289 9.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiArrowsExpand;
impl IconShape for HiArrowsExpand {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 4C3 3.44772 3.44772 3 4 3H8C8.55228 3 9 3.44772 9 4C9 4.55228 8.55228 5 8 5H6.41421L8.70711 7.29289C9.09763 7.68342 9.09763 8.31658 8.70711 8.70711C8.31658 9.09763 7.68342 9.09763 7.29289 8.70711L5 6.41421V8C5 8.55228 4.55228 9 4 9C3.44772 9 3 8.55228 3 8V4ZM12 5C11.4477 5 11 4.55228 11 4C11 3.44772 11.4477 3 12 3H16C16.5523 3 17 3.44772 17 4V8C17 8.55228 16.5523 9 16 9C15.4477 9 15 8.55228 15 8V6.41421L12.7071 8.70711C12.3166 9.09763 11.6834 9.09763 11.2929 8.70711C10.9024 8.31658 10.9024 7.68342 11.2929 7.29289L13.5858 5H12ZM3 12C3 11.4477 3.44772 11 4 11C4.55228 11 5 11.4477 5 12V13.5858L7.29289 11.2929C7.68342 10.9024 8.31658 10.9024 8.70711 11.2929C9.09763 11.6834 9.09763 12.3166 8.70711 12.7071L6.41421 15H8C8.55228 15 9 15.4477 9 16C9 16.5523 8.55228 17 8 17H4C3.44772 17 3 16.5523 3 16V12ZM16 11C16.5523 11 17 11.4477 17 12V16C17 16.5523 16.5523 17 16 17H12C11.4477 17 11 16.5523 11 16C11 15.4477 11.4477 15 12 15H13.5858L11.2929 12.7071C10.9024 12.3166 10.9024 11.6834 11.2929 11.2929C11.6834 10.9024 12.3166 10.9024 12.7071 11.2929L15 13.5858V12C15 11.4477 15.4477 11 16 11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiAtSymbol;
impl IconShape for HiAtSymbol {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.2426 5.75736C11.8995 3.41421 8.10051 3.41421 5.75736 5.75736C3.41421 8.10051 3.41421 11.8995 5.75736 14.2426C7.79395 16.2792 10.9325 16.5464 13.257 15.0408C13.7205 14.7405 14.3397 14.8729 14.6399 15.3364C14.9402 15.8 14.8078 16.4191 14.3443 16.7194C11.2445 18.7273 7.0606 18.3743 4.34315 15.6569C1.21895 12.5327 1.21895 7.46734 4.34315 4.34315C7.46734 1.21895 12.5327 1.21895 15.6569 4.34315C17.2187 5.90503 18 7.9542 18 10C18 11.6569 16.6569 13 15 13C14.3247 13 13.7015 12.7769 13.2001 12.4003C12.4703 13.3717 11.3085 14 10 14C7.79086 14 6 12.2091 6 10C6 7.79086 7.79086 6 10 6C12.2091 6 14 7.79086 14 10C14 10.5523 14.4477 11 15 11C15.5523 11 16 10.5523 16 10C16 8.46294 15.4144 6.9291 14.2426 5.75736ZM12 10C12 8.89543 11.1046 8 10 8C8.89543 8 8 8.89543 8 10C8 11.1046 8.89543 12 10 12C11.1046 12 12 11.1046 12 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBackspace;
impl IconShape for HiBackspace {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.70711 4.87868C7.26972 4.31607 8.03278 4 8.82843 4H15C16.6569 4 18 5.34315 18 7V13C18 14.6569 16.6569 16 15 16H8.82843C8.03278 16 7.26972 15.6839 6.70711 15.1213L2.29289 10.7071C2.10536 10.5196 2 10.2652 2 10C2 9.73478 2.10536 9.48043 2.29289 9.29289L6.70711 4.87868ZM10.7071 7.29289C10.3166 6.90237 9.68342 6.90237 9.29289 7.29289C8.90237 7.68342 8.90237 8.31658 9.29289 8.70711L10.5858 10L9.29289 11.2929C8.90237 11.6834 8.90237 12.3166 9.29289 12.7071C9.68342 13.0976 10.3166 13.0976 10.7071 12.7071L12 11.4142L13.2929 12.7071C13.6834 13.0976 14.3166 13.0976 14.7071 12.7071C15.0976 12.3166 15.0976 11.6834 14.7071 11.2929L13.4142 10L14.7071 8.70711C15.0976 8.31658 15.0976 7.68342 14.7071 7.29289C14.3166 6.90237 13.6834 6.90237 13.2929 7.29289L12 8.58579L10.7071 7.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBadgeCheck;
impl IconShape for HiBadgeCheck {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.26701 3.45496C6.91008 3.40364 7.52057 3.15077 8.01158 2.73234C9.15738 1.75589 10.8426 1.75589 11.9884 2.73234C12.4794 3.15077 13.0899 3.40364 13.733 3.45496C15.2336 3.57471 16.4253 4.76636 16.545 6.26701C16.5964 6.91008 16.8492 7.52057 17.2677 8.01158C18.2441 9.15738 18.2441 10.8426 17.2677 11.9884C16.8492 12.4794 16.5964 13.0899 16.545 13.733C16.4253 15.2336 15.2336 16.4253 13.733 16.545C13.0899 16.5964 12.4794 16.8492 11.9884 17.2677C10.8426 18.2441 9.15738 18.2441 8.01158 17.2677C7.52057 16.8492 6.91008 16.5964 6.26701 16.545C4.76636 16.4253 3.57471 15.2336 3.45496 13.733C3.40364 13.0899 3.15077 12.4794 2.73234 11.9884C1.75589 10.8426 1.75589 9.15738 2.73234 8.01158C3.15077 7.52057 3.40364 6.91008 3.45496 6.26701C3.57471 4.76636 4.76636 3.57471 6.26701 3.45496ZM13.7071 8.70711C14.0976 8.31658 14.0976 7.68342 13.7071 7.29289C13.3166 6.90237 12.6834 6.90237 12.2929 7.29289L9 10.5858L7.70711 9.29289C7.31658 8.90237 6.68342 8.90237 6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071L8.29289 12.7071C8.68342 13.0976 9.31658 13.0976 9.70711 12.7071L13.7071 8.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBan;
impl IconShape for HiBan {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M13.4766 14.8907C12.4958 15.5892 11.2959 16 10 16C6.68629 16 4 13.3137 4 10C4 8.70414 4.41081 7.50423 5.1093 6.52339L13.4766 14.8907ZM14.8908 13.4765L6.52354 5.1092C7.50434 4.41077 8.7042 4 10 4C13.3137 4 16 6.68629 16 10C16 11.2958 15.5892 12.4957 14.8908 13.4765ZM18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBeaker;
impl IconShape for HiBeaker {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.99985 2C6.59539 2 6.23075 2.24364 6.07597 2.61732C5.92119 2.99099 6.00675 3.42111 6.29275 3.70711L6.99985 4.41421V8.17157C6.99985 8.43679 6.8945 8.69114 6.70696 8.87868L2.70696 12.8787C0.817066 14.7686 2.15556 18 4.82828 18H15.1714C17.8441 18 19.1826 14.7686 17.2927 12.8787L13.2927 8.87868C13.1052 8.69114 12.9999 8.43679 12.9999 8.17157V4.41421L13.707 3.70711C13.993 3.42111 14.0785 2.99099 13.9237 2.61732C13.769 2.24364 13.4043 2 12.9999 2H6.99985ZM8.99985 8.17157V4H10.9999V8.17157C10.9999 8.96722 11.3159 9.73028 11.8785 10.2929L12.9061 11.3204C12.1892 11.1537 11.4377 11.1874 10.7349 11.4217L10.2647 11.5784C9.44364 11.8521 8.55596 11.8521 7.73489 11.5784L7.17244 11.3909C7.13436 11.3782 7.09607 11.3667 7.05762 11.3564L8.12117 10.2929C8.68378 9.73028 8.99985 8.96722 8.99985 8.17157Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBell;
impl IconShape for HiBell {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 2C6.68632 2 4.00003 4.68629 4.00003 8V11.5858L3.29292 12.2929C3.00692 12.5789 2.92137 13.009 3.07615 13.3827C3.23093 13.7564 3.59557 14 4.00003 14H16C16.4045 14 16.7691 13.7564 16.9239 13.3827C17.0787 13.009 16.9931 12.5789 16.7071 12.2929L16 11.5858V8C16 4.68629 13.3137 2 10 2Z",
            }
            path {
                d: "M10 18C8.34315 18 7 16.6569 7 15H13C13 16.6569 11.6569 18 10 18Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBookOpen;
impl IconShape for HiBookOpen {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 4.80423C7.9428 4.28906 6.75516 4 5.5 4C4.24484 4 3.0572 4.28906 2 4.80423V14.8042C3.0572 14.2891 4.24484 14 5.5 14C7.1686 14 8.71789 14.5108 10 15.3847C11.2821 14.5108 12.8314 14 14.5 14C15.7552 14 16.9428 14.2891 18 14.8042V4.80423C16.9428 4.28906 15.7552 4 14.5 4C13.2448 4 12.0572 4.28906 11 4.80423V12C11 12.5523 10.5523 13 10 13C9.44772 13 9 12.5523 9 12V4.80423Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBookmarkAlt;
impl IconShape for HiBookmarkAlt {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 5C3 3.89543 3.89543 3 5 3H15C16.1046 3 17 3.89543 17 5V15C17 16.1046 16.1046 17 15 17H5C3.89543 17 3 16.1046 3 15V5ZM14 6H6V14L10 12L14 14V6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBookmark;
impl IconShape for HiBookmark {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 4C5 2.89543 5.89543 2 7 2H13C14.1046 2 15 2.89543 15 4V18L10 15.5L5 18V4Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiBriefcase;
impl IconShape for HiBriefcase {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 6V5C6 3.34315 7.34315 2 9 2H11C12.6569 2 14 3.34315 14 5V6H16C17.1046 6 18 6.89543 18 8V11.5708C15.5096 12.4947 12.8149 12.9999 10 12.9999C7.18514 12.9999 4.49037 12.4947 2 11.5707V8C2 6.89543 2.89543 6 4 6H6ZM8 5C8 4.44772 8.44772 4 9 4H11C11.5523 4 12 4.44772 12 5V6H8V5ZM9 10C9 9.44772 9.44772 9 10 9H10.01C10.5623 9 11.01 9.44772 11.01 10C11.01 10.5523 10.5623 11 10.01 11H10C9.44772 11 9 10.5523 9 10Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M2 13.6923V16C2 17.1046 2.89543 18 4 18H16C17.1046 18 18 17.1046 18 16V13.6923C15.4872 14.5404 12.7964 14.9999 10 14.9999C7.20363 14.9999 4.51279 14.5404 2 13.6923Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCake;
impl IconShape for HiCake {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 3C6 2.44772 6.44772 2 7 2H7.01C7.56228 2 8.01 2.44772 8.01 3C8.01 3.55228 7.56228 4 7.01 4H7C6.44772 4 6 3.55228 6 3ZM8 6C8 5.44772 7.55228 5 7 5C6.44772 5 6 5.44772 6 6V7C4.89543 7 4 7.89543 4 9V10C2.89543 10 2 10.8954 2 12V12.6833C2.36868 12.7866 2.72499 12.9482 3.0547 13.168C3.62713 13.5496 4.37287 13.5496 4.9453 13.168C6.18953 12.3385 7.81047 12.3385 9.0547 13.168C9.62713 13.5496 10.3729 13.5496 10.9453 13.168C12.1895 12.3385 13.8105 12.3385 15.0547 13.168C15.6271 13.5496 16.3729 13.5496 16.9453 13.168C17.275 12.9482 17.6313 12.7866 18 12.6833V12C18 10.8954 17.1046 10 16 10V9C16 7.89543 15.1046 7 14 7V6C14 5.44772 13.5523 5 13 5C12.4477 5 12 5.44772 12 6V7H11V6C11 5.44772 10.5523 5 10 5C9.44772 5 9 5.44772 9 6V7H8V6ZM18 14.8679C16.7633 15.6614 15.1714 15.6495 13.9453 14.8321C13.3729 14.4505 12.6271 14.4505 12.0547 14.8321C10.8105 15.6616 9.18953 15.6616 7.9453 14.8321C7.37287 14.4505 6.62713 14.4505 6.0547 14.8321C4.82863 15.6495 3.23675 15.6614 2 14.8679V17C2 17.5523 2.44772 18 3 18H17C17.5523 18 18 17.5523 18 17V14.8679ZM9 3C9 2.44772 9.44772 2 10 2H10.01C10.5623 2 11.01 2.44772 11.01 3C11.01 3.55228 10.5623 4 10.01 4H10C9.44772 4 9 3.55228 9 3ZM12 3C12 2.44772 12.4477 2 13 2H13.01C13.5623 2 14.01 2.44772 14.01 3C14.01 3.55228 13.5623 4 13.01 4H13C12.4477 4 12 3.55228 12 3Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCalculator;
impl IconShape for HiCalculator {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C4.89543 2 4 2.89543 4 4V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V4C16 2.89543 15.1046 2 14 2H6ZM7 4C6.44772 4 6 4.44772 6 5C6 5.55228 6.44772 6 7 6H13C13.5523 6 14 5.55228 14 5C14 4.44772 13.5523 4 13 4H7ZM13 11C13.5523 11 14 11.4477 14 12V15C14 15.5523 13.5523 16 13 16C12.4477 16 12 15.5523 12 15V12C12 11.4477 12.4477 11 13 11ZM10 14C9.44772 14 9 14.4477 9 15C9 15.5523 9.44772 16 10 16H10.01C10.5623 16 11.01 15.5523 11.01 15C11.01 14.4477 10.5623 14 10.01 14H10ZM6 15C6 14.4477 6.44772 14 7 14H7.01C7.56228 14 8.01 14.4477 8.01 15C8.01 15.5523 7.56228 16 7.01 16H7C6.44772 16 6 15.5523 6 15ZM7 11C6.44772 11 6 11.4477 6 12C6 12.5523 6.44772 13 7 13H7.01C7.56228 13 8.01 12.5523 8.01 12C8.01 11.4477 7.56228 11 7.01 11H7ZM9 12C9 11.4477 9.44772 11 10 11H10.01C10.5623 11 11.01 11.4477 11.01 12C11.01 12.5523 10.5623 13 10.01 13H10C9.44772 13 9 12.5523 9 12ZM13 8C12.4477 8 12 8.44772 12 9C12 9.55228 12.4477 10 13 10H13.01C13.5623 10 14.01 9.55228 14.01 9C14.01 8.44772 13.5623 8 13.01 8H13ZM9 9C9 8.44772 9.44772 8 10 8H10.01C10.5623 8 11.01 8.44772 11.01 9C11.01 9.55228 10.5623 10 10.01 10H10C9.44772 10 9 9.55228 9 9ZM7 8C6.44772 8 6 8.44772 6 9C6 9.55228 6.44772 10 7 10H7.01C7.56228 10 8.01 9.55228 8.01 9C8.01 8.44772 7.56228 8 7.01 8H7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCalendar;
impl IconShape for HiCalendar {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C5.44772 2 5 2.44772 5 3V4H4C2.89543 4 2 4.89543 2 6V16C2 17.1046 2.89543 18 4 18H16C17.1046 18 18 17.1046 18 16V6C18 4.89543 17.1046 4 16 4H15V3C15 2.44772 14.5523 2 14 2C13.4477 2 13 2.44772 13 3V4H7V3C7 2.44772 6.55228 2 6 2ZM6 7C5.44772 7 5 7.44772 5 8C5 8.55228 5.44772 9 6 9H14C14.5523 9 15 8.55228 15 8C15 7.44772 14.5523 7 14 7H6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCamera;
impl IconShape for HiCamera {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 5C2.89543 5 2 5.89543 2 7V15C2 16.1046 2.89543 17 4 17H16C17.1046 17 18 16.1046 18 15V7C18 5.89543 17.1046 5 16 5H14.4142C14.149 5 13.8946 4.89464 13.7071 4.70711L12.5858 3.58579C12.2107 3.21071 11.702 3 11.1716 3H8.82843C8.29799 3 7.78929 3.21071 7.41421 3.58579L6.29289 4.70711C6.10536 4.89464 5.851 5 5.58579 5H4ZM10 14C11.6569 14 13 12.6569 13 11C13 9.34315 11.6569 8 10 8C8.34315 8 7 9.34315 7 11C7 12.6569 8.34315 14 10 14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCash;
impl IconShape for HiCash {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 4C2.89543 4 2 4.89543 2 6V10C2 11.1046 2.89543 12 4 12V6H14C14 4.89543 13.1046 4 12 4H4ZM6 10C6 8.89543 6.89543 8 8 8H16C17.1046 8 18 8.89543 18 10V14C18 15.1046 17.1046 16 16 16H8C6.89543 16 6 15.1046 6 14V10ZM12 14C13.1046 14 14 13.1046 14 12C14 10.8954 13.1046 10 12 10C10.8954 10 10 10.8954 10 12C10 13.1046 10.8954 14 12 14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChartBar;
impl IconShape for HiChartBar {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 11C2 10.4477 2.44772 10 3 10H5C5.55228 10 6 10.4477 6 11V16C6 16.5523 5.55228 17 5 17H3C2.44772 17 2 16.5523 2 16V11Z",
            }
            path {
                d: "M8 7C8 6.44772 8.44772 6 9 6H11C11.5523 6 12 6.44772 12 7V16C12 16.5523 11.5523 17 11 17H9C8.44772 17 8 16.5523 8 16V7Z",
            }
            path {
                d: "M14 4C14 3.44772 14.4477 3 15 3H17C17.5523 3 18 3.44772 18 4V16C18 16.5523 17.5523 17 17 17H15C14.4477 17 14 16.5523 14 16V4Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChartPie;
impl IconShape for HiChartPie {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 10C2 5.58172 5.58172 2 10 2V10H18C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10Z",
            }
            path {
                d: "M12 2.25195C14.8113 2.97552 17.0245 5.18877 17.748 8.00004H12V2.25195Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChartSquareBar;
impl IconShape for HiChartSquareBar {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 3C3.89543 3 3 3.89543 3 5V15C3 16.1046 3.89543 17 5 17H15C16.1046 17 17 16.1046 17 15V5C17 3.89543 16.1046 3 15 3H5ZM14 7C14 6.44772 13.5523 6 13 6C12.4477 6 12 6.44772 12 7V13C12 13.5523 12.4477 14 13 14C13.5523 14 14 13.5523 14 13V7ZM11 9C11 8.44772 10.5523 8 10 8C9.44772 8 9 8.44772 9 9V13C9 13.5523 9.44772 14 10 14C10.5523 14 11 13.5523 11 13V9ZM8 12C8 11.4477 7.55228 11 7 11C6.44772 11 6 11.4477 6 12V13C6 13.5523 6.44772 14 7 14C7.55228 14 8 13.5523 8 13V12Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChatAlt2;
impl IconShape for HiChatAlt2 {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 5C2 3.89543 2.89543 3 4 3H11C12.1046 3 13 3.89543 13 5V9C13 10.1046 12.1046 11 11 11H9L6 14V11H4C2.89543 11 2 10.1046 2 9V5Z",
            }
            path {
                d: "M15 7V9C15 11.2091 13.2091 13 11 13H9.82843L8.06173 14.7667C8.34154 14.9156 8.66091 15 9 15H11L14 18V15H16C17.1046 15 18 14.1046 18 13V9C18 7.89543 17.1046 7 16 7H15Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChatAlt;
impl IconShape for HiChatAlt {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 5V13C18 14.1046 17.1046 15 16 15H11L6 19V15H4C2.89543 15 2 14.1046 2 13V5C2 3.89543 2.89543 3 4 3H16C17.1046 3 18 3.89543 18 5ZM7 8H5V10H7V8ZM9 8H11V10H9V8ZM15 8H13V10H15V8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChat;
impl IconShape for HiChat {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 13.866 14.4183 17 10 17C8.50836 17 7.11208 16.6428 5.91677 16.0208L2 17L3.3383 13.8773C2.4928 12.7673 2 11.434 2 10C2 6.13401 5.58172 3 10 3C14.4183 3 18 6.13401 18 10ZM7 9H5V11H7V9ZM15 9H13V11H15V9ZM9 9H11V11H9V9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCheckCircle;
impl IconShape for HiCheckCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM13.7071 8.70711C14.0976 8.31658 14.0976 7.68342 13.7071 7.29289C13.3166 6.90237 12.6834 6.90237 12.2929 7.29289L9 10.5858L7.70711 9.29289C7.31658 8.90237 6.68342 8.90237 6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071L8.29289 12.7071C8.68342 13.0976 9.31658 13.0976 9.70711 12.7071L13.7071 8.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCheck;
impl IconShape for HiCheck {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M16.7071 5.29289C17.0976 5.68342 17.0976 6.31658 16.7071 6.70711L8.70711 14.7071C8.31658 15.0976 7.68342 15.0976 7.29289 14.7071L3.29289 10.7071C2.90237 10.3166 2.90237 9.68342 3.29289 9.29289C3.68342 8.90237 4.31658 8.90237 4.70711 9.29289L8 12.5858L15.2929 5.29289C15.6834 4.90237 16.3166 4.90237 16.7071 5.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronDoubleDown;
impl IconShape for HiChevronDoubleDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M15.7071 4.29289C16.0976 4.68342 16.0976 5.31658 15.7071 5.70711L10.7071 10.7071C10.3166 11.0976 9.68342 11.0976 9.29289 10.7071L4.29289 5.70711C3.90237 5.31658 3.90237 4.68342 4.29289 4.29289C4.68342 3.90237 5.31658 3.90237 5.70711 4.29289L10 8.58579L14.2929 4.29289C14.6834 3.90237 15.3166 3.90237 15.7071 4.29289ZM15.7071 10.2929C16.0976 10.6834 16.0976 11.3166 15.7071 11.7071L10.7071 16.7071C10.3166 17.0976 9.68342 17.0976 9.29289 16.7071L4.29289 11.7071C3.90237 11.3166 3.90237 10.6834 4.29289 10.2929C4.68342 9.90237 5.31658 9.90237 5.70711 10.2929L10 14.5858L14.2929 10.2929C14.6834 9.90237 15.3166 9.90237 15.7071 10.2929Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronDoubleLeft;
impl IconShape for HiChevronDoubleLeft {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M15.7071 15.7071C15.3166 16.0976 14.6834 16.0976 14.2929 15.7071L9.29289 10.7071C8.90237 10.3166 8.90237 9.68342 9.29289 9.29289L14.2929 4.29289C14.6834 3.90237 15.3166 3.90237 15.7071 4.29289C16.0976 4.68342 16.0976 5.31658 15.7071 5.70711L11.4142 10L15.7071 14.2929C16.0976 14.6834 16.0976 15.3166 15.7071 15.7071ZM9.70711 15.7071C9.31658 16.0976 8.68342 16.0976 8.29289 15.7071L3.29289 10.7071C2.90237 10.3166 2.90237 9.68342 3.29289 9.29289L8.29289 4.29289C8.68342 3.90237 9.31658 3.90237 9.70711 4.29289C10.0976 4.68342 10.0976 5.31658 9.70711 5.70711L5.41421 10L9.70711 14.2929C10.0976 14.6834 10.0976 15.3166 9.70711 15.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronDoubleRight;
impl IconShape for HiChevronDoubleRight {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.2929 15.7071C9.90237 15.3166 9.90237 14.6834 10.2929 14.2929L14.5858 10L10.2929 5.70711C9.90237 5.31658 9.90237 4.68342 10.2929 4.29289C10.6834 3.90237 11.3166 3.90237 11.7071 4.29289L16.7071 9.29289C17.0976 9.68342 17.0976 10.3166 16.7071 10.7071L11.7071 15.7071C11.3166 16.0976 10.6834 16.0976 10.2929 15.7071Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M4.29289 15.7071C3.90237 15.3166 3.90237 14.6834 4.29289 14.2929L8.58579 10L4.29289 5.70711C3.90237 5.31658 3.90237 4.68342 4.29289 4.29289C4.68342 3.90237 5.31658 3.90237 5.70711 4.29289L10.7071 9.29289C11.0976 9.68342 11.0976 10.3166 10.7071 10.7071L5.70711 15.7071C5.31658 16.0976 4.68342 16.0976 4.29289 15.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronDoubleUp;
impl IconShape for HiChevronDoubleUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.29289 15.7071C3.90237 15.3166 3.90237 14.6834 4.29289 14.2929L9.29289 9.29289C9.68342 8.90237 10.3166 8.90237 10.7071 9.29289L15.7071 14.2929C16.0976 14.6834 16.0976 15.3166 15.7071 15.7071C15.3166 16.0976 14.6834 16.0976 14.2929 15.7071L10 11.4142L5.70711 15.7071C5.31658 16.0976 4.68342 16.0976 4.29289 15.7071ZM4.29289 9.70711C3.90237 9.31658 3.90237 8.68342 4.29289 8.29289L9.29289 3.29289C9.68342 2.90237 10.3166 2.90237 10.7071 3.29289L15.7071 8.29289C16.0976 8.68342 16.0976 9.31658 15.7071 9.70711C15.3166 10.0976 14.6834 10.0976 14.2929 9.70711L10 5.41421L5.70711 9.70711C5.31658 10.0976 4.68342 10.0976 4.29289 9.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronDown;
impl IconShape for HiChevronDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.29289 7.29289C5.68342 6.90237 6.31658 6.90237 6.70711 7.29289L10 10.5858L13.2929 7.29289C13.6834 6.90237 14.3166 6.90237 14.7071 7.29289C15.0976 7.68342 15.0976 8.31658 14.7071 8.70711L10.7071 12.7071C10.3166 13.0976 9.68342 13.0976 9.29289 12.7071L5.29289 8.70711C4.90237 8.31658 4.90237 7.68342 5.29289 7.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronLeft;
impl IconShape for HiChevronLeft {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.7071 5.29289C13.0976 5.68342 13.0976 6.31658 12.7071 6.70711L9.41421 10L12.7071 13.2929C13.0976 13.6834 13.0976 14.3166 12.7071 14.7071C12.3166 15.0976 11.6834 15.0976 11.2929 14.7071L7.29289 10.7071C6.90237 10.3166 6.90237 9.68342 7.29289 9.29289L11.2929 5.29289C11.6834 4.90237 12.3166 4.90237 12.7071 5.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronRight;
impl IconShape for HiChevronRight {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.29289 14.7071C6.90237 14.3166 6.90237 13.6834 7.29289 13.2929L10.5858 10L7.29289 6.70711C6.90237 6.31658 6.90237 5.68342 7.29289 5.29289C7.68342 4.90237 8.31658 4.90237 8.70711 5.29289L12.7071 9.29289C13.0976 9.68342 13.0976 10.3166 12.7071 10.7071L8.70711 14.7071C8.31658 15.0976 7.68342 15.0976 7.29289 14.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChevronUp;
impl IconShape for HiChevronUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M14.7071 12.7071C14.3166 13.0976 13.6834 13.0976 13.2929 12.7071L10 9.41421L6.70711 12.7071C6.31658 13.0976 5.68342 13.0976 5.29289 12.7071C4.90237 12.3166 4.90237 11.6834 5.29289 11.2929L9.29289 7.29289C9.68342 6.90237 10.3166 6.90237 10.7071 7.29289L14.7071 11.2929C15.0976 11.6834 15.0976 12.3166 14.7071 12.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiChip;
impl IconShape for HiChip {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 7H7V13H13V7Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M7 2C7 1.44772 7.44772 1 8 1C8.55228 1 9 1.44772 9 2V3H11V2C11 1.44772 11.4477 1 12 1C12.5523 1 13 1.44772 13 2V3H15C16.1046 3 17 3.89543 17 5V7H18C18.5523 7 19 7.44772 19 8C19 8.55228 18.5523 9 18 9H17V11H18C18.5523 11 19 11.4477 19 12C19 12.5523 18.5523 13 18 13H17V15C17 16.1046 16.1046 17 15 17H13V18C13 18.5523 12.5523 19 12 19C11.4477 19 11 18.5523 11 18V17H9V18C9 18.5523 8.55228 19 8 19C7.44772 19 7 18.5523 7 18V17H5C3.89543 17 3 16.1046 3 15V13H2C1.44772 13 1 12.5523 1 12C1 11.4477 1.44772 11 2 11H3V9H2C1.44772 9 1 8.55228 1 8C1 7.44772 1.44772 7 2 7H3V5C3 3.89543 3.89543 3 5 3H7V2ZM5 5H15V15H5V5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiClipboardCheck;
impl IconShape for HiClipboardCheck {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 2C8.44772 2 8 2.44772 8 3C8 3.55228 8.44772 4 9 4H11C11.5523 4 12 3.55228 12 3C12 2.44772 11.5523 2 11 2H9Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M4 5C4 3.89543 4.89543 3 6 3C6 4.65685 7.34315 6 9 6H11C12.6569 6 14 4.65685 14 3C15.1046 3 16 3.89543 16 5V16C16 17.1046 15.1046 18 14 18H6C4.89543 18 4 17.1046 4 16V5ZM13.7071 10.7071C14.0976 10.3166 14.0976 9.68342 13.7071 9.29289C13.3166 8.90237 12.6834 8.90237 12.2929 9.29289L9 12.5858L7.70711 11.2929C7.31658 10.9024 6.68342 10.9024 6.29289 11.2929C5.90237 11.6834 5.90237 12.3166 6.29289 12.7071L8.29289 14.7071C8.68342 15.0976 9.31658 15.0976 9.70711 14.7071L13.7071 10.7071Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiClipboardCopy;
impl IconShape for HiClipboardCopy {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 2C7.44772 2 7 2.44772 7 3C7 3.55228 7.44772 4 8 4H10C10.5523 4 11 3.55228 11 3C11 2.44772 10.5523 2 10 2H8Z",
            }
            path {
                d: "M3 5C3 3.89543 3.89543 3 5 3C5 4.65685 6.34315 6 8 6H10C11.6569 6 13 4.65685 13 3C14.1046 3 15 3.89543 15 5V11H10.4142L11.7071 9.70711C12.0976 9.31658 12.0976 8.68342 11.7071 8.29289C11.3166 7.90237 10.6834 7.90237 10.2929 8.29289L7.29289 11.2929C6.90237 11.6834 6.90237 12.3166 7.29289 12.7071L10.2929 15.7071C10.6834 16.0976 11.3166 16.0976 11.7071 15.7071C12.0976 15.3166 12.0976 14.6834 11.7071 14.2929L10.4142 13H15V16C15 17.1046 14.1046 18 13 18H5C3.89543 18 3 17.1046 3 16V5Z",
            }
            path {
                d: "M15 11H17C17.5523 11 18 11.4477 18 12C18 12.5523 17.5523 13 17 13H15V11Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiClipboardList;
impl IconShape for HiClipboardList {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 2C8.44772 2 8 2.44772 8 3C8 3.55228 8.44772 4 9 4H11C11.5523 4 12 3.55228 12 3C12 2.44772 11.5523 2 11 2H9Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M4 5C4 3.89543 4.89543 3 6 3C6 4.65685 7.34315 6 9 6H11C12.6569 6 14 4.65685 14 3C15.1046 3 16 3.89543 16 5V16C16 17.1046 15.1046 18 14 18H6C4.89543 18 4 17.1046 4 16V5ZM7 9C6.44772 9 6 9.44772 6 10C6 10.5523 6.44772 11 7 11H7.01C7.56228 11 8.01 10.5523 8.01 10C8.01 9.44772 7.56228 9 7.01 9H7ZM10 9C9.44772 9 9 9.44772 9 10C9 10.5523 9.44772 11 10 11H13C13.5523 11 14 10.5523 14 10C14 9.44772 13.5523 9 13 9H10ZM7 13C6.44772 13 6 13.4477 6 14C6 14.5523 6.44772 15 7 15H7.01C7.56228 15 8.01 14.5523 8.01 14C8.01 13.4477 7.56228 13 7.01 13H7ZM10 13C9.44772 13 9 13.4477 9 14C9 14.5523 9.44772 15 10 15H13C13.5523 15 14 14.5523 14 14C14 13.4477 13.5523 13 13 13H10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiClipboard;
impl IconShape for HiClipboard {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 3C8 2.44772 8.44772 2 9 2H11C11.5523 2 12 2.44772 12 3C12 3.55228 11.5523 4 11 4H9C8.44772 4 8 3.55228 8 3Z",
            }
            path {
                d: "M6 3C4.89543 3 4 3.89543 4 5V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V5C16 3.89543 15.1046 3 14 3C14 4.65685 12.6569 6 11 6H9C7.34315 6 6 4.65685 6 3Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiClock;
impl IconShape for HiClock {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM11 6C11 5.44772 10.5523 5 10 5C9.44771 5 9 5.44772 9 6V10C9 10.2652 9.10536 10.5196 9.29289 10.7071L12.1213 13.5355C12.5118 13.9261 13.145 13.9261 13.5355 13.5355C13.9261 13.145 13.9261 12.5118 13.5355 12.1213L11 9.58579V6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCloudDownload;
impl IconShape for HiCloudDownload {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 9.5C2 11.433 3.567 13 5.5 13H9V15.5858L7.70711 14.2929C7.31658 13.9024 6.68342 13.9024 6.29289 14.2929C5.90237 14.6834 5.90237 15.3166 6.29289 15.7071L9.29289 18.7071C9.68342 19.0976 10.3166 19.0976 10.7071 18.7071L13.7071 15.7071C14.0976 15.3166 14.0976 14.6834 13.7071 14.2929C13.3166 13.9024 12.6834 13.9024 12.2929 14.2929L11 15.5858V13H13.5C15.9853 13 18 10.9853 18 8.5C18 6.01472 15.9853 4 13.5 4C13.2912 4 13.0857 4.01422 12.8845 4.04175C12.4551 2.29538 10.8788 1 9 1C6.79086 1 5 2.79086 5 5C5 5.35223 5.04553 5.69382 5.13102 6.01922C3.37146 6.20358 2 7.69163 2 9.5ZM11 13H9V8C9 7.44772 9.44772 7 10 7C10.5523 7 11 7.44772 11 8V13Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCloudUpload;
impl IconShape for HiCloudUpload {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.5 13C3.567 13 2 11.433 2 9.5C2 7.69163 3.37146 6.20358 5.13102 6.01922C5.04553 5.69382 5 5.35223 5 5C5 2.79086 6.79086 1 9 1C10.8788 1 12.4551 2.29538 12.8845 4.04175C13.0857 4.01422 13.2912 4 13.5 4C15.9853 4 18 6.01472 18 8.5C18 10.9853 15.9853 13 13.5 13H11V9.41421L12.2929 10.7071C12.6834 11.0976 13.3166 11.0976 13.7071 10.7071C14.0976 10.3166 14.0976 9.68342 13.7071 9.29289L10.7071 6.29289C10.3166 5.90237 9.68342 5.90237 9.29289 6.29289L6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071C6.68342 11.0976 7.31658 11.0976 7.70711 10.7071L9 9.41421L9 13H5.5Z",
            }
            path {
                d: "M9 13H11L11 18C11 18.5523 10.5523 19 10 19C9.44772 19 9 18.5523 9 18L9 13Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCloud;
impl IconShape for HiCloud {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5.5 16C3.567 16 2 14.433 2 12.5C2 10.6916 3.37146 9.20358 5.13102 9.01922C5.04553 8.69382 5 8.35223 5 8C5 5.79086 6.79086 4 9 4C10.8788 4 12.4551 5.29538 12.8845 7.04175C13.0857 7.01422 13.2912 7 13.5 7C15.9853 7 18 9.01472 18 11.5C18 13.9853 15.9853 16 13.5 16H5.5Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCode;
impl IconShape for HiCode {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.3162 3.05134C12.8402 3.22599 13.1233 3.79231 12.9487 4.31625L8.94868 16.3163C8.77404 16.8402 8.20772 17.1234 7.68377 16.9487C7.15983 16.7741 6.87667 16.2077 7.05132 15.6838L11.0513 3.6838C11.226 3.15986 11.7923 2.8767 12.3162 3.05134ZM5.70711 6.29292C6.09763 6.68344 6.09763 7.31661 5.70711 7.70713L3.41421 10L5.70711 12.2929C6.09763 12.6834 6.09763 13.3166 5.70711 13.7071C5.31658 14.0977 4.68342 14.0977 4.29289 13.7071L1.29289 10.7071C0.902369 10.3166 0.902369 9.68344 1.29289 9.29292L4.29289 6.29292C4.68342 5.9024 5.31658 5.9024 5.70711 6.29292ZM14.2929 6.29292C14.6834 5.9024 15.3166 5.9024 15.7071 6.29292L18.7071 9.29292C19.0976 9.68344 19.0976 10.3166 18.7071 10.7071L15.7071 13.7071C15.3166 14.0977 14.6834 14.0977 14.2929 13.7071C13.9024 13.3166 13.9024 12.6834 14.2929 12.2929L16.5858 10L14.2929 7.70713C13.9024 7.31661 13.9024 6.68344 14.2929 6.29292Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCog;
impl IconShape for HiCog {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.4892 3.17094C11.1102 1.60969 8.8898 1.60969 8.51078 3.17094C8.26594 4.17949 7.11045 4.65811 6.22416 4.11809C4.85218 3.28212 3.28212 4.85218 4.11809 6.22416C4.65811 7.11045 4.17949 8.26593 3.17094 8.51078C1.60969 8.8898 1.60969 11.1102 3.17094 11.4892C4.17949 11.7341 4.65811 12.8896 4.11809 13.7758C3.28212 15.1478 4.85218 16.7179 6.22417 15.8819C7.11045 15.3419 8.26594 15.8205 8.51078 16.8291C8.8898 18.3903 11.1102 18.3903 11.4892 16.8291C11.7341 15.8205 12.8896 15.3419 13.7758 15.8819C15.1478 16.7179 16.7179 15.1478 15.8819 13.7758C15.3419 12.8896 15.8205 11.7341 16.8291 11.4892C18.3903 11.1102 18.3903 8.8898 16.8291 8.51078C15.8205 8.26593 15.3419 7.11045 15.8819 6.22416C16.7179 4.85218 15.1478 3.28212 13.7758 4.11809C12.8896 4.65811 11.7341 4.17949 11.4892 3.17094ZM10 13C11.6569 13 13 11.6569 13 10C13 8.34315 11.6569 7 10 7C8.34315 7 7 8.34315 7 10C7 11.6569 8.34315 13 10 13Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCollection;
impl IconShape for HiCollection {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 3C6.44772 3 6 3.44772 6 4C6 4.55228 6.44772 5 7 5H13C13.5523 5 14 4.55228 14 4C14 3.44772 13.5523 3 13 3H7Z",
            }
            path {
                d: "M4 7C4 6.44772 4.44772 6 5 6H15C15.5523 6 16 6.44772 16 7C16 7.55228 15.5523 8 15 8H5C4.44772 8 4 7.55228 4 7Z",
            }
            path {
                d: "M2 11C2 9.89543 2.89543 9 4 9H16C17.1046 9 18 9.89543 18 11V15C18 16.1046 17.1046 17 16 17H4C2.89543 17 2 16.1046 2 15V11Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiColorSwatch;
impl IconShape for HiColorSwatch {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 2C2.89543 2 2 2.89543 2 4V15C2 16.6569 3.34315 18 5 18C6.65685 18 8 16.6569 8 15V4C8 2.89543 7.10457 2 6 2H4ZM5 16C5.55228 16 6 15.5523 6 15C6 14.4477 5.55228 14 5 14C4.44772 14 4 14.4477 4 15C4 15.5523 4.44772 16 5 16ZM10 14.2426L14.8995 9.34308C15.6805 8.56203 15.6805 7.2957 14.8995 6.51465L13.4853 5.10044C12.7042 4.31939 11.4379 4.31939 10.6568 5.10044L10 5.75728V14.2426ZM16 18H9.07104L15.071 12H16C17.1046 12 18 12.8954 18 14V16C18 17.1046 17.1046 18 16 18Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCreditCard;
impl IconShape for HiCreditCard {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4C2.89543 4 2 4.89543 2 6V7H18V6C18 4.89543 17.1046 4 16 4H4Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M18 9H2V14C2 15.1046 2.89543 16 4 16H16C17.1046 16 18 15.1046 18 14V9ZM4 13C4 12.4477 4.44772 12 5 12H6C6.55228 12 7 12.4477 7 13C7 13.5523 6.55228 14 6 14H5C4.44772 14 4 13.5523 4 13ZM9 12C8.44772 12 8 12.4477 8 13C8 13.5523 8.44772 14 9 14H10C10.5523 14 11 13.5523 11 13C11 12.4477 10.5523 12 10 12H9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCubeTransparent;
impl IconShape for HiCubeTransparent {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.50386 1.13176C9.81129 0.956081 10.1887 0.956081 10.4961 1.13176L12.2461 2.13176C12.7257 2.40577 12.8923 3.01662 12.6182 3.49614C12.3442 3.97566 11.7334 4.14225 11.2539 3.86824L10 3.15175L8.74614 3.86824C8.26662 4.14225 7.65577 3.97566 7.38176 3.49614C7.10775 3.01662 7.27434 2.40577 7.75386 2.13176L9.50386 1.13176ZM5.61824 4.50386C5.89225 4.98338 5.72566 5.59423 5.24614 5.86824L5.01556 6L5.24614 6.13176C5.72566 6.40577 5.89225 7.01662 5.61824 7.49614C5.34423 7.97566 4.73338 8.14225 4.25386 7.86824L4 7.72318V8C4 8.55228 3.55228 9 3 9C2.44772 9 2 8.55228 2 8V6C2 5.75001 2.09173 5.52145 2.24336 5.34614C2.27802 5.30603 2.31598 5.26854 2.35699 5.23411C2.40754 5.19163 2.46236 5.15405 2.52071 5.12213L4.25386 4.13176C4.73338 3.85775 5.34423 4.02434 5.61824 4.50386ZM14.3818 4.50386C14.6558 4.02434 15.2666 3.85775 15.7461 4.13176L17.4793 5.12212C17.5376 5.15405 17.5925 5.19162 17.643 5.23411C17.8613 5.41755 18 5.69258 18 6V8C18 8.55228 17.5523 9 17 9C16.4477 9 16 8.55228 16 8V7.72318L15.7461 7.86824C15.2666 8.14225 14.6558 7.97566 14.3818 7.49614C14.1077 7.01662 14.2743 6.40577 14.7539 6.13176L14.9844 6L14.7539 5.86824C14.2743 5.59423 14.1077 4.98338 14.3818 4.50386ZM7.38176 8.50386C7.65577 8.02434 8.26662 7.85775 8.74614 8.13176L10 8.84825L11.2539 8.13176C11.7334 7.85775 12.3442 8.02434 12.6182 8.50386C12.8923 8.98338 12.7257 9.59423 12.2461 9.86824L11 10.5803V12C11 12.5523 10.5523 13 10 13C9.44772 13 9 12.5523 9 12V10.5803L7.75386 9.86824C7.27434 9.59423 7.10775 8.98338 7.38176 8.50386ZM3 11C3.55228 11 4 11.4477 4 12V13.4197L5.24614 14.1318C5.72566 14.4058 5.89225 15.0166 5.61824 15.4961C5.34423 15.9757 4.73338 16.1423 4.25386 15.8682L2.50386 14.8682C2.19229 14.6902 2 14.3589 2 14V12C2 11.4477 2.44772 11 3 11ZM17 11C17.5523 11 18 11.4477 18 12V14C18 14.3589 17.8077 14.6902 17.4961 14.8682L15.7461 15.8682C15.2666 16.1423 14.6558 15.9757 14.3818 15.4961C14.1077 15.0166 14.2743 14.4058 14.7539 14.1318L16 13.4197V12C16 11.4477 16.4477 11 17 11ZM7.38176 16.5039C7.65577 16.0243 8.26662 15.8577 8.74614 16.1318L9 16.2768V16C9 15.4477 9.44772 15 10 15C10.5523 15 11 15.4477 11 16V16.2768L11.2539 16.1318C11.7334 15.8577 12.3442 16.0243 12.6182 16.5039C12.8923 16.9834 12.7257 17.5942 12.2461 17.8682L10.5113 18.8596C10.3617 18.9488 10.1868 19 10 19C9.81316 19 9.63828 18.9488 9.48866 18.8596L7.75386 17.8682C7.27434 17.5942 7.10775 16.9834 7.38176 16.5039Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCube;
impl IconShape for HiCube {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 17C11 17.3466 11.1795 17.6684 11.4743 17.8507C11.7691 18.0329 12.1372 18.0494 12.4472 17.8944L16.4472 15.8944C16.786 15.725 17 15.3788 17 15V9.23607C17 8.88949 16.8205 8.56762 16.5257 8.38542C16.2309 8.20321 15.8628 8.18665 15.5528 8.34164L11.5528 10.3416C11.214 10.511 11 10.8573 11 11.2361V17Z",
            }
            path {
                d: "M15.2111 6.27639C15.5499 6.107 15.7639 5.76074 15.7639 5.38197C15.7639 5.00319 15.5499 4.65693 15.2111 4.48754L10.4472 2.10557C10.1657 1.96481 9.83431 1.96481 9.55279 2.10557L4.78885 4.48754C4.45007 4.65693 4.23607 5.00319 4.23607 5.38197C4.23607 5.76074 4.45007 6.107 4.78885 6.27639L9.55279 8.65836C9.83431 8.79912 10.1657 8.79912 10.4472 8.65836L15.2111 6.27639Z",
            }
            path {
                d: "M4.44721 8.34164C4.13723 8.18665 3.76909 8.20321 3.47427 8.38542C3.17945 8.56762 3 8.88949 3 9.23607V15C3 15.3788 3.214 15.725 3.55279 15.8944L7.55279 17.8944C7.86277 18.0494 8.23091 18.0329 8.52573 17.8507C8.82055 17.6684 9 17.3466 9 17V11.2361C9 10.8573 8.786 10.511 8.44721 10.3416L4.44721 8.34164Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCurrencyBangladeshi;
impl IconShape for HiCurrencyBangladeshi {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7 4C6.44772 4 6 4.44772 6 5C6 5.55228 6.44772 6 7 6C7.55228 6 8 6.44772 8 7V8H7C6.44772 8 6 8.44772 6 9C6 9.55228 6.44772 10 7 10H8V13C8 14.6569 9.34315 16 11 16C12.6569 16 14 14.6569 14 13V12C14 11.4477 13.5523 11 13 11C12.4477 11 12 11.4477 12 12V13C12 13.5523 11.5523 14 11 14C10.4477 14 10 13.5523 10 13V10H13C13.5523 10 14 9.55228 14 9C14 8.44772 13.5523 8 13 8H10V7C10 5.34315 8.65685 4 7 4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCurrencyDollar;
impl IconShape for HiCurrencyDollar {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.43338 7.41784C8.58818 7.31464 8.77939 7.2224 9 7.15101L9.00001 8.84899C8.77939 8.7776 8.58818 8.68536 8.43338 8.58216C8.06927 8.33942 8 8.1139 8 8C8 7.8861 8.06927 7.66058 8.43338 7.41784Z",
            }
            path {
                d: "M11 12.849L11 11.151C11.2206 11.2224 11.4118 11.3146 11.5666 11.4178C11.9308 11.6606 12 11.8861 12 12C12 12.1139 11.9308 12.3394 11.5666 12.5822C11.4118 12.6854 11.2206 12.7776 11 12.849Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM11 5C11 4.44772 10.5523 4 10 4C9.44772 4 9 4.44772 9 5V5.09199C8.3784 5.20873 7.80348 5.43407 7.32398 5.75374C6.6023 6.23485 6 7.00933 6 8C6 8.99067 6.6023 9.76515 7.32398 10.2463C7.80348 10.5659 8.37841 10.7913 9.00001 10.908L9.00002 12.8492C8.60902 12.7223 8.31917 12.5319 8.15667 12.3446C7.79471 11.9275 7.16313 11.8827 6.74599 12.2447C6.32885 12.6067 6.28411 13.2382 6.64607 13.6554C7.20855 14.3036 8.05956 14.7308 9 14.9076L9 15C8.99999 15.5523 9.44769 16 9.99998 16C10.5523 16 11 15.5523 11 15L11 14.908C11.6216 14.7913 12.1965 14.5659 12.676 14.2463C13.3977 13.7651 14 12.9907 14 12C14 11.0093 13.3977 10.2348 12.676 9.75373C12.1965 9.43407 11.6216 9.20873 11 9.09199L11 7.15075C11.391 7.27771 11.6808 7.4681 11.8434 7.65538C12.2053 8.07252 12.8369 8.11726 13.254 7.7553C13.6712 7.39335 13.7159 6.76176 13.354 6.34462C12.7915 5.69637 11.9405 5.26915 11 5.09236V5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCurrencyEuro;
impl IconShape for HiCurrencyEuro {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM8.73617 6.97896C9.20793 6.1927 9.69618 6 10 6C10.3038 6 10.7921 6.1927 11.2638 6.97896C11.548 7.45254 12.1622 7.60611 12.6358 7.32196C13.1094 7.03781 13.263 6.42355 12.9788 5.94997C12.279 4.78361 11.2317 4 10 4C8.76829 4 7.721 4.78361 7.02119 5.94997C6.73632 6.42475 6.51422 6.94939 6.35097 7.5H6C5.44772 7.5 5 7.94772 5 8.5C5 9.05228 5.44772 9.5 6 9.5H6.01337C6.00443 9.66702 6 9.83388 6 10C6 10.1661 6.00443 10.333 6.01337 10.5H6C5.44772 10.5 5 10.9477 5 11.5C5 12.0523 5.44772 12.5 6 12.5H6.35097C6.51422 13.0506 6.73632 13.5753 7.02119 14.05C7.721 15.2164 8.76829 16 10 16C11.2317 16 12.279 15.2164 12.9788 14.05C13.263 13.5764 13.1094 12.9622 12.6358 12.678C12.1622 12.3939 11.548 12.5475 11.2638 13.021C10.7921 13.8073 10.3038 14 10 14C9.69618 14 9.20793 13.8073 8.73617 13.021C8.63927 12.8595 8.5511 12.6851 8.47216 12.5H10C10.5523 12.5 11 12.0523 11 11.5C11 10.9477 10.5523 10.5 10 10.5H8.01695C8.00571 10.335 8 10.1681 8 10C8 9.83191 8.00571 9.66498 8.01695 9.5H10C10.5523 9.5 11 9.05228 11 8.5C11 7.94772 10.5523 7.5 10 7.5H8.47216C8.5511 7.31488 8.63927 7.14047 8.73617 6.97896Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCurrencyPound;
impl IconShape for HiCurrencyPound {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM11 4C9.34315 4 8 5.34315 8 7V9H7C6.44772 9 6 9.44772 6 10C6 10.5523 6.44772 11 7 11H8V12C8 12.5523 7.55228 13 7 13C6.44772 13 6 13.4477 6 14C6 14.5523 6.44772 15 7 15H13C13.5523 15 14 14.5523 14 14C14 13.4477 13.5523 13 13 13H9.82929C9.93985 12.6872 10 12.3506 10 12V11H11C11.5523 11 12 10.5523 12 10C12 9.44772 11.5523 9 11 9H10V7C10 6.44772 10.4477 6 11 6C11.5523 6 12 6.44772 12 7C12 7.55228 12.4477 8 13 8C13.5523 8 14 7.55228 14 7C14 5.34315 12.6569 4 11 4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCurrencyRupee;
impl IconShape for HiCurrencyRupee {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7.00003 5C6.44774 5 6.00003 5.44772 6.00003 6C6.00003 6.55228 6.44774 7 7.00003 7H8.00003C8.74031 7 9.38666 7.4022 9.73246 8H7.00003C6.44774 8 6.00003 8.44772 6.00003 9C6.00003 9.55228 6.44774 10 7.00003 10H9.73246C9.38665 10.5978 8.74031 11 8.00003 11H7.00003C6.59557 11 6.23093 11.2436 6.07615 11.6173C5.92137 11.991 6.00692 12.4211 6.29292 12.7071L9.29292 15.7071C9.68345 16.0976 10.3166 16.0976 10.7071 15.7071C11.0977 15.3166 11.0977 14.6834 10.7071 14.2929L9.22363 12.8094C10.5222 12.3926 11.5316 11.3302 11.874 10H13C13.5523 10 14 9.55228 14 9C14 8.44772 13.5523 8 13 8H11.874C11.7827 7.64523 11.6439 7.30951 11.4649 7H13C13.5523 7 14 6.55228 14 6C14 5.44772 13.5523 5 13 5H7.00003Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCurrencyYen;
impl IconShape for HiCurrencyYen {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7.85752 5.48541C7.57337 5.01183 6.95911 4.85827 6.48553 5.14241C6.01195 5.42656 5.85839 6.04082 6.14254 6.5144L7.63384 8.99991H7.00003C6.44774 8.99991 6.00003 9.44762 6.00003 9.99991C6.00003 10.5522 6.44774 10.9999 7.00003 10.9999H8.83384L9.00003 11.2769V11.9999H7.00003C6.44774 11.9999 6.00003 12.4476 6.00003 12.9999C6.00003 13.5522 6.44774 13.9999 7.00003 13.9999H9.00003V14.9999C9.00003 15.5522 9.44774 15.9999 10 15.9999C10.5523 15.9999 11 15.5522 11 14.9999V13.9999H13C13.5523 13.9999 14 13.5522 14 12.9999C14 12.4476 13.5523 11.9999 13 11.9999H11V11.2769L11.1662 10.9999H13C13.5523 10.9999 14 10.5522 14 9.99991C14 9.44762 13.5523 8.99991 13 8.99991H12.3662L13.8575 6.5144C14.1417 6.04082 13.9881 5.42656 13.5145 5.14241C13.0409 4.85827 12.4267 5.01183 12.1425 5.48541L10.0338 8.99991H9.96622L7.85752 5.48541Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiCursorClick;
impl IconShape for HiCursorClick {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.67187 1.91147C6.52893 1.37801 5.9806 1.06142 5.44713 1.20437C4.91366 1.34731 4.59708 1.89565 4.74002 2.42911L4.99884 3.39504C5.14178 3.9285 5.69012 4.24509 6.22359 4.10214C6.75705 3.9592 7.07363 3.41086 6.93069 2.8774L6.67187 1.91147ZM2.42923 4.7399C1.89577 4.59696 1.34743 4.91354 1.20449 5.44701C1.06155 5.98047 1.37813 6.52881 1.9116 6.67175L2.87752 6.93057C3.41099 7.07351 3.95932 6.75693 4.10227 6.22346C4.24521 5.69 3.92863 5.14166 3.39516 4.99872L2.42923 4.7399ZM11.2427 4.17149C11.6332 3.78097 11.6332 3.1478 11.2427 2.75728C10.8522 2.36676 10.219 2.36676 9.82847 2.75728L9.12136 3.46439C8.73084 3.85491 8.73084 4.48808 9.12136 4.8786C9.51189 5.26912 10.1451 5.26912 10.5356 4.8786L11.2427 4.17149ZM4.17162 11.2426L4.87872 10.5355C5.26925 10.1449 5.26925 9.51177 4.87872 9.12124C4.4882 8.73072 3.85503 8.73072 3.46451 9.12124L2.7574 9.82835C2.36688 10.2189 2.36688 10.852 2.7574 11.2426C3.14793 11.6331 3.78109 11.6331 4.17162 11.2426ZM7.37154 6.07152C7.00012 5.92295 6.5759 6.01002 6.29304 6.29289C6.01018 6.57575 5.92311 6.99997 6.07167 7.37138L10.0717 17.3714C10.2179 17.737 10.5651 17.9828 10.9586 17.9991C11.352 18.0155 11.7185 17.7994 11.8946 17.4472L13.2741 14.6882L16.293 17.7071C16.6836 18.0976 17.3167 18.0976 17.7073 17.7071C18.0978 17.3166 18.0978 16.6834 17.7073 16.2929L14.6883 13.2739L17.4474 11.8944C17.7996 11.7183 18.0157 11.3519 17.9993 10.9584C17.9829 10.565 17.7372 10.2178 17.3715 10.0715L7.37154 6.07152Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDatabase;
impl IconShape for HiDatabase {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 12V15C3 16.6569 6.13401 18 10 18C13.866 18 17 16.6569 17 15V12C17 13.6569 13.866 15 10 15C6.13401 15 3 13.6569 3 12Z",
            }
            path {
                d: "M3 7V10C3 11.6569 6.13401 13 10 13C13.866 13 17 11.6569 17 10V7C17 8.65685 13.866 10 10 10C6.13401 10 3 8.65685 3 7Z",
            }
            path {
                d: "M17 5C17 6.65685 13.866 8 10 8C6.13401 8 3 6.65685 3 5C3 3.34315 6.13401 2 10 2C13.866 2 17 3.34315 17 5Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDesktopComputer;
impl IconShape for HiDesktopComputer {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 5C3 3.89543 3.89543 3 5 3H15C16.1046 3 17 3.89543 17 5V13C17 14.1046 16.1046 15 15 15H12.7808L12.903 15.4887L13.7071 16.2929C13.9931 16.5789 14.0787 17.009 13.9239 17.3827C13.7691 17.7563 13.4045 18 13 18H7.00003C6.59557 18 6.23093 17.7563 6.07615 17.3827C5.92137 17.009 6.00692 16.5789 6.29292 16.2929L7.09706 15.4887L7.21925 15H5C3.89543 15 3 14.1046 3 13V5ZM8.7713 12C8.75657 11.9997 8.74189 11.9997 8.72725 12H5V5H15V12H11.2728C11.2582 11.9997 11.2435 11.9997 11.2288 12H8.7713Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDeviceMobile;
impl IconShape for HiDeviceMobile {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7 2C5.89543 2 5 2.89543 5 4V16C5 17.1046 5.89543 18 7 18H13C14.1046 18 15 17.1046 15 16V4C15 2.89543 14.1046 2 13 2H7ZM10 16C10.5523 16 11 15.5523 11 15C11 14.4477 10.5523 14 10 14C9.44772 14 9 14.4477 9 15C9 15.5523 9.44772 16 10 16Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDeviceTablet;
impl IconShape for HiDeviceTablet {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C4.89543 2 4 2.89543 4 4V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V4C16 2.89543 15.1046 2 14 2H6ZM10 16C10.5523 16 11 15.5523 11 15C11 14.4477 10.5523 14 10 14C9.44772 14 9 14.4477 9 15C9 15.5523 9.44772 16 10 16Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentAdd;
impl IconShape for HiDocumentAdd {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C4.89543 2 4 2.89543 4 4V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V7.41421C16 6.88378 15.7893 6.37507 15.4142 6L12 2.58579C11.6249 2.21071 11.1162 2 10.5858 2H6ZM11 8C11 7.44772 10.5523 7 10 7C9.44772 7 9 7.44772 9 8V10H7C6.44772 10 6 10.4477 6 11C6 11.5523 6.44772 12 7 12H9V14C9 14.5523 9.44771 15 10 15C10.5523 15 11 14.5523 11 14L11 12H13C13.5523 12 14 11.5523 14 11C14 10.4477 13.5523 10 13 10H11V8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentDownload;
impl IconShape for HiDocumentDownload {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C4.89543 2 4 2.89543 4 4V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V7.41421C16 6.88378 15.7893 6.37507 15.4142 6L12 2.58579C11.6249 2.21071 11.1162 2 10.5858 2H6ZM11 8C11 7.44772 10.5523 7 10 7C9.44772 7 9 7.44772 9 8V11.5858L7.70711 10.2929C7.31658 9.90237 6.68342 9.90237 6.29289 10.2929C5.90237 10.6834 5.90237 11.3166 6.29289 11.7071L9.29289 14.7071C9.68342 15.0976 10.3166 15.0976 10.7071 14.7071L13.7071 11.7071C14.0976 11.3166 14.0976 10.6834 13.7071 10.2929C13.3166 9.90237 12.6834 9.90237 12.2929 10.2929L11 11.5858V8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentDuplicate;
impl IconShape for HiDocumentDuplicate {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 2C7.89543 2 7 2.89543 7 4V12C7 13.1046 7.89543 14 9 14H15C16.1046 14 17 13.1046 17 12V6.41421C17 5.88378 16.7893 5.37507 16.4142 5L14 2.58579C13.6249 2.21071 13.1162 2 12.5858 2H9Z",
            }
            path {
                d: "M3 8C3 6.89543 3.89543 6 5 6V16H13C13 17.1046 12.1046 18 11 18H5C3.89543 18 3 17.1046 3 16V8Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentRemove;
impl IconShape for HiDocumentRemove {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C4.89543 2 4 2.89543 4 4V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V7.41421C16 6.88378 15.7893 6.37507 15.4142 6L12 2.58579C11.6249 2.21071 11.1162 2 10.5858 2H6ZM7 10C6.44772 10 6 10.4477 6 11C6 11.5523 6.44772 12 7 12H13C13.5523 12 14 11.5523 14 11C14 10.4477 13.5523 10 13 10H7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentReport;
impl IconShape for HiDocumentReport {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6 2C4.89543 2 4 2.89543 4 4V16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V7.41421C16 6.88378 15.7893 6.37507 15.4142 6L12 2.58579C11.6249 2.21071 11.1162 2 10.5858 2H6ZM8 12C8 11.4477 7.55228 11 7 11C6.44772 11 6 11.4477 6 12V15C6 15.5523 6.44772 16 7 16C7.55228 16 8 15.5523 8 15V12ZM10 9C10.5523 9 11 9.44772 11 10V15C11 15.5523 10.5523 16 10 16C9.44772 16 9 15.5523 9 15V10C9 9.44772 9.44772 9 10 9ZM14 8C14 7.44772 13.5523 7 13 7C12.4477 7 12 7.44772 12 8V15C12 15.5523 12.4477 16 13 16C13.5523 16 14 15.5523 14 15V8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentSearch;
impl IconShape for HiDocumentSearch {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4C4 2.89543 4.89543 2 6 2H10.5858C11.1162 2 11.6249 2.21071 12 2.58579L15.4142 6C15.7893 6.37507 16 6.88378 16 7.41421V16C16 17.1046 15.1046 18 14 18H12.4722C13.4223 16.9385 14 15.5367 14 14C14 10.6863 11.3137 8 8 8C6.46329 8 5.06151 8.57771 4 9.52779V4Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M8 10C5.79086 10 4 11.7909 4 14C4 14.7414 4.20229 15.4364 4.55397 16.0318L3.29289 17.2929C2.90237 17.6834 2.90237 18.3166 3.29289 18.7071C3.68342 19.0976 4.31658 19.0976 4.70711 18.7071L5.96818 17.446C6.56362 17.7977 7.25862 18 8 18C10.2091 18 12 16.2091 12 14C12 11.7909 10.2091 10 8 10ZM6 14C6 12.8954 6.89543 12 8 12C9.10457 12 10 12.8954 10 14C10 15.1046 9.10457 16 8 16C7.44744 16 6.94881 15.7772 6.58579 15.4142C6.22276 15.0512 6 14.5526 6 14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocumentText;
impl IconShape for HiDocumentText {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 4C4 2.89543 4.89543 2 6 2H10.5858C11.1162 2 11.6249 2.21071 12 2.58579L15.4142 6C15.7893 6.37507 16 6.88378 16 7.41421V16C16 17.1046 15.1046 18 14 18H6C4.89543 18 4 17.1046 4 16V4ZM6 10C6 9.44772 6.44772 9 7 9H13C13.5523 9 14 9.44772 14 10C14 10.5523 13.5523 11 13 11H7C6.44772 11 6 10.5523 6 10ZM7 13C6.44772 13 6 13.4477 6 14C6 14.5523 6.44772 15 7 15H13C13.5523 15 14 14.5523 14 14C14 13.4477 13.5523 13 13 13H7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDocument;
impl IconShape for HiDocument {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 4C4 2.89543 4.89543 2 6 2H10.5858C11.1162 2 11.6249 2.21071 12 2.58579L15.4142 6C15.7893 6.37507 16 6.88378 16 7.41421V16C16 17.1046 15.1046 18 14 18H6C4.89543 18 4 17.1046 4 16V4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDotsCircleHorizontal;
impl IconShape for HiDotsCircleHorizontal {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7 9H5V11H7V9ZM15 9H13V11H15V9ZM9 9H11V11H9V9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDotsHorizontal;
impl IconShape for HiDotsHorizontal {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M6 10C6 11.1046 5.10457 12 4 12C2.89543 12 2 11.1046 2 10C2 8.89543 2.89543 8 4 8C5.10457 8 6 8.89543 6 10Z",
            }
            path {
                d: "M12 10C12 11.1046 11.1046 12 10 12C8.89543 12 8 11.1046 8 10C8 8.89543 8.89543 8 10 8C11.1046 8 12 8.89543 12 10Z",
            }
            path {
                d: "M16 12C17.1046 12 18 11.1046 18 10C18 8.89543 17.1046 8 16 8C14.8954 8 14 8.89543 14 10C14 11.1046 14.8954 12 16 12Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDotsVertical;
impl IconShape for HiDotsVertical {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 6C8.89543 6 8 5.10457 8 4C8 2.89543 8.89543 2 10 2C11.1046 2 12 2.89543 12 4C12 5.10457 11.1046 6 10 6Z",
            }
            path {
                d: "M10 12C8.89543 12 8 11.1046 8 10C8 8.89543 8.89543 8 10 8C11.1046 8 12 8.89543 12 10C12 11.1046 11.1046 12 10 12Z",
            }
            path {
                d: "M10 18C8.89543 18 8 17.1046 8 16C8 14.8954 8.89543 14 10 14C11.1046 14 12 14.8954 12 16C12 17.1046 11.1046 18 10 18Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDownload;
impl IconShape for HiDownload {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 17C3 16.4477 3.44772 16 4 16H16C16.5523 16 17 16.4477 17 17C17 17.5523 16.5523 18 16 18H4C3.44772 18 3 17.5523 3 17ZM6.29289 9.29289C6.68342 8.90237 7.31658 8.90237 7.70711 9.29289L9 10.5858L9 3C9 2.44772 9.44771 2 10 2C10.5523 2 11 2.44771 11 3L11 10.5858L12.2929 9.29289C12.6834 8.90237 13.3166 8.90237 13.7071 9.29289C14.0976 9.68342 14.0976 10.3166 13.7071 10.7071L10.7071 13.7071C10.5196 13.8946 10.2652 14 10 14C9.73478 14 9.48043 13.8946 9.29289 13.7071L6.29289 10.7071C5.90237 10.3166 5.90237 9.68342 6.29289 9.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiDuplicate;
impl IconShape for HiDuplicate {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7 9C7 7.89543 7.89543 7 9 7H15C16.1046 7 17 7.89543 17 9V15C17 16.1046 16.1046 17 15 17H9C7.89543 17 7 16.1046 7 15V9Z",
            }
            path {
                d: "M5 3C3.89543 3 3 3.89543 3 5V11C3 12.1046 3.89543 13 5 13L5 5H13C13 3.89543 12.1046 3 11 3H5Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiEmojiHappy;
impl IconShape for HiEmojiHappy {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7 9C7.55228 9 8 8.55228 8 8C8 7.44772 7.55228 7 7 7C6.44772 7 6 7.44772 6 8C6 8.55228 6.44772 9 7 9ZM14 8C14 8.55228 13.5523 9 13 9C12.4477 9 12 8.55228 12 8C12 7.44772 12.4477 7 13 7C13.5523 7 14 7.44772 14 8ZM13.5355 13.5354C13.9261 13.1449 13.9261 12.5118 13.5355 12.1212C13.145 11.7307 12.5118 11.7307 12.1213 12.1212C10.9497 13.2928 9.05025 13.2928 7.87868 12.1212C7.48816 11.7307 6.85499 11.7307 6.46447 12.1212C6.07394 12.5118 6.07394 13.1449 6.46447 13.5354C8.41709 15.4881 11.5829 15.4881 13.5355 13.5354Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiEmojiSad;
impl IconShape for HiEmojiSad {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7 9C7.55228 9 8 8.55228 8 8C8 7.44772 7.55228 7 7 7C6.44772 7 6 7.44772 6 8C6 8.55228 6.44772 9 7 9ZM14 8C14 8.55228 13.5523 9 13 9C12.4477 9 12 8.55228 12 8C12 7.44772 12.4477 7 13 7C13.5523 7 14 7.44772 14 8ZM6.46447 13.8785C6.85499 14.269 7.48816 14.269 7.87868 13.8785C9.05025 12.7069 10.9497 12.7069 12.1213 13.8785C12.5118 14.269 13.145 14.269 13.5355 13.8785C13.9261 13.4879 13.9261 12.8548 13.5355 12.4642C11.5829 10.5116 8.41709 10.5116 6.46447 12.4642C6.07394 12.8548 6.07394 13.4879 6.46447 13.8785Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiExclamationCircle;
impl IconShape for HiExclamationCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM11 14C11 14.5523 10.5523 15 10 15C9.44772 15 9 14.5523 9 14C9 13.4477 9.44772 13 10 13C10.5523 13 11 13.4477 11 14ZM10 5C9.44772 5 9 5.44772 9 6V10C9 10.5523 9.44772 11 10 11C10.5523 11 11 10.5523 11 10V6C11 5.44772 10.5523 5 10 5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiExclamation;
impl IconShape for HiExclamation {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8.25706 3.09882C9.02167 1.73952 10.9788 1.73952 11.7434 3.09882L17.3237 13.0194C18.0736 14.3526 17.1102 15.9999 15.5805 15.9999H4.4199C2.89025 15.9999 1.92682 14.3526 2.67675 13.0194L8.25706 3.09882ZM11.0001 13C11.0001 13.5523 10.5524 14 10.0001 14C9.44784 14 9.00012 13.5523 9.00012 13C9.00012 12.4477 9.44784 12 10.0001 12C10.5524 12 11.0001 12.4477 11.0001 13ZM10.0001 5C9.44784 5 9.00012 5.44772 9.00012 6V9C9.00012 9.55228 9.44784 10 10.0001 10C10.5524 10 11.0001 9.55228 11.0001 9V6C11.0001 5.44772 10.5524 5 10.0001 5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiExternalLink;
impl IconShape for HiExternalLink {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 3C10.4477 3 10 3.44772 10 4C10 4.55228 10.4477 5 11 5H13.5858L7.29289 11.2929C6.90237 11.6834 6.90237 12.3166 7.29289 12.7071C7.68342 13.0976 8.31658 13.0976 8.70711 12.7071L15 6.41421V9C15 9.55228 15.4477 10 16 10C16.5523 10 17 9.55228 17 9V4C17 3.44772 16.5523 3 16 3H11Z",
            }
            path {
                d: "M5 5C3.89543 5 3 5.89543 3 7V15C3 16.1046 3.89543 17 5 17H13C14.1046 17 15 16.1046 15 15V12C15 11.4477 14.5523 11 14 11C13.4477 11 13 11.4477 13 12V15H5V7L8 7C8.55228 7 9 6.55228 9 6C9 5.44772 8.55228 5 8 5H5Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiEyeOff;
impl IconShape for HiEyeOff {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.70711 2.29289C3.31658 1.90237 2.68342 1.90237 2.29289 2.29289C1.90237 2.68342 1.90237 3.31658 2.29289 3.70711L16.2929 17.7071C16.6834 18.0976 17.3166 18.0976 17.7071 17.7071C18.0976 17.3166 18.0976 16.6834 17.7071 16.2929L16.2339 14.8197C17.7715 13.5924 18.939 11.9211 19.5424 9.99996C18.2681 5.94288 14.4778 3 10.0002 3C8.37665 3 6.84344 3.38692 5.48779 4.07358L3.70711 2.29289ZM7.96813 6.55391L9.48201 8.0678C9.6473 8.02358 9.82102 8 10.0003 8C11.1048 8 12.0003 8.89543 12.0003 10C12.0003 10.1792 11.9767 10.353 11.9325 10.5182L13.4463 12.0321C13.7983 11.4366 14.0003 10.7419 14.0003 10C14.0003 7.79086 12.2094 6 10.0003 6C9.25838 6 8.56367 6.20197 7.96813 6.55391Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12.4541 16.6967L9.74965 13.9923C7.74013 13.8681 6.1322 12.2601 6.00798 10.2506L2.33492 6.57754C1.50063 7.57223 0.856368 8.73169 0.458008 10C1.73228 14.0571 5.52257 17 10.0002 17C10.8469 17 11.6689 16.8948 12.4541 16.6967Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiEye;
impl IconShape for HiEye {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 12C11.1046 12 12 11.1046 12 10C12 8.89543 11.1046 8 10 8C8.89544 8 8.00001 8.89543 8.00001 10C8.00001 11.1046 8.89544 12 10 12Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M0.457764 10C1.73202 5.94291 5.52232 3 9.99997 3C14.4776 3 18.2679 5.94288 19.5422 9.99996C18.2679 14.0571 14.4776 17 9.99995 17C5.52232 17 1.73204 14.0571 0.457764 10ZM14 10C14 12.2091 12.2091 14 10 14C7.79087 14 6.00001 12.2091 6.00001 10C6.00001 7.79086 7.79087 6 10 6C12.2091 6 14 7.79086 14 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFastForward;
impl IconShape for HiFastForward {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4.5547 5.16795C4.24784 4.96338 3.8533 4.94431 3.52814 5.11833C3.20298 5.29235 3 5.63121 3 6V14C3 14.3688 3.20298 14.7077 3.52814 14.8817C3.8533 15.0557 4.24784 15.0366 4.5547 14.8321L10 11.2019V14C10 14.3688 10.203 14.7077 10.5281 14.8817C10.8533 15.0557 11.2478 15.0366 11.5547 14.8321L17.5547 10.8321C17.8329 10.6466 18 10.3344 18 10C18 9.66565 17.8329 9.35342 17.5547 9.16795L11.5547 5.16795C11.2478 4.96338 10.8533 4.94431 10.5281 5.11833C10.203 5.29235 10 5.63121 10 6V8.79815L4.5547 5.16795Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFilm;
impl IconShape for HiFilm {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 3C2.89543 3 2 3.89543 2 5V15C2 16.1046 2.89543 17 4 17H16C17.1046 17 18 16.1046 18 15V5C18 3.89543 17.1046 3 16 3H4ZM7 5L13 5V9H7V5ZM15 13V15H16V13H15ZM13 11H7V15H13V11ZM15 11H16V9H15V11ZM16 7V5H15V7H16ZM5 5V7H4V5H5ZM5 9H4V11H5V9ZM4 13H5V15H4V13Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFilter;
impl IconShape for HiFilter {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 3C3 2.44772 3.44772 2 4 2H16C16.5523 2 17 2.44772 17 3V6C17 6.26522 16.8946 6.51957 16.7071 6.70711L12 11.4142V15C12 15.2652 11.8946 15.5196 11.7071 15.7071L9.70711 17.7071C9.42111 17.9931 8.99099 18.0787 8.61732 17.9239C8.24364 17.7691 8 17.4045 8 17V11.4142L3.29289 6.70711C3.10536 6.51957 3 6.26522 3 6V3Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFingerPrint;
impl IconShape for HiFingerPrint {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M6.62478 2.65458C7.6684 2.23213 8.80833 2 10 2C14.9706 2 19 6.02944 19 11C19 11.5523 18.5523 12 18 12C17.4477 12 17 11.5523 17 11C17 7.13401 13.866 4 10 4C9.06987 4 8.18446 4.18088 7.37522 4.50845C6.86328 4.71568 6.28029 4.46867 6.07306 3.95673C5.86584 3.4448 6.11285 2.8618 6.62478 2.65458ZM4.66173 4.95861C5.0758 5.32408 5.1152 5.95602 4.74974 6.37008C3.66007 7.60467 3 9.22404 3 11C3 11.5523 2.55228 12 2 12C1.44772 12 1 11.5523 1 11C1 8.71818 1.85048 6.63256 3.25026 5.04662C3.61573 4.63255 4.24766 4.59315 4.66173 4.95861Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M5 11C5 8.23858 7.23857 6 10 6C12.7614 6 15 8.23858 15 11C15 11.5523 14.5523 12 14 12C13.4477 12 13 11.5523 13 11C13 9.34315 11.6569 8 10 8C8.34315 8 7 9.34315 7 11C7 12.6772 6.65535 14.2764 6.03206 15.7288C5.81426 16.2363 5.22626 16.4712 4.71874 16.2533C4.21122 16.0355 3.97636 15.4475 4.19416 14.94C4.71247 13.7323 5 12.401 5 11ZM13.9212 13.0123C14.4666 13.0989 14.8387 13.6112 14.7521 14.1567C14.6205 14.9867 14.4378 15.7998 14.2072 16.5928C14.0531 17.1231 13.4982 17.428 12.9679 17.2739C12.4375 17.1197 12.1326 16.5648 12.2868 16.0345C12.494 15.3215 12.6584 14.5901 12.7768 13.8433C12.8634 13.2979 13.3757 12.9258 13.9212 13.0123Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M10 10C10.5523 10 11 10.4477 11 11C11 13.2363 10.5406 15.3679 9.71014 17.3036C9.49239 17.8111 8.90441 18.046 8.39687 17.8283C7.88933 17.6105 7.65441 17.0225 7.87217 16.515C8.59772 14.8239 9 12.9602 9 11C9 10.4477 9.44771 10 10 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFire;
impl IconShape for HiFire {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.3945 2.55279C12.2662 2.29624 12.034 2.10713 11.7568 2.03351C11.4795 1.95988 11.184 2.00885 10.9454 2.16795C10.5995 2.39858 10.3314 2.72608 10.1229 3.04791C9.90855 3.37854 9.71986 3.76148 9.553 4.16366C9.21939 4.96773 8.93911 5.93195 8.71375 6.89778C8.42752 8.12448 8.21568 9.41687 8.10004 10.4776C7.61585 10.1512 7.33491 9.78527 7.15481 9.41104C6.82729 8.73046 6.75736 7.8772 6.75736 6.75739C6.75736 6.35292 6.51372 5.98829 6.14004 5.83351C5.76637 5.67872 5.33625 5.76428 5.05025 6.05028C3.68361 7.41692 3 9.21013 3 11C3 12.7899 3.68361 14.5831 5.05025 15.9498C7.78392 18.6834 12.2161 18.6834 14.9497 15.9498C16.3164 14.5831 17 12.7899 17 11C17 9.21013 16.3164 7.41692 14.9497 6.05028C14.3584 5.45889 13.9696 5.06453 13.6021 4.5828C13.239 4.10688 12.8781 3.51991 12.3945 2.55279ZM12.1213 15.1213C10.9497 16.2929 9.05025 16.2929 7.87868 15.1213C7.29289 14.5355 7 13.7678 7 13C7 13 7.87868 13.5 9.50005 13.5C9.50005 12.5 10 9.5 10.75 9C11.25 10 11.5355 10.2929 12.1213 10.8787C12.7071 11.4645 13 12.2322 13 13C13 13.7678 12.7071 14.5355 12.1213 15.1213Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFlag;
impl IconShape for HiFlag {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 6C3 4.34315 4.34315 3 6 3H16C16.3788 3 16.725 3.214 16.8944 3.55279C17.0638 3.89157 17.0273 4.29698 16.8 4.6L14.25 8L16.8 11.4C17.0273 11.703 17.0638 12.1084 16.8944 12.4472C16.725 12.786 16.3788 13 16 13H6C5.44772 13 5 13.4477 5 14V17C5 17.5523 4.55228 18 4 18C3.44772 18 3 17.5523 3 17V6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFolderAdd;
impl IconShape for HiFolderAdd {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4C2.89543 4 2 4.89543 2 6V14C2 15.1046 2.89543 16 4 16H16C17.1046 16 18 15.1046 18 14V8C18 6.89543 17.1046 6 16 6H11L9 4H4ZM11 9C11 8.44771 10.5523 8 10 8C9.44772 8 9 8.44771 9 9V10H8C7.44772 10 7 10.4477 7 11C7 11.5523 7.44772 12 8 12H9V13C9 13.5523 9.44772 14 10 14C10.5523 14 11 13.5523 11 13V12H12C12.5523 12 13 11.5523 13 11C13 10.4477 12.5523 10 12 10H11V9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFolderDownload;
impl IconShape for HiFolderDownload {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4C2.89543 4 2 4.89543 2 6V14C2 15.1046 2.89543 16 4 16H16C17.1046 16 18 15.1046 18 14V8C18 6.89543 17.1046 6 16 6H11L9 4H4ZM11 9C11 8.44771 10.5523 8 10 8C9.44772 8 9 8.44771 9 9V10.5858L8.70711 10.2929C8.31658 9.90237 7.68342 9.90237 7.29289 10.2929C6.90237 10.6834 6.90237 11.3166 7.29289 11.7071L9.2926 13.7068L9.29289 13.7071L9.29502 13.7092C9.3904 13.804 9.50014 13.8757 9.61722 13.9241C9.73512 13.973 9.86441 14 10 14C10.1356 14 10.2649 13.973 10.3828 13.9241C10.4999 13.8757 10.6096 13.804 10.705 13.7092L10.7071 13.7071L10.7074 13.7068L12.7071 11.7071C13.0976 11.3166 13.0976 10.6834 12.7071 10.2929C12.3166 9.90237 11.6834 9.90237 11.2929 10.2929L11 10.5858V9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFolderOpen;
impl IconShape for HiFolderOpen {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 6C2 4.89543 2.89543 4 4 4H8L10 6H14C15.1046 6 16 6.89543 16 8V9H8C6.34315 9 5 10.3431 5 12V13.5C5 14.3284 4.32843 15 3.5 15C2.67157 15 2 14.3284 2 13.5V6Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M6 12C6 10.8954 6.89543 10 8 10H16C17.1046 10 18 10.8954 18 12V14C18 15.1046 17.1046 16 16 16H2H4C5.10457 16 6 15.1046 6 14V12Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFolderRemove;
impl IconShape for HiFolderRemove {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M4 4C2.89543 4 2 4.89543 2 6V14C2 15.1046 2.89543 16 4 16H16C17.1046 16 18 15.1046 18 14V8C18 6.89543 17.1046 6 16 6H11L9 4H4ZM8 10C7.44772 10 7 10.4477 7 11C7 11.5523 7.44772 12 8 12H12C12.5523 12 13 11.5523 13 11C13 10.4477 12.5523 10 12 10H8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiFolder;
impl IconShape for HiFolder {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 6C2 4.89543 2.89543 4 4 4H9L11 6H16C17.1046 6 18 6.89543 18 8V14C18 15.1046 17.1046 16 16 16H4C2.89543 16 2 15.1046 2 14V6Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiGift;
impl IconShape for HiGift {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 5C5 3.34315 6.34315 2 8 2C8.76836 2 9.46924 2.28885 10 2.7639C10.5308 2.28885 11.2316 2 12 2C13.6569 2 15 3.34315 15 5C15 5.35064 14.9398 5.68722 14.8293 6H16C17.1046 6 18 6.89543 18 8C18 9.10457 17.1046 10 16 10H11V9C11 8.44772 10.5523 8 10 8C9.44772 8 9 8.44771 9 9V10H4C2.89543 10 2 9.10457 2 8C2 6.89543 2.89543 6 4 6H5.17071C5.06015 5.68722 5 5.35064 5 5ZM9 6V5C9 4.44772 8.55228 4 8 4C7.44772 4 7 4.44772 7 5C7 5.55228 7.44772 6 8 6H9ZM12 6C12.5523 6 13 5.55228 13 5C13 4.44772 12.5523 4 12 4C11.4477 4 11 4.44772 11 5V6H12Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M9 11H3V16C3 17.1046 3.89543 18 5 18H9V11Z",
            }
            path {
                d: "M11 18H15C16.1046 18 17 17.1046 17 16V11H11V18Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiGlobeAlt;
impl IconShape for HiGlobeAlt {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.08296 9H6.02863C6.11783 7.45361 6.41228 6.02907 6.86644 4.88228C5.41752 5.77135 4.37513 7.25848 4.08296 9ZM10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2ZM10 4C9.92395 4 9.76787 4.03173 9.5347 4.26184C9.29723 4.4962 9.03751 4.8849 8.79782 5.44417C8.40914 6.3511 8.12491 7.58559 8.03237 9H11.9676C11.8751 7.58559 11.5909 6.3511 11.2022 5.44417C10.9625 4.8849 10.7028 4.4962 10.4653 4.26184C10.2321 4.03173 10.076 4 10 4ZM13.9714 9C13.8822 7.45361 13.5877 6.02907 13.1336 4.88228C14.5825 5.77135 15.6249 7.25848 15.917 9H13.9714ZM11.9676 11H8.03237C8.12491 12.4144 8.40914 13.6489 8.79782 14.5558C9.03751 15.1151 9.29723 15.5038 9.5347 15.7382C9.76787 15.9683 9.92395 16 10 16C10.076 16 10.2321 15.9683 10.4653 15.7382C10.7028 15.5038 10.9625 15.1151 11.2022 14.5558C11.5909 13.6489 11.8751 12.4144 11.9676 11ZM13.1336 15.1177C13.5877 13.9709 13.8822 12.5464 13.9714 11H15.917C15.6249 12.7415 14.5825 14.2287 13.1336 15.1177ZM6.86644 15.1177C6.41228 13.9709 6.11783 12.5464 6.02863 11H4.08296C4.37513 12.7415 5.41752 14.2287 6.86644 15.1177Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiGlobe;
impl IconShape for HiGlobe {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM4.33179 8.02741C4.70542 6.95361 5.37558 6.01864 6.24421 5.32056C6.51209 5.72966 6.97449 5.99991 7.50001 5.99991C8.32844 5.99991 9.00001 6.67148 9.00001 7.49991V7.99991C9.00001 9.10448 9.89545 9.99991 11 9.99991C12.1046 9.99991 13 9.10448 13 7.99991C13 7.05979 13.6487 6.27118 14.5228 6.05719C15.4428 7.11161 16 8.49069 16 9.99992C16 10.3407 15.9716 10.6748 15.917 11.0001H15C13.8954 11.0001 13 11.8955 13 13.0001V15.1973C12.1175 15.7078 11.0928 15.9999 9.99992 15.9999V14C9.99992 12.8954 9.10448 12 7.99992 12C6.89535 12 5.99992 11.1046 5.99992 10C5.99992 9.00849 5.27841 8.1855 4.33179 8.02741Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiHand;
impl IconShape for HiHand {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9 3C9 2.44772 9.44772 2 10 2C10.5523 2 11 2.44772 11 3V8V8.5C11 8.77614 11.2239 9 11.5 9C11.7761 9 12 8.77614 12 8.5V8V4C12 3.44772 12.4477 3 13 3C13.5523 3 14 3.44772 14 4V8V8.5C14 8.77614 14.2239 9 14.5 9C14.7761 9 15 8.77614 15 8.5V8V6C15 5.44772 15.4477 5 16 5C16.5523 5 17 5.44772 17 6V11C17 14.866 13.866 18 10 18C6.13401 18 3 14.866 3 11V9C3 8.44772 3.44772 8 4 8C4.55228 8 5 8.44772 5 9V11V11.5C5 11.7761 5.22386 12 5.5 12C5.77614 12 6 11.7761 6 11.5V11V10V8V4C6 3.44772 6.44772 3 7 3C7.55228 3 8 3.44772 8 4V8V8.5C8 8.77614 8.22386 9 8.5 9C8.77614 9 9 8.77614 9 8.5V8V3Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiHashtag;
impl IconShape for HiHashtag {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.24254 3.02985C9.77833 3.1638 10.1041 3.70673 9.97014 4.24253L9.53078 5.99999H12.4692L13.0299 3.75746C13.1638 3.22166 13.7067 2.8959 14.2425 3.02985C14.7783 3.1638 15.1041 3.70673 14.9701 4.24253L14.5308 5.99999H17C17.5523 5.99999 18 6.44771 18 6.99999C18 7.55228 17.5523 7.99999 17 7.99999H14.0308L13.0308 12H15C15.5523 12 16 12.4477 16 13C16 13.5523 15.5523 14 15 14H12.5308L11.9701 16.2425C11.8362 16.7783 11.2933 17.1041 10.7575 16.9701C10.2217 16.8362 9.89591 16.2933 10.0299 15.7575L10.4692 14H7.53078L6.97014 16.2425C6.83619 16.7783 6.29326 17.1041 5.75746 16.9701C5.22167 16.8362 4.89591 16.2933 5.02986 15.7575L5.46922 14H3C2.44772 14 2 13.5523 2 13C2 12.4477 2.44772 12 3 12H5.96922L6.96922 7.99999H5C4.44772 7.99999 4 7.55228 4 6.99999C4 6.44771 4.44772 5.99999 5 5.99999H7.46922L8.02986 3.75746C8.16381 3.22166 8.70674 2.8959 9.24254 3.02985ZM9.03078 7.99999L8.03078 12H10.9692L11.9692 7.99999H9.03078Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiHeart;
impl IconShape for HiHeart {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3.17157 5.17157C4.73367 3.60948 7.26633 3.60948 8.82843 5.17157L10 6.34315L11.1716 5.17157C12.7337 3.60948 15.2663 3.60948 16.8284 5.17157C18.3905 6.73367 18.3905 9.26633 16.8284 10.8284L10 17.6569L3.17157 10.8284C1.60948 9.26633 1.60948 6.73367 3.17157 5.17157Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiHome;
impl IconShape for HiHome {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.7071 2.29289C10.3166 1.90237 9.68342 1.90237 9.29289 2.29289L2.29289 9.29289C1.90237 9.68342 1.90237 10.3166 2.29289 10.7071C2.68342 11.0976 3.31658 11.0976 3.70711 10.7071L4 10.4142V17C4 17.5523 4.44772 18 5 18H7C7.55228 18 8 17.5523 8 17V15C8 14.4477 8.44772 14 9 14H11C11.5523 14 12 14.4477 12 15V17C12 17.5523 12.4477 18 13 18H15C15.5523 18 16 17.5523 16 17V10.4142L16.2929 10.7071C16.6834 11.0976 17.3166 11.0976 17.7071 10.7071C18.0976 10.3166 18.0976 9.68342 17.7071 9.29289L10.7071 2.29289Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiIdentification;
impl IconShape for HiIdentification {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 2C9.44772 2 9 2.44772 9 3V4C9 4.55228 9.44772 5 10 5C10.5523 5 11 4.55228 11 4V3C11 2.44772 10.5523 2 10 2ZM4 4H7C7 5.65685 8.34315 7 10 7C11.6569 7 13 5.65685 13 4H16C17.1046 4 18 4.89543 18 6V15C18 16.1046 17.1046 17 16 17H4C2.89543 17 2 16.1046 2 15V6C2 4.89543 2.89543 4 4 4ZM6.5 11C7.32843 11 8 10.3284 8 9.5C8 8.67157 7.32843 8 6.5 8C5.67157 8 5 8.67157 5 9.5C5 10.3284 5.67157 11 6.5 11ZM8.95048 15C8.98327 14.8384 9.00049 14.6712 9.00049 14.5C9.00049 13.1193 7.8812 12 6.50049 12C5.11978 12 4.00049 13.1193 4.00049 14.5C4.00049 14.6712 4.0177 14.8384 4.0505 15H8.95048ZM12 9C11.4477 9 11 9.44772 11 10C11 10.5523 11.4477 11 12 11H15C15.5523 11 16 10.5523 16 10C16 9.44772 15.5523 9 15 9H12ZM11 13C11 12.4477 11.4477 12 12 12H14C14.5523 12 15 12.4477 15 13C15 13.5523 14.5523 14 14 14H12C11.4477 14 11 13.5523 11 13Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiInboxIn;
impl IconShape for HiInboxIn {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.70711 7.29289C8.31658 6.90237 7.68342 6.90237 7.29289 7.29289C6.90237 7.68342 6.90237 8.31658 7.29289 8.70711L9.29289 10.7071C9.48043 10.8946 9.73478 11 10 11C10.2652 11 10.5196 10.8946 10.7071 10.7071L12.7071 8.70711C13.0976 8.31658 13.0976 7.68342 12.7071 7.29289C12.3166 6.90237 11.6834 6.90237 11.2929 7.29289L11 7.58579L11 3C11 2.44771 10.5523 2 10 2C9.44771 2 9 2.44772 9 3V7.58579L8.70711 7.29289Z",
            }
            path {
                d: "M3 5C3 3.89543 3.89543 3 5 3H6C6.55228 3 7 3.44772 7 4C7 4.55228 6.55228 5 6 5L5 5V12H7L8 14H12L13 12H15V5H14C13.4477 5 13 4.55228 13 4C13 3.44772 13.4477 3 14 3H15C16.1046 3 17 3.89543 17 5V15C17 16.1046 16.1046 17 15 17H5C3.89543 17 3 16.1046 3 15V5Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiInbox;
impl IconShape for HiInbox {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 3C3.89543 3 3 3.89543 3 5V15C3 16.1046 3.89543 17 5 17H15C16.1046 17 17 16.1046 17 15V5C17 3.89543 16.1046 3 15 3H5ZM5 5L15 5V12H13L12 14H8L7 12H5V5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiInformationCircle;
impl IconShape for HiInformationCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM11 6C11 6.55228 10.5523 7 10 7C9.44772 7 9 6.55228 9 6C9 5.44772 9.44772 5 10 5C10.5523 5 11 5.44772 11 6ZM9 9C8.44772 9 8 9.44772 8 10C8 10.5523 8.44772 11 9 11V14C9 14.5523 9.44772 15 10 15H11C11.5523 15 12 14.5523 12 14C12 13.4477 11.5523 13 11 13V10C11 9.44772 10.5523 9 10 9H9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiKey;
impl IconShape for HiKey {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 8C18 11.3137 15.3137 14 12 14C11.3938 14 10.8087 13.9101 10.2571 13.7429L10 14L9 15L8 16H6V18H2V14L6.25707 9.74293C6.08989 9.19135 6 8.60617 6 8C6 4.68629 8.68629 2 12 2C15.3137 2 18 4.68629 18 8ZM12 4C11.4477 4 11 4.44772 11 5C11 5.55228 11.4477 6 12 6C13.1046 6 14 6.89543 14 8C14 8.55228 14.4477 9 15 9C15.5523 9 16 8.55228 16 8C16 5.79086 14.2091 4 12 4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLibrary;
impl IconShape for HiLibrary {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10.4963 2.13176C10.1889 1.95608 9.81146 1.95608 9.50403 2.13176L2.50403 6.13176C2.02451 6.40577 1.85792 7.01662 2.13193 7.49614C2.31631 7.81881 2.65322 7.99979 3 8.00017V15C2.44772 15 2 15.4477 2 16C2 16.5523 2.44772 17 3 17H17C17.5523 17 18 16.5523 18 16C18 15.4477 17.5523 15 17 15V8.00017C17.3469 7.99991 17.684 7.81892 17.8684 7.49614C18.1424 7.01662 17.9758 6.40577 17.4963 6.13176L10.4963 2.13176ZM6 9C5.44772 9 5 9.44772 5 10V13C5 13.5523 5.44772 14 6 14C6.55228 14 7 13.5523 7 13V10C7 9.44772 6.55228 9 6 9ZM9 10C9 9.44772 9.44772 9 10 9C10.5523 9 11 9.44772 11 10V13C11 13.5523 10.5523 14 10 14C9.44772 14 9 13.5523 9 13V10ZM14 9C13.4477 9 13 9.44772 13 10V13C13 13.5523 13.4477 14 14 14C14.5523 14 15 13.5523 15 13V10C15 9.44772 14.5523 9 14 9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLightBulb;
impl IconShape for HiLightBulb {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 3C11 2.44772 10.5523 2 10 2C9.44771 2 9 2.44772 9 3V4C9 4.55228 9.44771 5 10 5C10.5523 5 11 4.55228 11 4V3Z",
            }
            path {
                d: "M15.6568 5.75731C16.0473 5.36678 16.0473 4.73362 15.6568 4.34309C15.2663 3.95257 14.6331 3.95257 14.2426 4.34309L13.5355 5.0502C13.145 5.44072 13.145 6.07389 13.5355 6.46441C13.926 6.85494 14.5592 6.85494 14.9497 6.46441L15.6568 5.75731Z",
            }
            path {
                d: "M18 10C18 10.5523 17.5523 11 17 11H16C15.4477 11 15 10.5523 15 10C15 9.44771 15.4477 9 16 9H17C17.5523 9 18 9.44771 18 10Z",
            }
            path {
                d: "M5.05019 6.46443C5.44071 6.85496 6.07388 6.85496 6.4644 6.46443C6.85493 6.07391 6.85493 5.44074 6.4644 5.05022L5.7573 4.34311C5.36677 3.95259 4.73361 3.95259 4.34308 4.34311C3.95256 4.73363 3.95256 5.3668 4.34308 5.75732L5.05019 6.46443Z",
            }
            path {
                d: "M5 10C5 10.5523 4.55228 11 4 11H3C2.44772 11 2 10.5523 2 10C2 9.44771 2.44772 9 3 9H4C4.55228 9 5 9.44771 5 10Z",
            }
            path {
                d: "M8 16V15H12V16C12 17.1046 11.1046 18 10 18C8.89543 18 8 17.1046 8 16Z",
            }
            path {
                d: "M12.0009 14C12.0155 13.6597 12.2076 13.3537 12.4768 13.1411C13.4046 12.4086 14 11.2738 14 10C14 7.79086 12.2091 6 10 6C7.79086 6 6 7.79086 6 10C6 11.2738 6.59545 12.4086 7.52319 13.1411C7.79241 13.3537 7.98451 13.6597 7.99911 14H12.0009Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLightningBolt;
impl IconShape for HiLightningBolt {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M11.3006 1.04621C11.7169 1.17743 12 1.56348 12 1.99995V6.99995L16 6.99995C16.3729 6.99995 16.7148 7.20741 16.887 7.53814C17.0592 7.86887 17.0331 8.26794 16.8192 8.57341L9.81924 18.5734C9.56894 18.931 9.11564 19.0849 8.69936 18.9537C8.28309 18.8225 8 18.4364 8 18L8 13H4C3.62713 13 3.28522 12.7925 3.11302 12.4618C2.94083 12.131 2.96694 11.732 3.18077 11.4265L10.1808 1.42649C10.4311 1.06892 10.8844 0.914992 11.3006 1.04621Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLink;
impl IconShape for HiLink {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12.5858 4.58579C13.3668 3.80474 14.6331 3.80474 15.4142 4.58579C16.1952 5.36683 16.1952 6.63316 15.4142 7.41421L12.4142 10.4142C11.6331 11.1953 10.3668 11.1953 9.58577 10.4142C9.19524 10.0237 8.56208 10.0237 8.17156 10.4142C7.78103 10.8047 7.78103 11.4379 8.17156 11.8284C9.73365 13.3905 12.2663 13.3905 13.8284 11.8284L16.8284 8.82843C18.3905 7.26633 18.3905 4.73367 16.8284 3.17157C15.2663 1.60948 12.7337 1.60948 11.1716 3.17157L9.67156 4.67157C9.28103 5.0621 9.28103 5.69526 9.67156 6.08579C10.0621 6.47631 10.6952 6.47631 11.0858 6.08579L12.5858 4.58579ZM7.58579 9.58579C8.36683 8.80474 9.63316 8.80474 10.4142 9.58579C10.8047 9.97631 11.4379 9.97631 11.8284 9.58579C12.219 9.19526 12.219 8.5621 11.8284 8.17157C10.2663 6.60948 7.73367 6.60948 6.17157 8.17157L3.17157 11.1716C1.60948 12.7337 1.60948 15.2663 3.17157 16.8284C4.73367 18.3905 7.26633 18.3905 8.82843 16.8284L10.3284 15.3284C10.719 14.9379 10.719 14.3047 10.3284 13.9142C9.9379 13.5237 9.30474 13.5237 8.91421 13.9142L7.41421 15.4142C6.63316 16.1953 5.36684 16.1953 4.58579 15.4142C3.80474 14.6332 3.80474 13.3668 4.58579 12.5858L7.58579 9.58579Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLocationMarker;
impl IconShape for HiLocationMarker {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.05025 4.05025C7.78392 1.31658 12.2161 1.31658 14.9497 4.05025C17.6834 6.78392 17.6834 11.2161 14.9497 13.9497L10 18.8995L5.05025 13.9497C2.31658 11.2161 2.31658 6.78392 5.05025 4.05025ZM10 11C11.1046 11 12 10.1046 12 9C12 7.89543 11.1046 7 10 7C8.89543 7 8 7.89543 8 9C8 10.1046 8.89543 11 10 11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLockClosed;
impl IconShape for HiLockClosed {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 9V7C5 4.23858 7.23858 2 10 2C12.7614 2 15 4.23858 15 7V9C16.1046 9 17 9.89543 17 11V16C17 17.1046 16.1046 18 15 18H5C3.89543 18 3 17.1046 3 16V11C3 9.89543 3.89543 9 5 9ZM13 7V9H7V7C7 5.34315 8.34315 4 10 4C11.6569 4 13 5.34315 13 7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLockOpen;
impl IconShape for HiLockOpen {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 2C7.23858 2 5 4.23858 5 7V9C3.89543 9 3 9.89543 3 11V16C3 17.1046 3.89543 18 5 18H15C16.1046 18 17 17.1046 17 16V11C17 9.89543 16.1046 9 15 9H7V7C7 5.34315 8.34315 4 10 4C11.3965 4 12.5725 4.95512 12.9055 6.24926C13.0432 6.78411 13.5884 7.1061 14.1232 6.96844C14.6581 6.83078 14.9801 6.28559 14.8424 5.75074C14.2874 3.59442 12.3312 2 10 2Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLogin;
impl IconShape for HiLogin {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 3C3.55229 3 4 3.44771 4 4L4 16C4 16.5523 3.55228 17 3 17C2.44771 17 2 16.5523 2 16L2 4C2 3.44771 2.44772 3 3 3ZM10.7071 6.29289C11.0976 6.68342 11.0976 7.31658 10.7071 7.70711L9.41421 9L17 9C17.5523 9 18 9.44771 18 10C18 10.5523 17.5523 11 17 11L9.41421 11L10.7071 12.2929C11.0976 12.6834 11.0976 13.3166 10.7071 13.7071C10.3166 14.0976 9.68342 14.0976 9.29289 13.7071L6.29289 10.7071C6.10536 10.5196 6 10.2652 6 10C6 9.73478 6.10536 9.48043 6.29289 9.29289L9.29289 6.29289C9.68342 5.90237 10.3166 5.90237 10.7071 6.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiLogout;
impl IconShape for HiLogout {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 3C2.44772 3 2 3.44772 2 4V16C2 16.5523 2.44772 17 3 17C3.55228 17 4 16.5523 4 16V4C4 3.44772 3.55228 3 3 3ZM13.2929 12.2929C12.9024 12.6834 12.9024 13.3166 13.2929 13.7071C13.6834 14.0976 14.3166 14.0976 14.7071 13.7071L17.7071 10.7071C17.8946 10.5196 18 10.2652 18 10C18 9.73478 17.8946 9.48043 17.7071 9.29289L14.7071 6.29289C14.3166 5.90237 13.6834 5.90237 13.2929 6.29289C12.9024 6.68342 12.9024 7.31658 13.2929 7.70711L14.5858 9L7 9C6.44771 9 6 9.44772 6 10C6 10.5523 6.44772 11 7 11H14.5858L13.2929 12.2929Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMailOpen;
impl IconShape for HiMailOpen {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.94 6.4124C2.35524 6.77788 2 7.41882 2 8.1084V15.9999C2 17.1045 2.89543 17.9999 4 17.9999H16C17.1046 17.9999 18 17.1045 18 15.9999V8.1084C18 7.41882 17.6448 6.77788 17.06 6.4124L11.06 2.6624C10.4115 2.25706 9.58854 2.25706 8.94 2.6624L2.94 6.4124ZM5.5547 8.83462C5.09517 8.52826 4.4743 8.65244 4.16795 9.11197C3.8616 9.5715 3.98577 10.1924 4.4453 10.4987L9.4453 13.8321C9.7812 14.056 10.2188 14.056 10.5547 13.8321L15.5547 10.4987C16.0142 10.1924 16.1384 9.5715 15.8321 9.11197C15.5257 8.65244 14.9048 8.52826 14.4453 8.83462L10 11.7981L5.5547 8.83462Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMail;
impl IconShape for HiMail {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2.00333 5.88355L9.99995 9.88186L17.9967 5.8835C17.9363 4.83315 17.0655 4 16 4H4C2.93452 4 2.06363 4.83318 2.00333 5.88355Z",
            }
            path {
                d: "M18 8.1179L9.99995 12.1179L2 8.11796V14C2 15.1046 2.89543 16 4 16H16C17.1046 16 18 15.1046 18 14V8.1179Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMap;
impl IconShape for HiMap {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12 1.58582L8 5.58582V18.4142L12 14.4142V1.58582Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3.70711 3.29292C3.42111 3.00692 2.99099 2.92137 2.61732 3.07615C2.24364 3.23093 2 3.59557 2 4.00003V14C2 14.2652 2.10536 14.5196 2.29289 14.7071L6 18.4142V5.58582L3.70711 3.29292Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M17.7071 5.29292L14 1.58582V14.4142L16.2929 16.7071C16.5789 16.9931 17.009 17.0787 17.3827 16.9239C17.7564 16.7691 18 16.4045 18 16V6.00003C18 5.73481 17.8946 5.48046 17.7071 5.29292Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMenuAlt1;
impl IconShape for HiMenuAlt1 {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 5C3 4.44772 3.44772 4 4 4H16C16.5523 4 17 4.44772 17 5C17 5.55228 16.5523 6 16 6H4C3.44772 6 3 5.55228 3 5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 10C3 9.44772 3.44772 9 4 9H10C10.5523 9 11 9.44772 11 10C11 10.5523 10.5523 11 10 11H4C3.44772 11 3 10.5523 3 10Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 15C3 14.4477 3.44772 14 4 14H16C16.5523 14 17 14.4477 17 15C17 15.5523 16.5523 16 16 16H4C3.44772 16 3 15.5523 3 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMenuAlt2;
impl IconShape for HiMenuAlt2 {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 5C3 4.44772 3.44772 4 4 4H16C16.5523 4 17 4.44772 17 5C17 5.55228 16.5523 6 16 6H4C3.44772 6 3 5.55228 3 5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 10C3 9.44772 3.44772 9 4 9H16C16.5523 9 17 9.44772 17 10C17 10.5523 16.5523 11 16 11H4C3.44772 11 3 10.5523 3 10Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 15C3 14.4477 3.44772 14 4 14H10C10.5523 14 11 14.4477 11 15C11 15.5523 10.5523 16 10 16H4C3.44772 16 3 15.5523 3 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMenuAlt3;
impl IconShape for HiMenuAlt3 {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 5C3 4.44772 3.44772 4 4 4H16C16.5523 4 17 4.44772 17 5C17 5.55228 16.5523 6 16 6H4C3.44772 6 3 5.55228 3 5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 10C3 9.44772 3.44772 9 4 9H16C16.5523 9 17 9.44772 17 10C17 10.5523 16.5523 11 16 11H4C3.44772 11 3 10.5523 3 10Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M9 15C9 14.4477 9.44772 14 10 14H16C16.5523 14 17 14.4477 17 15C17 15.5523 16.5523 16 16 16H10C9.44772 16 9 15.5523 9 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMenuAlt4;
impl IconShape for HiMenuAlt4 {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 7C3 6.44772 3.44772 6 4 6H16C16.5523 6 17 6.44772 17 7C17 7.55228 16.5523 8 16 8H4C3.44772 8 3 7.55228 3 7Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 13C3 12.4477 3.44772 12 4 12H16C16.5523 12 17 12.4477 17 13C17 13.5523 16.5523 14 16 14H4C3.44772 14 3 13.5523 3 13Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMenu;
impl IconShape for HiMenu {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 5C3 4.44772 3.44772 4 4 4H16C16.5523 4 17 4.44772 17 5C17 5.55228 16.5523 6 16 6H4C3.44772 6 3 5.55228 3 5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 10C3 9.44772 3.44772 9 4 9H16C16.5523 9 17 9.44772 17 10C17 10.5523 16.5523 11 16 11H4C3.44772 11 3 10.5523 3 10Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 15C3 14.4477 3.44772 14 4 14H16C16.5523 14 17 14.4477 17 15C17 15.5523 16.5523 16 16 16H4C3.44772 16 3 15.5523 3 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMicrophone;
impl IconShape for HiMicrophone {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7 4C7 2.34315 8.34315 1 10 1C11.6569 1 13 2.34315 13 4V8C13 9.65685 11.6569 11 10 11C8.34315 11 7 9.65685 7 8V4ZM11 14.9291C14.3923 14.4439 17 11.5265 17 8C17 7.44772 16.5523 7 16 7C15.4477 7 15 7.44772 15 8C15 10.7614 12.7614 13 10 13C7.23858 13 5 10.7614 5 8C5 7.44772 4.55228 7 4 7C3.44772 7 3 7.44772 3 8C3 11.5265 5.60771 14.4439 9 14.9291V17H6C5.44772 17 5 17.4477 5 18C5 18.5523 5.44772 19 6 19H14C14.5523 19 15 18.5523 15 18C15 17.4477 14.5523 17 14 17H11V14.9291Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMinusCircle;
impl IconShape for HiMinusCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM7 9C6.44772 9 6 9.44772 6 10C6 10.5523 6.44772 11 7 11H13C13.5523 11 14 10.5523 14 10C14 9.44772 13.5523 9 13 9H7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMinusSm;
impl IconShape for HiMinusSm {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 10C5 9.44772 5.44772 9 6 9L14 9C14.5523 9 15 9.44772 15 10C15 10.5523 14.5523 11 14 11L6 11C5.44772 11 5 10.5523 5 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMinus;
impl IconShape for HiMinus {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 10C3 9.44772 3.44772 9 4 9L16 9C16.5523 9 17 9.44772 17 10C17 10.5523 16.5523 11 16 11L4 11C3.44772 11 3 10.5523 3 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMoon;
impl IconShape for HiMoon {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.2929 13.2929C16.2886 13.7471 15.1738 13.9999 14 13.9999C9.58172 13.9999 6 10.4182 6 5.9999C6 4.82593 6.25287 3.71102 6.70712 2.70667C3.93137 3.96191 2 6.75526 2 9.9997C2 14.418 5.58172 17.9997 10 17.9997C13.2443 17.9997 16.0376 16.0685 17.2929 13.2929Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiMusicNote;
impl IconShape for HiMusicNote {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 3.00001C18 2.70042 17.8657 2.41661 17.634 2.22667C17.4023 2.03673 17.0977 1.96067 16.8039 2.01943L6.80388 4.01943C6.33646 4.11291 6 4.52333 6 5.00001V14.1138C5.68722 14.0401 5.35064 14 5 14C3.34315 14 2 14.8954 2 16C2 17.1046 3.34315 18 5 18C6.65685 18 7.99999 17.1046 8 16V7.81981L16 6.21981V12.1138C15.6872 12.0401 15.3506 12 15 12C13.3431 12 12 12.8954 12 14C12 15.1046 13.3431 16 15 16C16.6569 16 18 15.1046 18 14V3.00001Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiNewspaper;
impl IconShape for HiNewspaper {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 5C2 3.89543 2.89543 3 4 3H12C13.1046 3 14 3.89543 14 5V15C14 16.1046 14.8954 17 16 17H4C2.89543 17 2 16.1046 2 15V5ZM5 6H11V10H5V6ZM11 12H5V14H11V12Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M15 7H16C17.1046 7 18 7.89543 18 9V14.5C18 15.3284 17.3284 16 16.5 16C15.6716 16 15 15.3284 15 14.5V7Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiOfficeBuilding;
impl IconShape for HiOfficeBuilding {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 4C4 2.89543 4.89543 2 6 2H14C15.1046 2 16 2.89543 16 4V16C16.5523 16 17 16.4477 17 17C17 17.5523 16.5523 18 16 18H13C12.4477 18 12 17.5523 12 17V15C12 14.4477 11.5523 14 11 14H9C8.44772 14 8 14.4477 8 15V17C8 17.5523 7.55228 18 7 18H4C3.44772 18 3 17.5523 3 17C3 16.4477 3.44772 16 4 16V4ZM7 5H9V7H7V5ZM9 9H7V11H9V9ZM11 5H13V7H11V5ZM13 9H11V11H13V9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPaperAirplane;
impl IconShape for HiPaperAirplane {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10.8944 2.55279C10.725 2.214 10.3788 2 10 2C9.62124 2 9.27498 2.214 9.10558 2.55279L2.10558 16.5528C1.92823 16.9075 1.97724 17.3335 2.2305 17.6386C2.48376 17.9438 2.89342 18.0705 3.27473 17.9615L8.27472 16.533C8.70402 16.4103 9 16.0179 9 15.5714V11C9 10.4477 9.44772 10 10 10C10.5523 10 11 10.4477 11 11V15.5714C11 16.0179 11.296 16.4103 11.7253 16.533L16.7253 17.9615C17.1066 18.0705 17.5163 17.9438 17.7695 17.6386C18.0228 17.3335 18.0718 16.9075 17.8944 16.5528L10.8944 2.55279Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPaperClip;
impl IconShape for HiPaperClip {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 4C6.34315 4 5 5.34315 5 7V11C5 13.7614 7.23858 16 10 16C12.7614 16 15 13.7614 15 11V7C15 6.44772 15.4477 6 16 6C16.5523 6 17 6.44772 17 7V11C17 14.866 13.866 18 10 18C6.13401 18 3 14.866 3 11V7C3 4.23858 5.23858 2 8 2C10.7614 2 13 4.23858 13 7V11C13 12.6569 11.6569 14 10 14C8.34315 14 7 12.6569 7 11V7C7 6.44772 7.44772 6 8 6C8.55228 6 9 6.44772 9 7V11C9 11.5523 9.44772 12 10 12C10.5523 12 11 11.5523 11 11V7C11 5.34315 9.65685 4 8 4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPause;
impl IconShape for HiPause {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM7 8C7 7.44772 7.44772 7 8 7C8.55228 7 9 7.44772 9 8V12C9 12.5523 8.55228 13 8 13C7.44772 13 7 12.5523 7 12V8ZM12 7C11.4477 7 11 7.44772 11 8V12C11 12.5523 11.4477 13 12 13C12.5523 13 13 12.5523 13 12V8C13 7.44772 12.5523 7 12 7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPencilAlt;
impl IconShape for HiPencilAlt {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.4142 2.58579C16.6332 1.80474 15.3668 1.80474 14.5858 2.58579L7 10.1716V13H9.82842L17.4142 5.41421C18.1953 4.63316 18.1953 3.36683 17.4142 2.58579Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M2 6C2 4.89543 2.89543 4 4 4H8C8.55228 4 9 4.44772 9 5C9 5.55228 8.55228 6 8 6H4V16H14V12C14 11.4477 14.4477 11 15 11C15.5523 11 16 11.4477 16 12V16C16 17.1046 15.1046 18 14 18H4C2.89543 18 2 17.1046 2 16V6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPencil;
impl IconShape for HiPencil {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13.5858 3.58579C14.3668 2.80474 15.6332 2.80474 16.4142 3.58579C17.1953 4.36683 17.1953 5.63316 16.4142 6.41421L15.6213 7.20711L12.7929 4.37868L13.5858 3.58579Z",
            }
            path {
                d: "M11.3787 5.79289L3 14.1716V17H5.82842L14.2071 8.62132L11.3787 5.79289Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPhoneIncoming;
impl IconShape for HiPhoneIncoming {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M14.4142 7L17.7071 3.70711C18.0976 3.31658 18.0976 2.68342 17.7071 2.29289C17.3166 1.90237 16.6834 1.90237 16.2929 2.29289L13 5.58579V4C13 3.44772 12.5523 3 12 3C11.4477 3 11 3.44772 11 4V7.99931C11 8.00031 11 8.002 11 8.003C11.0004 8.1375 11.0273 8.26575 11.0759 8.38278C11.1236 8.49805 11.1937 8.6062 11.2864 8.70055C11.2907 8.70494 11.2951 8.70929 11.2995 8.7136C11.3938 8.80626 11.502 8.87643 11.6172 8.92412C11.7351 8.97301 11.8644 9 12 9H16C16.5523 9 17 8.55228 17 8C17 7.44772 16.5523 7 16 7H14.4142Z",
            }
            path {
                d: "M2 3C2 2.44772 2.44772 2 3 2H5.15287C5.64171 2 6.0589 2.35341 6.13927 2.8356L6.87858 7.27147C6.95075 7.70451 6.73206 8.13397 6.3394 8.3303L4.79126 9.10437C5.90756 11.8783 8.12168 14.0924 10.8956 15.2087L11.6697 13.6606C11.866 13.2679 12.2955 13.0492 12.7285 13.1214L17.1644 13.8607C17.6466 13.9411 18 14.3583 18 14.8471V17C18 17.5523 17.5523 18 17 18H15C7.8203 18 2 12.1797 2 5V3Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPhoneMissedCall;
impl IconShape for HiPhoneMissedCall {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 3C2 2.44772 2.44772 2 3 2H5.15287C5.64171 2 6.0589 2.35341 6.13927 2.8356L6.87858 7.27147C6.95075 7.70451 6.73206 8.13397 6.3394 8.3303L4.79126 9.10437C5.90756 11.8783 8.12168 14.0924 10.8956 15.2087L11.6697 13.6606C11.866 13.2679 12.2955 13.0492 12.7285 13.1214L17.1644 13.8607C17.6466 13.9411 18 14.3583 18 14.8471V17C18 17.5523 17.5523 18 17 18H15C7.8203 18 2 12.1797 2 5V3Z",
            }
            path {
                d: "M16.7071 3.29289C17.0976 3.68342 17.0976 4.31658 16.7071 4.70711L15.4142 6L16.7071 7.29289C17.0976 7.68342 17.0976 8.31658 16.7071 8.70711C16.3166 9.09763 15.6834 9.09763 15.2929 8.70711L14 7.41421L12.7071 8.70711C12.3166 9.09763 11.6834 9.09763 11.2929 8.70711C10.9024 8.31658 10.9024 7.68342 11.2929 7.29289L12.5858 6L11.2929 4.70711C10.9024 4.31658 10.9024 3.68342 11.2929 3.29289C11.6834 2.90237 12.3166 2.90237 12.7071 3.29289L14 4.58579L15.2929 3.29289C15.6834 2.90237 16.3166 2.90237 16.7071 3.29289Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPhoneOutgoing;
impl IconShape for HiPhoneOutgoing {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M17.9241 2.61722C17.8757 2.50014 17.804 2.3904 17.7092 2.29502C17.7078 2.2936 17.7064 2.29219 17.705 2.29078C17.5242 2.11106 17.2751 2 17 2H13C12.4477 2 12 2.44772 12 3C12 3.55228 12.4477 4 13 4H14.5858L11.2929 7.29289C10.9024 7.68342 10.9024 8.31658 11.2929 8.70711C11.6834 9.09763 12.3166 9.09763 12.7071 8.70711L16 5.41421V7C16 7.55228 16.4477 8 17 8C17.5523 8 18 7.55228 18 7V3C18 2.86441 17.973 2.73512 17.9241 2.61722Z",
            }
            path {
                d: "M2 3C2 2.44772 2.44772 2 3 2H5.15287C5.64171 2 6.0589 2.35341 6.13927 2.8356L6.87858 7.27147C6.95075 7.70451 6.73206 8.13397 6.3394 8.3303L4.79126 9.10437C5.90756 11.8783 8.12168 14.0924 10.8956 15.2087L11.6697 13.6606C11.866 13.2679 12.2955 13.0492 12.7285 13.1214L17.1644 13.8607C17.6466 13.9411 18 14.3583 18 14.8471V17C18 17.5523 17.5523 18 17 18H15C7.8203 18 2 12.1797 2 5V3Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPhone;
impl IconShape for HiPhone {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 3C2 2.44772 2.44772 2 3 2H5.15287C5.64171 2 6.0589 2.35341 6.13927 2.8356L6.87858 7.27147C6.95075 7.70451 6.73206 8.13397 6.3394 8.3303L4.79126 9.10437C5.90756 11.8783 8.12168 14.0924 10.8956 15.2087L11.6697 13.6606C11.866 13.2679 12.2955 13.0492 12.7285 13.1214L17.1644 13.8607C17.6466 13.9411 18 14.3583 18 14.8471V17C18 17.5523 17.5523 18 17 18H15C7.8203 18 2 12.1797 2 5V3Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPhotograph;
impl IconShape for HiPhotograph {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 3C2.89543 3 2 3.89543 2 5V15C2 16.1046 2.89543 17 4 17H16C17.1046 17 18 16.1046 18 15V5C18 3.89543 17.1046 3 16 3H4ZM16 15H4L8 7L11 13L13 9L16 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPlay;
impl IconShape for HiPlay {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM9.5547 7.16795C9.24784 6.96338 8.8533 6.94431 8.52814 7.11833C8.20298 7.29235 8 7.63121 8 8V12C8 12.3688 8.20298 12.7077 8.52814 12.8817C8.8533 13.0557 9.24784 13.0366 9.5547 12.8321L12.5547 10.8321C12.8329 10.6466 13 10.3344 13 10C13 9.66565 12.8329 9.35342 12.5547 9.16795L9.5547 7.16795Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPlusCircle;
impl IconShape for HiPlusCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM11 7C11 6.44772 10.5523 6 10 6C9.44772 6 9 6.44772 9 7V9H7C6.44772 9 6 9.44771 6 10C6 10.5523 6.44772 11 7 11H9V13C9 13.5523 9.44772 14 10 14C10.5523 14 11 13.5523 11 13V11H13C13.5523 11 14 10.5523 14 10C14 9.44772 13.5523 9 13 9H11V7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPlusSm;
impl IconShape for HiPlusSm {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 5C10.5523 5 11 5.44772 11 6V9L14 9C14.5523 9 15 9.44772 15 10C15 10.5523 14.5523 11 14 11H11V14C11 14.5523 10.5523 15 10 15C9.44771 15 9 14.5523 9 14V11H6C5.44772 11 5 10.5523 5 10C5 9.44771 5.44772 9 6 9L9 9V6C9 5.44772 9.44771 5 10 5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPlus;
impl IconShape for HiPlus {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 3C10.5523 3 11 3.44772 11 4V9H16C16.5523 9 17 9.44772 17 10C17 10.5523 16.5523 11 16 11H11V16C11 16.5523 10.5523 17 10 17C9.44772 17 9 16.5523 9 16V11H4C3.44772 11 3 10.5523 3 10C3 9.44771 3.44772 9 4 9L9 9V4C9 3.44772 9.44772 3 10 3Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPresentationChartBar;
impl IconShape for HiPresentationChartBar {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 3C2.44772 3 2 3.44772 2 4C2 4.55228 2.44772 5 3 5V13C3 14.1046 3.89543 15 5 15H7.58579L6.29289 16.2929C5.90237 16.6834 5.90237 17.3166 6.29289 17.7071C6.68342 18.0976 7.31658 18.0976 7.70711 17.7071L10 15.4142L12.2929 17.7071C12.6834 18.0976 13.3166 18.0976 13.7071 17.7071C14.0976 17.3166 14.0976 16.6834 13.7071 16.2929L12.4142 15H15C16.1046 15 17 14.1046 17 13V5C17.5523 5 18 4.55228 18 4C18 3.44772 17.5523 3 17 3H3ZM14 7C14 6.44772 13.5523 6 13 6C12.4477 6 12 6.44772 12 7V11C12 11.5523 12.4477 12 13 12C13.5523 12 14 11.5523 14 11V7ZM11 8C11 7.44772 10.5523 7 10 7C9.44772 7 9 7.44772 9 8V11C9 11.5523 9.44772 12 10 12C10.5523 12 11 11.5523 11 11V8ZM8 9C8 8.44772 7.55228 8 7 8C6.44772 8 6 8.44772 6 9V11C6 11.5523 6.44772 12 7 12C7.55228 12 8 11.5523 8 11V9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPresentationChartLine;
impl IconShape for HiPresentationChartLine {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 3C2.44772 3 2 3.44772 2 4C2 4.55228 2.44772 5 3 5V13C3 14.1046 3.89543 15 5 15H7.58579L6.29289 16.2929C5.90237 16.6834 5.90237 17.3166 6.29289 17.7071C6.68342 18.0976 7.31658 18.0976 7.70711 17.7071L10 15.4142L12.2929 17.7071C12.6834 18.0976 13.3166 18.0976 13.7071 17.7071C14.0976 17.3166 14.0976 16.6834 13.7071 16.2929L12.4142 15H15C16.1046 15 17 14.1046 17 13V5C17.5523 5 18 4.55228 18 4C18 3.44772 17.5523 3 17 3H3ZM14.7071 7.70711C15.0976 7.31658 15.0976 6.68342 14.7071 6.29289C14.3166 5.90237 13.6834 5.90237 13.2929 6.29289L10 9.58579L8.70711 8.29289C8.31658 7.90237 7.68342 7.90237 7.29289 8.29289L5.29289 10.2929C4.90237 10.6834 4.90237 11.3166 5.29289 11.7071C5.68342 12.0976 6.31658 12.0976 6.70711 11.7071L8 10.4142L9.29289 11.7071C9.68342 12.0976 10.3166 12.0976 10.7071 11.7071L14.7071 7.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPrinter;
impl IconShape for HiPrinter {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 4V7H4C2.89543 7 2 7.89543 2 9V12C2 13.1046 2.89543 14 4 14H5V16C5 17.1046 5.89543 18 7 18H13C14.1046 18 15 17.1046 15 16V14H16C17.1046 14 18 13.1046 18 12V9C18 7.89543 17.1046 7 16 7H15V4C15 2.89543 14.1046 2 13 2H7C5.89543 2 5 2.89543 5 4ZM13 4H7V7H13V4ZM13 12H7V16H13V12Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiPuzzle;
impl IconShape for HiPuzzle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M10 3.5C10 2.67157 10.6716 2 11.5 2C12.3284 2 13 2.67157 13 3.5V4C13 4.55228 13.4477 5 14 5H17C17.5523 5 18 5.44772 18 6V9C18 9.55228 17.5523 10 17 10H16.5C15.6716 10 15 10.6716 15 11.5C15 12.3284 15.6716 13 16.5 13H17C17.5523 13 18 13.4477 18 14V17C18 17.5523 17.5523 18 17 18H14C13.4477 18 13 17.5523 13 17V16.5C13 15.6716 12.3284 15 11.5 15C10.6716 15 10 15.6716 10 16.5V17C10 17.5523 9.55228 18 9 18H6C5.44772 18 5 17.5523 5 17V14C5 13.4477 4.55228 13 4 13H3.5C2.67157 13 2 12.3284 2 11.5C2 10.6716 2.67157 10 3.5 10H4C4.55228 10 5 9.55228 5 9V6C5 5.44772 5.44772 5 6 5H9C9.55228 5 10 4.55228 10 4V3.5Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiQrcode;
impl IconShape for HiQrcode {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 4C3 3.44772 3.44772 3 4 3H7C7.55228 3 8 3.44772 8 4V7C8 7.55228 7.55228 8 7 8H4C3.44772 8 3 7.55228 3 7V4ZM5 6V5H6V6H5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M3 13C3 12.4477 3.44772 12 4 12H7C7.55228 12 8 12.4477 8 13V16C8 16.5523 7.55228 17 7 17H4C3.44772 17 3 16.5523 3 16V13ZM5 15V14H6V15H5Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M13 3C12.4477 3 12 3.44772 12 4V7C12 7.55228 12.4477 8 13 8H16C16.5523 8 17 7.55228 17 7V4C17 3.44772 16.5523 3 16 3H13ZM14 5V6H15V5H14Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M11 4C11 3.44772 10.5523 3 10 3C9.44772 3 9 3.44772 9 4V5C9 5.55228 9.44772 6 10 6C10.5523 6 11 5.55228 11 5V4Z",
            }
            path {
                d: "M10 7C10.5523 7 11 7.44772 11 8V9H13C13.5523 9 14 9.44772 14 10C14 10.5523 13.5523 11 13 11H10C9.44772 11 9 10.5523 9 10V8C9 7.44772 9.44772 7 10 7Z",
            }
            path {
                d: "M16 9C15.4477 9 15 9.44772 15 10C15 10.5523 15.4477 11 16 11C16.5523 11 17 10.5523 17 10C17 9.44772 16.5523 9 16 9Z",
            }
            path {
                d: "M9 13C9 12.4477 9.44772 12 10 12H11C11.5523 12 12 12.4477 12 13C12 13.5523 11.5523 14 11 14V16C11 16.5523 10.5523 17 10 17C9.44772 17 9 16.5523 9 16V13Z",
            }
            path {
                d: "M7 11C7.55228 11 8 10.5523 8 10C8 9.44772 7.55228 9 7 9H4C3.44772 9 3 9.44771 3 10C3 10.5523 3.44772 11 4 11H7Z",
            }
            path {
                d: "M17 13C17 13.5523 16.5523 14 16 14H14C13.4477 14 13 13.5523 13 13C13 12.4477 13.4477 12 14 12H16C16.5523 12 17 12.4477 17 13Z",
            }
            path {
                d: "M16 17C16.5523 17 17 16.5523 17 16C17 15.4477 16.5523 15 16 15H13C12.4477 15 12 15.4477 12 16C12 16.5523 12.4477 17 13 17H16Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiQuestionMarkCircle;
impl IconShape for HiQuestionMarkCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM10 7C9.63113 7 9.3076 7.19922 9.13318 7.50073C8.85664 7.97879 8.24491 8.14215 7.76685 7.86561C7.28879 7.58906 7.12543 6.97733 7.40197 6.49927C7.91918 5.60518 8.88833 5 10 5C11.6569 5 13 6.34315 13 8C13 9.30622 12.1652 10.4175 11 10.8293V11C11 11.5523 10.5523 12 10 12C9.44773 12 9.00001 11.5523 9.00001 11V10C9.00001 9.44772 9.44773 9 10 9C10.5523 9 11 8.55228 11 8C11 7.44772 10.5523 7 10 7ZM10 15C10.5523 15 11 14.5523 11 14C11 13.4477 10.5523 13 10 13C9.44772 13 9 13.4477 9 14C9 14.5523 9.44772 15 10 15Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiReceiptRefund;
impl IconShape for HiReceiptRefund {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 2C3.89543 2 3 2.89543 3 4V18L6.5 16L10 18L13.5 16L17 18V4C17 2.89543 16.1046 2 15 2H5ZM9.70711 5.70711C10.0976 5.31658 10.0976 4.68342 9.70711 4.29289C9.31658 3.90237 8.68342 3.90237 8.29289 4.29289L5.29289 7.29289C4.90237 7.68342 4.90237 8.31658 5.29289 8.70711L8.29289 11.7071C8.68342 12.0976 9.31658 12.0976 9.70711 11.7071C10.0976 11.3166 10.0976 10.6834 9.70711 10.2929L8.41421 9H10C11.6569 9 13 10.3431 13 12V13C13 13.5523 13.4477 14 14 14C14.5523 14 15 13.5523 15 13V12C15 9.23858 12.7614 7 10 7H8.41421L9.70711 5.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiReceiptTax;
impl IconShape for HiReceiptTax {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 2C3.89543 2 3 2.89543 3 4V18L6.5 16L10 18L13.5 16L17 18V4C17 2.89543 16.1046 2 15 2H5ZM7.5 5C6.67157 5 6 5.67157 6 6.5C6 7.32843 6.67157 8 7.5 8C8.32843 8 9 7.32843 9 6.5C9 5.67157 8.32843 5 7.5 5ZM13.7071 5.29289C13.3166 4.90237 12.6834 4.90237 12.2929 5.29289L6.29289 11.2929C5.90237 11.6834 5.90237 12.3166 6.29289 12.7071C6.68342 13.0976 7.31658 13.0976 7.70711 12.7071L13.7071 6.70711C14.0976 6.31658 14.0976 5.68342 13.7071 5.29289ZM12.5 10C11.6716 10 11 10.6716 11 11.5C11 12.3284 11.6716 13 12.5 13C13.3284 13 14 12.3284 14 11.5C14 10.6716 13.3284 10 12.5 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiRefresh;
impl IconShape for HiRefresh {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4 2C4.55228 2 5 2.44772 5 3V5.10125C6.27009 3.80489 8.04052 3 10 3C13.0494 3 15.641 4.94932 16.6014 7.66675C16.7855 8.18747 16.5126 8.75879 15.9918 8.94284C15.4711 9.12689 14.8998 8.85396 14.7157 8.33325C14.0289 6.38991 12.1755 5 10 5C8.36507 5 6.91204 5.78502 5.99935 7H9C9.55228 7 10 7.44772 10 8C10 8.55228 9.55228 9 9 9H4C3.44772 9 3 8.55228 3 8V3C3 2.44772 3.44772 2 4 2ZM4.00817 11.0572C4.52888 10.8731 5.1002 11.146 5.28425 11.6668C5.97112 13.6101 7.82453 15 10 15C11.6349 15 13.088 14.215 14.0006 13L11 13C10.4477 13 10 12.5523 10 12C10 11.4477 10.4477 11 11 11H16C16.2652 11 16.5196 11.1054 16.7071 11.2929C16.8946 11.4804 17 11.7348 17 12V17C17 17.5523 16.5523 18 16 18C15.4477 18 15 17.5523 15 17V14.8987C13.7299 16.1951 11.9595 17 10 17C6.95059 17 4.35905 15.0507 3.39857 12.3332C3.21452 11.8125 3.48745 11.2412 4.00817 11.0572Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiReply;
impl IconShape for HiReply {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.70711 3.29289C8.09763 3.68342 8.09763 4.31658 7.70711 4.70711L5.41421 7H11C14.866 7 18 10.134 18 14V16C18 16.5523 17.5523 17 17 17C16.4477 17 16 16.5523 16 16V14C16 11.2386 13.7614 9 11 9H5.41421L7.70711 11.2929C8.09763 11.6834 8.09763 12.3166 7.70711 12.7071C7.31658 13.0976 6.68342 13.0976 6.29289 12.7071L2.29289 8.70711C1.90237 8.31658 1.90237 7.68342 2.29289 7.29289L6.29289 3.29289C6.68342 2.90237 7.31658 2.90237 7.70711 3.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiRewind;
impl IconShape for HiRewind {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8.4453 14.8321C8.75216 15.0366 9.1467 15.0557 9.47186 14.8817C9.79701 14.7077 10 14.3688 10 14L10 11.2019L15.4453 14.8321C15.7522 15.0366 16.1467 15.0557 16.4719 14.8817C16.797 14.7077 17 14.3688 17 14V6C17 5.63121 16.797 5.29235 16.4719 5.11833C16.1467 4.94431 15.7522 4.96338 15.4453 5.16795L10 8.79815V6C10 5.63121 9.79702 5.29235 9.47186 5.11833C9.1467 4.94431 8.75216 4.96338 8.4453 5.16795L2.4453 9.16795C2.1671 9.35342 2 9.66565 2 10C2 10.3344 2.1671 10.6466 2.4453 10.8321L8.4453 14.8321Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiRss;
impl IconShape for HiRss {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 3C4.44772 3 4 3.44772 4 4C4 4.55228 4.44772 5 5 5C10.5228 5 15 9.47715 15 15C15 15.5523 15.4477 16 16 16C16.5523 16 17 15.5523 17 15C17 8.37258 11.6274 3 5 3Z",
            }
            path {
                d: "M4 9C4 8.44772 4.44772 8 5 8C8.86599 8 12 11.134 12 15C12 15.5523 11.5523 16 11 16C10.4477 16 10 15.5523 10 15C10 12.2386 7.76142 10 5 10C4.44772 10 4 9.55228 4 9Z",
            }
            path {
                d: "M3 15C3 13.8954 3.89543 13 5 13C6.10457 13 7 13.8954 7 15C7 16.1046 6.10457 17 5 17C3.89543 17 3 16.1046 3 15Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSaveAs;
impl IconShape for HiSaveAs {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.70711 7.29289C9.31658 6.90237 8.68342 6.90237 8.29289 7.29289C7.90237 7.68342 7.90237 8.31658 8.29289 8.70711L11.2929 11.7071C11.6834 12.0976 12.3166 12.0976 12.7071 11.7071L15.7071 8.70711C16.0976 8.31658 16.0976 7.68342 15.7071 7.29289C15.3166 6.90237 14.6834 6.90237 14.2929 7.29289L13 8.58579L13 5H16C17.1046 5 18 5.89543 18 7V12C18 13.1046 17.1046 14 16 14H8C6.89543 14 6 13.1046 6 12V7C6 5.89543 6.89543 5 8 5H11L11 8.58579L9.70711 7.29289Z",
            }
            path {
                d: "M11 3C11 2.44772 11.4477 2 12 2C12.5523 2 13 2.44772 13 3L13 5H11L11 3Z",
            }
            path {
                d: "M4 9C2.89543 9 2 9.89543 2 11V16C2 17.1046 2.89543 18 4 18H12C13.1046 18 14 17.1046 14 16H4V9Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSave;
impl IconShape for HiSave {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M7.70711 10.2929C7.31658 9.90237 6.68342 9.90237 6.29289 10.2929C5.90237 10.6834 5.90237 11.3166 6.29289 11.7071L9.29289 14.7071C9.68342 15.0976 10.3166 15.0976 10.7071 14.7071L13.7071 11.7071C14.0976 11.3166 14.0976 10.6834 13.7071 10.2929C13.3166 9.90237 12.6834 9.90237 12.2929 10.2929L11 11.5858L11 6H16C17.1046 6 18 6.89543 18 8V15C18 16.1046 17.1046 17 16 17H4C2.89543 17 2 16.1046 2 15V8C2 6.89543 2.89543 6 4 6H9L9 11.5858L7.70711 10.2929Z",
            }
            path {
                d: "M9 4C9 3.44772 9.44772 3 10 3C10.5523 3 11 3.44772 11 4L11 6H9L9 4Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiScale;
impl IconShape for HiScale {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.99998 2C10.5523 2 11 2.44772 11 3V4.32297L14.9544 5.90474L16.5528 5.10557C17.0467 4.85858 17.6474 5.05881 17.8944 5.55279C18.1414 6.04676 17.9412 6.64744 17.4472 6.89443L16.214 7.51101L17.9522 12.9307C18.0727 13.3065 17.961 13.718 17.6669 13.9812C16.9599 14.614 16.0238 15 15 15C13.9761 15 13.0401 14.614 12.3331 13.9812C12.039 13.718 11.9272 13.3065 12.0477 12.9307L13.7631 7.58227L11 6.47703V16H13C13.5523 16 14 16.4477 14 17C14 17.5523 13.5523 18 13 18H6.99997C6.44769 18 5.99997 17.5523 5.99997 17C5.99997 16.4477 6.44769 16 6.99997 16H8.99997V6.47703L6.23689 7.58227L7.9522 12.9307C8.07272 13.3065 7.96096 13.718 7.66689 13.9812C6.95988 14.614 6.02381 15 4.99997 15C3.97614 15 3.04007 14.614 2.33306 13.9812C2.03899 13.718 1.92723 13.3065 2.04775 12.9307L3.78592 7.51101L2.55276 6.89443C2.05878 6.64744 1.85856 6.04676 2.10555 5.55279C2.35254 5.05881 2.95321 4.85858 3.44719 5.10557L5.04553 5.90474L8.99997 4.32297V3C8.99997 2.44772 9.44769 2 9.99998 2ZM4.99997 10.2745L4.18174 12.8258C4.43132 12.9378 4.708 13 4.99997 13C5.29194 13 5.56863 12.9378 5.81821 12.8258L4.99997 10.2745ZM15 10.2745L14.1817 12.8258C14.4313 12.9378 14.708 13 15 13C15.2919 13 15.5686 12.9378 15.8182 12.8258L15 10.2745Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiScissors;
impl IconShape for HiScissors {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.5 2C3.567 2 2 3.567 2 5.5C2 7.433 3.567 9 5.5 9C6.10276 9 6.66993 8.84763 7.1651 8.57931L8.58582 10L7.16515 11.4207C6.66997 11.1524 6.10278 11 5.5 11C3.567 11 2 12.567 2 14.5C2 16.433 3.567 18 5.5 18C7.433 18 9 16.433 9 14.5C9 13.8973 8.84764 13.3301 8.57934 12.835L16.7072 4.70711C17.0977 4.31658 17.0977 3.68342 16.7072 3.29289C16.3167 2.90237 15.6835 2.90237 15.293 3.29289L10 8.58582L8.57931 7.1651C8.84763 6.66993 9 6.10276 9 5.5C9 3.567 7.433 2 5.5 2ZM4 5.5C4 4.67157 4.67157 4 5.5 4C6.32843 4 7 4.67157 7 5.5C7 6.32843 6.32843 7 5.5 7C4.67157 7 4 6.32843 4 5.5ZM4 14.5C4 13.6716 4.67157 13 5.5 13C6.32843 13 7 13.6716 7 14.5C7 15.3284 6.32843 16 5.5 16C4.67157 16 4 15.3284 4 14.5Z",
                fill_rule: "evenodd",
            }
            path {
                d: "M12.8284 11.4142C12.4379 11.0237 11.8047 11.0237 11.4142 11.4142C11.0237 11.8047 11.0237 12.4379 11.4142 12.8284L15.2929 16.7071C15.6834 17.0976 16.3166 17.0976 16.7071 16.7071C17.0976 16.3166 17.0976 15.6834 16.7071 15.2929L12.8284 11.4142Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSearchCircle;
impl IconShape for HiSearchCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 9C9 7.89543 9.89543 7 11 7C12.1046 7 13 7.89543 13 9C13 10.1046 12.1046 11 11 11C10.4474 11 9.94881 10.7772 9.58579 10.4142C9.22276 10.0512 9 9.55256 9 9Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM11 5C8.79086 5 7 6.79086 7 9C7 9.74138 7.20229 10.4364 7.55397 11.0318L5.29289 13.2929C4.90237 13.6834 4.90237 14.3166 5.29289 14.7071C5.68342 15.0976 6.31658 15.0976 6.70711 14.7071L8.96818 12.446C9.56362 12.7977 10.2586 13 11 13C13.2091 13 15 11.2091 15 9C15 6.79086 13.2091 5 11 5Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSearch;
impl IconShape for HiSearch {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 4C5.79086 4 4 5.79086 4 8C4 10.2091 5.79086 12 8 12C10.2091 12 12 10.2091 12 8C12 5.79086 10.2091 4 8 4ZM2 8C2 4.68629 4.68629 2 8 2C11.3137 2 14 4.68629 14 8C14 9.29583 13.5892 10.4957 12.8907 11.4765L17.7071 16.2929C18.0976 16.6834 18.0976 17.3166 17.7071 17.7071C17.3166 18.0976 16.6834 18.0976 16.2929 17.7071L11.4765 12.8907C10.4957 13.5892 9.29583 14 8 14C4.68629 14 2 11.3137 2 8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSelector;
impl IconShape for HiSelector {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 3C10.2652 3 10.5196 3.10536 10.7071 3.29289L13.7071 6.29289C14.0976 6.68342 14.0976 7.31658 13.7071 7.70711C13.3166 8.09763 12.6834 8.09763 12.2929 7.70711L10 5.41421L7.70711 7.70711C7.31658 8.09763 6.68342 8.09763 6.29289 7.70711C5.90237 7.31658 5.90237 6.68342 6.29289 6.29289L9.29289 3.29289C9.48043 3.10536 9.73478 3 10 3ZM6.29289 12.2929C6.68342 11.9024 7.31658 11.9024 7.70711 12.2929L10 14.5858L12.2929 12.2929C12.6834 11.9024 13.3166 11.9024 13.7071 12.2929C14.0976 12.6834 14.0976 13.3166 13.7071 13.7071L10.7071 16.7071C10.3166 17.0976 9.68342 17.0976 9.29289 16.7071L6.29289 13.7071C5.90237 13.3166 5.90237 12.6834 6.29289 12.2929Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiServer;
impl IconShape for HiServer {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 5C2 3.89543 2.89543 3 4 3H16C17.1046 3 18 3.89543 18 5V7C18 8.10457 17.1046 9 16 9H4C2.89543 9 2 8.10457 2 7V5ZM16 6C16 6.55228 15.5523 7 15 7C14.4477 7 14 6.55228 14 6C14 5.44772 14.4477 5 15 5C15.5523 5 16 5.44772 16 6Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M2 13C2 11.8954 2.89543 11 4 11H16C17.1046 11 18 11.8954 18 13V15C18 16.1046 17.1046 17 16 17H4C2.89543 17 2 16.1046 2 15V13ZM16 14C16 14.5523 15.5523 15 15 15C14.4477 15 14 14.5523 14 14C14 13.4477 14.4477 13 15 13C15.5523 13 16 13.4477 16 14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiShare;
impl IconShape for HiShare {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M15 8C16.6569 8 18 6.65685 18 5C18 3.34315 16.6569 2 15 2C13.3431 2 12 3.34315 12 5C12 5.12548 12.0077 5.24917 12.0227 5.37061L7.08259 7.84064C6.54303 7.32015 5.8089 7 5 7C3.34315 7 2 8.34315 2 10C2 11.6569 3.34315 13 5 13C5.80892 13 6.54306 12.6798 7.08263 12.1593L12.0227 14.6293C12.0077 14.7508 12 14.8745 12 15C12 16.6569 13.3431 18 15 18C16.6569 18 18 16.6569 18 15C18 13.3431 16.6569 12 15 12C14.1911 12 13.457 12.3201 12.9174 12.8406L7.97733 10.3706C7.9923 10.2492 8 10.1255 8 10C8 9.8745 7.99229 9.7508 7.97733 9.62934L12.9174 7.15932C13.4569 7.67984 14.1911 8 15 8Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiShieldCheck;
impl IconShape for HiShieldCheck {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2.16611 4.99891C5.17437 4.95809 7.91528 3.81033 10 1.94446C12.0847 3.81033 14.8256 4.95809 17.8339 4.99891C17.9431 5.64968 18 6.31821 18 7.00003C18 12.2249 14.6608 16.6698 10 18.3172C5.33923 16.6698 2 12.2249 2 7.00003C2 6.31821 2.05686 5.64968 2.16611 4.99891ZM13.7071 8.70711C14.0976 8.31658 14.0976 7.68342 13.7071 7.29289C13.3166 6.90237 12.6834 6.90237 12.2929 7.29289L9 10.5858L7.70711 9.29289C7.31658 8.90237 6.68342 8.90237 6.29289 9.29289C5.90237 9.68342 5.90237 10.3166 6.29289 10.7071L8.29289 12.7071C8.68342 13.0976 9.31658 13.0976 9.70711 12.7071L13.7071 8.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiShieldExclamation;
impl IconShape for HiShieldExclamation {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 1.94446C7.91528 3.81033 5.17437 4.95809 2.16611 4.99891C2.05686 5.64968 2 6.31821 2 7.00003C2 12.2249 5.33923 16.6698 10 18.3172C14.6608 16.6698 18 12.2249 18 7.00003C18 6.31821 17.9431 5.64968 17.8339 4.99891C14.8256 4.95809 12.0847 3.81033 10 1.94446ZM11 14C11 14.5523 10.5523 15 10 15C9.44771 15 9 14.5523 9 14C9 13.4477 9.44771 13 10 13C10.5523 13 11 13.4477 11 14ZM11 7C11 6.44772 10.5523 6 10 6C9.44771 6 9 6.44772 9 7V10C9 10.5523 9.44771 11 10 11C10.5523 11 11 10.5523 11 10V7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiShoppingBag;
impl IconShape for HiShoppingBag {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 2C7.79086 2 6 3.79086 6 6V7H5C4.49046 7 4.06239 7.38314 4.00612 7.88957L3.00612 16.8896C2.97471 17.1723 3.06518 17.455 3.25488 17.6669C3.44458 17.8789 3.71556 18 4 18H16C16.2844 18 16.5554 17.8789 16.7451 17.6669C16.9348 17.455 17.0253 17.1723 16.9939 16.8896L15.9939 7.88957C15.9376 7.38314 15.5096 7 15 7H14V6C14 3.79086 12.2091 2 10 2ZM12 7V6C12 4.89543 11.1046 4 10 4C8.89543 4 8 4.89543 8 6V7H12ZM6 10C6 9.44772 6.44772 9 7 9C7.55228 9 8 9.44772 8 10C8 10.5523 7.55228 11 7 11C6.44772 11 6 10.5523 6 10ZM13 9C12.4477 9 12 9.44772 12 10C12 10.5523 12.4477 11 13 11C13.5523 11 14 10.5523 14 10C14 9.44772 13.5523 9 13 9Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiShoppingCart;
impl IconShape for HiShoppingCart {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 1C2.44772 1 2 1.44772 2 2C2 2.55228 2.44772 3 3 3H4.21922L4.52478 4.22224C4.52799 4.23637 4.5315 4.25039 4.5353 4.26429L5.89253 9.69321L4.99995 10.5858C3.74002 11.8457 4.63235 14 6.41416 14H15C15.5522 14 16 13.5523 16 13C16 12.4477 15.5522 12 15 12L6.41417 12L7.41416 11H14C14.3788 11 14.725 10.786 14.8944 10.4472L17.8944 4.44721C18.0494 4.13723 18.0329 3.76909 17.8507 3.47427C17.6684 3.17945 17.3466 3 17 3H6.28078L5.97014 1.75746C5.85885 1.3123 5.45887 1 5 1H3Z",
            }
            path {
                d: "M16 16.5C16 17.3284 15.3284 18 14.5 18C13.6716 18 13 17.3284 13 16.5C13 15.6716 13.6716 15 14.5 15C15.3284 15 16 15.6716 16 16.5Z",
            }
            path {
                d: "M6.5 18C7.32843 18 8 17.3284 8 16.5C8 15.6716 7.32843 15 6.5 15C5.67157 15 5 15.6716 5 16.5C5 17.3284 5.67157 18 6.5 18Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSortAscending;
impl IconShape for HiSortAscending {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 3C2.44772 3 2 3.44772 2 4C2 4.55228 2.44772 5 3 5H14C14.5523 5 15 4.55228 15 4C15 3.44772 14.5523 3 14 3H3Z",
            }
            path {
                d: "M3 7C2.44772 7 2 7.44772 2 8C2 8.55228 2.44772 9 3 9H8C8.55228 9 9 8.55228 9 8C9 7.44772 8.55228 7 8 7H3Z",
            }
            path {
                d: "M3 11C2.44772 11 2 11.4477 2 12C2 12.5523 2.44772 13 3 13H7C7.55228 13 8 12.5523 8 12C8 11.4477 7.55228 11 7 11H3Z",
            }
            path {
                d: "M13 16C13 16.5523 13.4477 17 14 17C14.5523 17 15 16.5523 15 16L15 10.4142L16.2929 11.7071C16.6834 12.0976 17.3166 12.0976 17.7071 11.7071C18.0976 11.3166 18.0976 10.6834 17.7071 10.2929L14.7071 7.29289C14.5196 7.10536 14.2652 7 14 7C13.7348 7 13.4804 7.10536 13.2929 7.29289L10.2929 10.2929C9.90237 10.6834 9.90237 11.3166 10.2929 11.7071C10.6834 12.0976 11.3166 12.0976 11.7071 11.7071L13 10.4142L13 16Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSortDescending;
impl IconShape for HiSortDescending {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 3C2.44772 3 2 3.44772 2 4C2 4.55228 2.44772 5 3 5H14C14.5523 5 15 4.55228 15 4C15 3.44772 14.5523 3 14 3H3Z",
            }
            path {
                d: "M3 7C2.44772 7 2 7.44772 2 8C2 8.55228 2.44772 9 3 9H10C10.5523 9 11 8.55228 11 8C11 7.44772 10.5523 7 10 7H3Z",
            }
            path {
                d: "M3 11C2.44772 11 2 11.4477 2 12C2 12.5523 2.44772 13 3 13H7C7.55228 13 8 12.5523 8 12C8 11.4477 7.55228 11 7 11H3Z",
            }
            path {
                d: "M15 8C15 7.44772 14.5523 7 14 7C13.4477 7 13 7.44771 13 8L13 13.5858L11.7071 12.2929C11.3166 11.9024 10.6834 11.9024 10.2929 12.2929C9.90237 12.6834 9.90237 13.3166 10.2929 13.7071L13.2929 16.7071C13.4804 16.8946 13.7348 17 14 17C14.2652 17 14.5196 16.8946 14.7071 16.7071L17.7071 13.7071C18.0976 13.3166 18.0976 12.6834 17.7071 12.2929C17.3166 11.9024 16.6834 11.9024 16.2929 12.2929L15 13.5858L15 8Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSparkles;
impl IconShape for HiSparkles {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 2C5.55228 2 6 2.44772 6 3V4H7C7.55228 4 8 4.44772 8 5C8 5.55228 7.55228 6 7 6H6V7C6 7.55228 5.55228 8 5 8C4.44772 8 4 7.55228 4 7V6H3C2.44772 6 2 5.55228 2 5C2 4.44772 2.44772 4 3 4H4V3C4 2.44772 4.44772 2 5 2ZM5 12C5.55228 12 6 12.4477 6 13V14H7C7.55228 14 8 14.4477 8 15C8 15.5523 7.55228 16 7 16H6V17C6 17.5523 5.55228 18 5 18C4.44772 18 4 17.5523 4 17V16H3C2.44772 16 2 15.5523 2 15C2 14.4477 2.44772 14 3 14H4V13C4 12.4477 4.44772 12 5 12Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M11.9999 2C12.4537 2 12.8505 2.30548 12.9667 2.74411L14.1459 7.19893L17.4997 9.13381C17.8092 9.31241 17.9999 9.64262 17.9999 10C17.9999 10.3574 17.8092 10.6876 17.4997 10.8662L14.1459 12.8011L12.9667 17.2559C12.8505 17.6945 12.4537 18 11.9999 18C11.5462 18 11.1493 17.6945 11.0332 17.2559L9.85402 12.8011L6.50027 10.8662C6.19072 10.6876 6 10.3574 6 10C6 9.64262 6.19072 9.31241 6.50027 9.13382L9.85402 7.19893L11.0332 2.74411C11.1493 2.30548 11.5462 2 11.9999 2Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSpeakerphone;
impl IconShape for HiSpeakerphone {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 3C18 2.65342 17.8205 2.33156 17.5257 2.14935C17.2309 1.96714 16.8628 1.95058 16.5528 2.10557L8.76393 6H5C3.34315 6 2 7.34315 2 9C2 10.6569 3.34315 12 5 12H5.27925L7.05132 17.3162C7.18744 17.7246 7.56958 18 8.00001 18H9.00001C9.55229 18 10 17.5523 10 17V12.618L16.5528 15.8944C16.8628 16.0494 17.2309 16.0329 17.5257 15.8507C17.8205 15.6684 18 15.3466 18 15V3Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiStar;
impl IconShape for HiStar {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9.04893 2.92707C9.34828 2.00576 10.6517 2.00576 10.951 2.92707L12.0206 6.21886C12.1545 6.63089 12.5384 6.90985 12.9717 6.90985H16.4329C17.4016 6.90985 17.8044 8.14946 17.0207 8.71886L14.2205 10.7533C13.87 11.0079 13.7233 11.4593 13.8572 11.8713L14.9268 15.1631C15.2261 16.0844 14.1717 16.8506 13.3879 16.2812L10.5878 14.2467C10.2373 13.9921 9.76269 13.9921 9.4122 14.2467L6.61203 16.2812C5.82832 16.8506 4.77384 16.0844 5.07319 15.1631L6.14276 11.8713C6.27663 11.4593 6.12997 11.0079 5.77949 10.7533L2.97932 8.71886C2.1956 8.14946 2.59838 6.90985 3.5671 6.90985H7.0283C7.46153 6.90985 7.84548 6.63089 7.97936 6.21886L9.04893 2.92707Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiStatusOffline;
impl IconShape for HiStatusOffline {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3.70711 2.29289C3.31658 1.90237 2.68342 1.90237 2.29289 2.29289C1.90237 2.68342 1.90237 3.31658 2.29289 3.70711L9.21426 10.6285C9.26325 10.6906 9.31947 10.7469 9.38164 10.7958L16.2929 17.7071C16.6834 18.0976 17.3166 18.0976 17.7071 17.7071C18.0976 17.3166 18.0976 16.6834 17.7071 16.2929L17.0323 15.6181C19.8626 12.0844 19.6398 6.91177 16.3641 3.63603C15.9736 3.24551 15.3404 3.24551 14.9499 3.63603C14.5593 4.02656 14.5593 4.65972 14.9499 5.05025C17.4435 7.54386 17.6625 11.4508 15.6068 14.1926L14.172 12.7578C15.4582 10.8164 15.2461 8.17494 13.5357 6.46446C13.1451 6.07394 12.512 6.07394 12.1214 6.46446C11.7309 6.85498 11.7309 7.48815 12.1214 7.87867C13.0451 8.80233 13.2406 10.1784 12.7078 11.2936L10.7164 9.30219C10.7103 9.29595 10.7042 9.28979 10.6979 9.2837L3.70711 2.29289Z",
            }
            path {
                d: "M3.23766 8.1865C3.38012 7.65291 3.06305 7.10485 2.52946 6.96239C1.99586 6.81992 1.44781 7.137 1.30535 7.67059C0.5045 10.6701 1.27982 14.0074 3.63615 16.3637C4.02667 16.7542 4.65984 16.7542 5.05036 16.3637C5.44089 15.9732 5.44089 15.34 5.05036 14.9495C3.21924 13.1184 2.6134 10.5246 3.23766 8.1865Z",
            }
            path {
                d: "M7.40075 11.4995C7.12434 11.0214 6.51266 10.8578 6.03452 11.1343C5.55639 11.4107 5.39285 12.0223 5.66926 12.5005C5.88367 12.8714 6.14907 13.2198 6.46458 13.5353C6.85511 13.9258 7.48827 13.9258 7.8788 13.5353C8.26932 13.1448 8.26932 12.5116 7.8788 12.1211C7.68771 11.93 7.52865 11.7208 7.40075 11.4995Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiStatusOnline;
impl IconShape for HiStatusOnline {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5.05025 3.63579C5.44078 4.02631 5.44078 4.65948 5.05025 5.05C2.31658 7.78367 2.31658 12.2158 5.05025 14.9495C5.44078 15.34 5.44078 15.9732 5.05025 16.3637C4.65973 16.7542 4.02656 16.7542 3.63604 16.3637C0.12132 12.849 0.12132 7.15051 3.63604 3.63579C4.02656 3.24526 4.65973 3.24526 5.05025 3.63579ZM14.9498 3.63602C15.3403 3.2455 15.9735 3.2455 16.364 3.63602C19.8787 7.15074 19.8787 12.8492 16.364 16.3639C15.9735 16.7545 15.3403 16.7545 14.9498 16.3639C14.5592 15.9734 14.5592 15.3403 14.9498 14.9497C17.6834 12.2161 17.6834 7.78391 14.9498 5.05023C14.5592 4.65971 14.5592 4.02655 14.9498 3.63602ZM7.87869 6.46422C8.26921 6.85474 8.26921 7.48791 7.87869 7.87843C6.70711 9.05 6.70711 10.9495 7.87869 12.1211C8.26921 12.5116 8.26921 13.1448 7.87868 13.5353C7.48816 13.9258 6.855 13.9258 6.46447 13.5353C4.51185 11.5827 4.51185 8.41684 6.46447 6.46422C6.855 6.07369 7.48816 6.07369 7.87869 6.46422ZM12.1213 6.46445C12.5119 6.07392 13.145 6.07392 13.5355 6.46445C15.4882 8.41707 15.4882 11.5829 13.5355 13.5355C13.145 13.926 12.5119 13.926 12.1213 13.5355C11.7308 13.145 11.7308 12.5118 12.1213 12.1213C13.2929 10.9497 13.2929 9.05023 12.1213 7.87866C11.7308 7.48814 11.7308 6.85497 12.1213 6.46445ZM10 8.99998C10.5523 8.99998 11 9.4477 11 9.99998V10.01C11 10.5623 10.5523 11.01 10 11.01C9.44772 11.01 9 10.5623 9 10.01V9.99998C9 9.4477 9.44772 8.99998 10 8.99998Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiStop;
impl IconShape for HiStop {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM8 7C7.44772 7 7 7.44772 7 8V12C7 12.5523 7.44772 13 8 13H12C12.5523 13 13 12.5523 13 12V8C13 7.44772 12.5523 7 12 7H8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSun;
impl IconShape for HiSun {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 2C10.5523 2 11 2.44772 11 3V4C11 4.55228 10.5523 5 10 5C9.44772 5 9 4.55228 9 4V3C9 2.44772 9.44772 2 10 2ZM14 10C14 12.2091 12.2091 14 10 14C7.79086 14 6 12.2091 6 10C6 7.79086 7.79086 6 10 6C12.2091 6 14 7.79086 14 10ZM13.5356 14.9497L14.2427 15.6568C14.6332 16.0473 15.2664 16.0473 15.6569 15.6568C16.0474 15.2663 16.0474 14.6331 15.6569 14.2426L14.9498 13.5355C14.5593 13.145 13.9261 13.145 13.5356 13.5355C13.1451 13.926 13.1451 14.5592 13.5356 14.9497ZM15.6568 4.34309C16.0473 4.73362 16.0473 5.36678 15.6568 5.75731L14.9497 6.46441C14.5592 6.85494 13.926 6.85494 13.5355 6.46441C13.145 6.07389 13.145 5.44072 13.5355 5.0502L14.2426 4.34309C14.6331 3.95257 15.2663 3.95257 15.6568 4.34309ZM17 11C17.5523 11 18 10.5523 18 10C18 9.44772 17.5523 9 17 9H16C15.4477 9 15 9.44772 15 10C15 10.5523 15.4477 11 16 11H17ZM10 15C10.5523 15 11 15.4477 11 16V17C11 17.5523 10.5523 18 10 18C9.44772 18 9 17.5523 9 17V16C9 15.4477 9.44772 15 10 15ZM5.05031 6.46443C5.44083 6.85496 6.074 6.85496 6.46452 6.46443C6.85505 6.07391 6.85505 5.44074 6.46452 5.05022L5.75742 4.34311C5.36689 3.95259 4.73373 3.95259 4.3432 4.34311C3.95268 4.73363 3.95268 5.3668 4.3432 5.75732L5.05031 6.46443ZM6.46443 14.9497L5.75732 15.6568C5.3668 16.0473 4.73363 16.0473 4.34311 15.6568C3.95259 15.2663 3.95259 14.6331 4.34311 14.2426L5.05022 13.5355C5.44074 13.145 6.07391 13.145 6.46443 13.5355C6.85496 13.926 6.85496 14.5592 6.46443 14.9497ZM4 11C4.55228 11 5 10.5523 5 10C5 9.44772 4.55228 9 4 9H3C2.44772 9 2 9.44772 2 10C2 10.5523 2.44772 11 3 11H4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSupport;
impl IconShape for HiSupport {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM16 10C16 10.9926 15.7589 11.929 15.3322 12.7537L13.8076 11.2291C13.9325 10.8419 14 10.4288 14 10C14 9.6714 13.9604 9.35205 13.8856 9.04648L15.4484 7.48368C15.8025 8.24895 16 9.1014 16 10ZM10.8345 13.9128L12.4156 15.4939C11.6765 15.8193 10.8594 16 10 16C9.1014 16 8.24895 15.8025 7.48368 15.4484L9.04648 13.8856C9.35205 13.9604 9.6714 14 10 14C10.2862 14 10.5653 13.9699 10.8345 13.9128ZM6.1581 11.1172C6.05517 10.7626 6 10.3878 6 10C6 9.66814 6.04041 9.34571 6.11659 9.03738L6.0378 9.11617L4.50608 7.58444C4.18066 8.32349 4 9.14065 4 10C4 10.9539 4.2226 11.8558 4.61868 12.6566L6.1581 11.1172ZM7.24631 4.66782C8.07101 4.24105 9.00735 4 10 4C10.9539 4 11.8558 4.2226 12.6566 4.61868L11.1172 6.1581C10.7626 6.05517 10.3878 6 10 6C9.57119 6 9.15814 6.06748 8.77088 6.19239L7.24631 4.66782ZM12 10C12 11.1046 11.1046 12 10 12C8.89543 12 8 11.1046 8 10C8 8.89543 8.89543 8 10 8C11.1046 8 12 8.89543 12 10Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSwitchHorizontal;
impl IconShape for HiSwitchHorizontal {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 5C7.44772 5 7 5.44771 7 6C7 6.55228 7.44772 7 8 7L13.5858 7L12.2929 8.29289C11.9024 8.68342 11.9024 9.31658 12.2929 9.70711C12.6834 10.0976 13.3166 10.0976 13.7071 9.70711L16.7071 6.70711C16.8946 6.51957 17 6.26522 17 6C17 5.73478 16.8946 5.48043 16.7071 5.29289L13.7071 2.29289C13.3166 1.90237 12.6834 1.90237 12.2929 2.29289C11.9024 2.68342 11.9024 3.31658 12.2929 3.70711L13.5858 5L8 5Z",
            }
            path {
                d: "M12 15C12.5523 15 13 14.5523 13 14C13 13.4477 12.5523 13 12 13L6.41421 13L7.70711 11.7071C8.09763 11.3166 8.09763 10.6834 7.70711 10.2929C7.31658 9.90237 6.68342 9.90237 6.29289 10.2929L3.29289 13.2929C3.10536 13.4804 3 13.7348 3 14C3 14.2652 3.10536 14.5196 3.29289 14.7071L6.29289 17.7071C6.68342 18.0976 7.31658 18.0976 7.70711 17.7071C8.09763 17.3166 8.09763 16.6834 7.70711 16.2929L6.41421 15L12 15Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiSwitchVertical;
impl IconShape for HiSwitchVertical {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 12C5 12.5523 5.44771 13 6 13C6.55228 13 7 12.5523 7 12L7 6.41421L8.29289 7.70711C8.68342 8.09763 9.31658 8.09763 9.70711 7.70711C10.0976 7.31658 10.0976 6.68342 9.70711 6.29289L6.70711 3.29289C6.51957 3.10536 6.26522 3 6 3C5.73478 3 5.48043 3.10536 5.29289 3.29289L2.29289 6.29289C1.90237 6.68342 1.90237 7.31658 2.29289 7.70711C2.68342 8.09763 3.31658 8.09763 3.70711 7.70711L5 6.41421L5 12Z",
            }
            path {
                d: "M15 8C15 7.44772 14.5523 7 14 7C13.4477 7 13 7.44772 13 8L13 13.5858L11.7071 12.2929C11.3166 11.9024 10.6834 11.9024 10.2929 12.2929C9.90237 12.6834 9.90237 13.3166 10.2929 13.7071L13.2929 16.7071C13.4804 16.8946 13.7348 17 14 17C14.2652 17 14.5196 16.8946 14.7071 16.7071L17.7071 13.7071C18.0976 13.3166 18.0976 12.6834 17.7071 12.2929C17.3166 11.9024 16.6834 11.9024 16.2929 12.2929L15 13.5858L15 8Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTable;
impl IconShape for HiTable {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M5 4C3.34315 4 2 5.34315 2 7V13C2 14.6569 3.34315 16 5 16H15C16.6569 16 18 14.6569 18 13V7C18 5.34315 16.6569 4 15 4H5ZM4 13V12H9V14H5C4.44772 14 4 13.5523 4 13ZM11 14H15C15.5523 14 16 13.5523 16 13V12H11V14ZM11 10H16V8H11V10ZM9 8H4V10H9V8Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTag;
impl IconShape for HiTag {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M17.7071 9.29289C18.0976 9.68342 18.0976 10.3166 17.7071 10.7071L10.7071 17.7071C10.3166 18.0976 9.68342 18.0976 9.29289 17.7071L2.29289 10.7071C2.0976 10.5118 1.99997 10.2558 2 9.99988V5C2 3.34315 3.34315 2 5 2H10.0003C10.2561 2.00007 10.5119 2.0977 10.7071 2.29289L17.7071 9.29289ZM5 6C5.55228 6 6 5.55228 6 5C6 4.44772 5.55228 4 5 4C4.44772 4 4 4.44772 4 5C4 5.55228 4.44772 6 5 6Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTemplate;
impl IconShape for HiTemplate {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M3 4C3 3.44772 3.44772 3 4 3H16C16.5523 3 17 3.44772 17 4V6C17 6.55228 16.5523 7 16 7H4C3.44772 7 3 6.55228 3 6V4Z",
            }
            path {
                d: "M3 10C3 9.44771 3.44772 9 4 9H10C10.5523 9 11 9.44771 11 10V16C11 16.5523 10.5523 17 10 17H4C3.44772 17 3 16.5523 3 16V10Z",
            }
            path {
                d: "M14 9C13.4477 9 13 9.44771 13 10V16C13 16.5523 13.4477 17 14 17H16C16.5523 17 17 16.5523 17 16V10C17 9.44771 16.5523 9 16 9H14Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTerminal;
impl IconShape for HiTerminal {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M2 5C2 3.89543 2.89543 3 4 3H16C17.1046 3 18 3.89543 18 5V15C18 16.1046 17.1046 17 16 17H4C2.89543 17 2 16.1046 2 15V5ZM5.29289 6.29289C5.68342 5.90237 6.31658 5.90237 6.70711 6.29289L9.70711 9.29289C10.0976 9.68342 10.0976 10.3166 9.70711 10.7071L6.70711 13.7071C6.31658 14.0976 5.68342 14.0976 5.29289 13.7071C4.90237 13.3166 4.90237 12.6834 5.29289 12.2929L7.58579 10L5.29289 7.70711C4.90237 7.31658 4.90237 6.68342 5.29289 6.29289ZM11 12C10.4477 12 10 12.4477 10 13C10 13.5523 10.4477 14 11 14H14C14.5523 14 15 13.5523 15 13C15 12.4477 14.5523 12 14 12H11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiThumbDown;
impl IconShape for HiThumbDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M18 9.5C18 10.3284 17.3285 11 16.5 11C15.6716 11 15 10.3284 15 9.5V3.5C15 2.67157 15.6716 2 16.5 2C17.3285 2 18 2.67157 18 3.5V9.5Z",
            }
            path {
                d: "M14 9.66667V4.23607C14 3.47852 13.572 2.786 12.8945 2.44721L12.8446 2.42229C12.2892 2.14458 11.6767 2 11.0558 2L5.63964 2C4.68628 2 3.86545 2.67292 3.67848 3.60777L2.47848 9.60777C2.23097 10.8453 3.17755 12 4.43964 12H8.00004V16C8.00004 17.1046 8.89547 18 10 18C10.5523 18 11 17.5523 11 17V16.3333C11 15.4679 11.2807 14.6257 11.8 13.9333L13.2 12.0667C13.7193 11.3743 14 10.5321 14 9.66667Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiThumbUp;
impl IconShape for HiThumbUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 10.5C2 9.67157 2.67157 9 3.5 9C4.32843 9 5 9.67157 5 10.5V16.5C5 17.3284 4.32843 18 3.5 18C2.67157 18 2 17.3284 2 16.5V10.5Z",
            }
            path {
                d: "M6 10.3333V15.7639C6 16.5215 6.428 17.214 7.10557 17.5528L7.15542 17.5777C7.71084 17.8554 8.32329 18 8.94427 18H14.3604C15.3138 18 16.1346 17.3271 16.3216 16.3922L17.5216 10.3922C17.7691 9.15465 16.8225 8 15.5604 8H12V4C12 2.89543 11.1046 2 10 2C9.44772 2 9 2.44772 9 3V3.66667C9 4.53215 8.71929 5.37428 8.2 6.06667L6.8 7.93333C6.28071 8.62572 6 9.46785 6 10.3333Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTicket;
impl IconShape for HiTicket {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 6C2 4.89543 2.89543 4 4 4H16C17.1046 4 18 4.89543 18 6V8C16.8954 8 16 8.89543 16 10C16 11.1046 16.8954 12 18 12V14C18 15.1046 17.1046 16 16 16H4C2.89543 16 2 15.1046 2 14V12C3.10457 12 4 11.1046 4 10C4 8.89543 3.10457 8 2 8V6Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTranslate;
impl IconShape for HiTranslate {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M7.00001 2C7.55229 2 8.00001 2.44772 8.00001 3V4H8.73223C8.744 3.99979 8.75581 3.99979 8.76765 4H11C11.5523 4 12 4.44772 12 5C12 5.55228 11.5523 6 11 6H9.57801C9.21635 7.68748 8.63076 9.29154 7.85405 10.7796C8.14482 11.1338 8.44964 11.476 8.76767 11.8055C9.15124 12.2028 9.14007 12.8359 8.74272 13.2195C8.34537 13.603 7.7123 13.5919 7.32873 13.1945C7.13962 12.9986 6.95468 12.7987 6.77405 12.5948C5.88895 13.9101 4.84387 15.1084 3.66692 16.1618C3.2554 16.5301 2.6232 16.4951 2.25487 16.0836C1.88655 15.672 1.92157 15.0398 2.3331 14.6715C3.54619 13.5858 4.60214 12.3288 5.4631 10.9389C4.90663 10.1499 4.40868 9.31652 3.97558 8.44503C3.7298 7.95045 3.93148 7.35027 4.42606 7.10449C4.92064 6.8587 5.52083 7.06039 5.76661 7.55497C6.00021 8.02502 6.25495 8.48278 6.52961 8.92699C6.947 7.99272 7.28247 7.01402 7.52698 6H3.00001C2.44772 6 2.00001 5.55228 2.00001 5C2.00001 4.44772 2.44772 4 3.00001 4H6.00001V3C6.00001 2.44772 6.44772 2 7.00001 2ZM13 8C13.3788 8 13.725 8.214 13.8944 8.55279L16.8854 14.5348C16.8919 14.5471 16.8982 14.5596 16.9041 14.5722L17.8944 16.5528C18.1414 17.0468 17.9412 17.6474 17.4472 17.8944C16.9532 18.1414 16.3526 17.9412 16.1056 17.4472L15.382 16H10.618L9.89444 17.4472C9.64745 17.9412 9.04677 18.1414 8.5528 17.8944C8.05882 17.6474 7.85859 17.0468 8.10558 16.5528L9.09589 14.5722C9.10187 14.5596 9.1081 14.5471 9.11458 14.5348L12.1056 8.55279C12.275 8.214 12.6212 8 13 8ZM11.618 14H14.382L13 11.2361L11.618 14Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTrash;
impl IconShape for HiTrash {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9 2C8.62123 2 8.27497 2.214 8.10557 2.55279L7.38197 4H4C3.44772 4 3 4.44772 3 5C3 5.55228 3.44772 6 4 6L4 16C4 17.1046 4.89543 18 6 18H14C15.1046 18 16 17.1046 16 16V6C16.5523 6 17 5.55228 17 5C17 4.44772 16.5523 4 16 4H12.618L11.8944 2.55279C11.725 2.214 11.3788 2 11 2H9ZM7 8C7 7.44772 7.44772 7 8 7C8.55228 7 9 7.44772 9 8V14C9 14.5523 8.55228 15 8 15C7.44772 15 7 14.5523 7 14V8ZM12 7C11.4477 7 11 7.44772 11 8V14C11 14.5523 11.4477 15 12 15C12.5523 15 13 14.5523 13 14V8C13 7.44772 12.5523 7 12 7Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTrendingDown;
impl IconShape for HiTrendingDown {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12 13C11.4477 13 11 13.4477 11 14C11 14.5523 11.4477 15 12 15H17C17.5523 15 18 14.5523 18 14V9C18 8.44772 17.5523 8 17 8C16.4477 8 16 8.44772 16 9V11.5858L11.7071 7.29289C11.3166 6.90237 10.6834 6.90237 10.2929 7.29289L8 9.58579L3.70711 5.29289C3.31658 4.90237 2.68342 4.90237 2.29289 5.29289C1.90237 5.68342 1.90237 6.31658 2.29289 6.70711L7.29289 11.7071C7.68342 12.0976 8.31658 12.0976 8.70711 11.7071L11 9.41421L14.5858 13H12Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTrendingUp;
impl IconShape for HiTrendingUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M12 7C11.4477 7 11 6.55228 11 6C11 5.44772 11.4477 5 12 5H17C17.5523 5 18 5.44772 18 6V11C18 11.5523 17.5523 12 17 12C16.4477 12 16 11.5523 16 11V8.41421L11.7071 12.7071C11.3166 13.0976 10.6834 13.0976 10.2929 12.7071L8 10.4142L3.70711 14.7071C3.31658 15.0976 2.68342 15.0976 2.29289 14.7071C1.90237 14.3166 1.90237 13.6834 2.29289 13.2929L7.29289 8.29289C7.68342 7.90237 8.31658 7.90237 8.70711 8.29289L11 10.5858L14.5858 7H12Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiTruck;
impl IconShape for HiTruck {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 16.5C8 17.3284 7.32843 18 6.5 18C5.67157 18 5 17.3284 5 16.5C5 15.6716 5.67157 15 6.5 15C7.32843 15 8 15.6716 8 16.5Z",
            }
            path {
                d: "M15 16.5C15 17.3284 14.3284 18 13.5 18C12.6716 18 12 17.3284 12 16.5C12 15.6716 12.6716 15 13.5 15C14.3284 15 15 15.6716 15 16.5Z",
            }
            path {
                d: "M3 4C2.44772 4 2 4.44772 2 5V15C2 15.5523 2.44772 16 3 16H4.05001C4.28164 14.8589 5.29052 14 6.5 14C7.70948 14 8.71836 14.8589 8.94999 16H10C10.5523 16 11 15.5523 11 15V5C11 4.44772 10.5523 4 10 4H3Z",
            }
            path {
                d: "M14 7C13.4477 7 13 7.44772 13 8V14.05C13.1616 14.0172 13.3288 14 13.5 14C14.7095 14 15.7184 14.8589 15.95 16H17C17.5523 16 18 15.5523 18 15V10C18 9.73478 17.8946 9.48043 17.7071 9.29289L15.7071 7.29289C15.5196 7.10536 15.2652 7 15 7H14Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUpload;
impl IconShape for HiUpload {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 17C3 16.4477 3.44772 16 4 16H16C16.5523 16 17 16.4477 17 17C17 17.5523 16.5523 18 16 18H4C3.44772 18 3 17.5523 3 17ZM6.29289 6.70711C5.90237 6.31658 5.90237 5.68342 6.29289 5.29289L9.29289 2.29289C9.48043 2.10536 9.73478 2 10 2C10.2652 2 10.5196 2.10536 10.7071 2.29289L13.7071 5.29289C14.0976 5.68342 14.0976 6.31658 13.7071 6.70711C13.3166 7.09763 12.6834 7.09763 12.2929 6.70711L11 5.41421L11 13C11 13.5523 10.5523 14 10 14C9.44771 14 9 13.5523 9 13L9 5.41421L7.70711 6.70711C7.31658 7.09763 6.68342 7.09763 6.29289 6.70711Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUserAdd;
impl IconShape for HiUserAdd {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M8 9C9.65685 9 11 7.65685 11 6C11 4.34315 9.65685 3 8 3C6.34315 3 5 4.34315 5 6C5 7.65685 6.34315 9 8 9Z",
            }
            path {
                d: "M8 11C11.3137 11 14 13.6863 14 17H2C2 13.6863 4.68629 11 8 11Z",
            }
            path {
                d: "M16 7C16 6.44772 15.5523 6 15 6C14.4477 6 14 6.44772 14 7V8H13C12.4477 8 12 8.44771 12 9C12 9.55228 12.4477 10 13 10H14V11C14 11.5523 14.4477 12 15 12C15.5523 12 16 11.5523 16 11V10H17C17.5523 10 18 9.55228 18 9C18 8.44772 17.5523 8 17 8H16V7Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUserCircle;
impl IconShape for HiUserCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM12 7C12 8.10457 11.1046 9 10 9C8.89543 9 8 8.10457 8 7C8 5.89543 8.89543 5 10 5C11.1046 5 12 5.89543 12 7ZM9.99993 11C7.98239 11 6.24394 12.195 5.45374 13.9157C6.55403 15.192 8.18265 16 9.99998 16C11.8173 16 13.4459 15.1921 14.5462 13.9158C13.756 12.195 12.0175 11 9.99993 11Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUserGroup;
impl IconShape for HiUserGroup {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M13 6C13 7.65685 11.6569 9 10 9C8.34315 9 7 7.65685 7 6C7 4.34315 8.34315 3 10 3C11.6569 3 13 4.34315 13 6Z",
            }
            path {
                d: "M18 8C18 9.10457 17.1046 10 16 10C14.8954 10 14 9.10457 14 8C14 6.89543 14.8954 6 16 6C17.1046 6 18 6.89543 18 8Z",
            }
            path {
                d: "M14 15C14 12.7909 12.2091 11 10 11C7.79086 11 6 12.7909 6 15V18H14V15Z",
            }
            path {
                d: "M6 8C6 9.10457 5.10457 10 4 10C2.89543 10 2 9.10457 2 8C2 6.89543 2.89543 6 4 6C5.10457 6 6 6.89543 6 8Z",
            }
            path {
                d: "M16 18V15C16 13.9459 15.7282 12.9552 15.2507 12.0943C15.4902 12.0327 15.7413 12 16 12C17.6569 12 19 13.3431 19 15V18H16Z",
            }
            path {
                d: "M4.74926 12.0943C4.27185 12.9552 4 13.9459 4 15V18H1V15C1 13.3431 2.34315 12 4 12C4.25871 12 4.50977 12.0327 4.74926 12.0943Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUserRemove;
impl IconShape for HiUserRemove {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M11 6C11 7.65685 9.65685 9 8 9C6.34315 9 5 7.65685 5 6C5 4.34315 6.34315 3 8 3C9.65685 3 11 4.34315 11 6Z",
            }
            path {
                d: "M14 17C14 13.6863 11.3137 11 8 11C4.68629 11 2 13.6863 2 17H14Z",
            }
            path {
                d: "M13 8C12.4477 8 12 8.44771 12 9C12 9.55229 12.4477 10 13 10H17C17.5523 10 18 9.55229 18 9C18 8.44771 17.5523 8 17 8H13Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUser;
impl IconShape for HiUser {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 9C11.6569 9 13 7.65685 13 6C13 4.34315 11.6569 3 10 3C8.34315 3 7 4.34315 7 6C7 7.65685 8.34315 9 10 9ZM3 18C3 14.134 6.13401 11 10 11C13.866 11 17 14.134 17 18H3Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiUsers;
impl IconShape for HiUsers {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M9 6C9 7.65685 7.65685 9 6 9C4.34315 9 3 7.65685 3 6C3 4.34315 4.34315 3 6 3C7.65685 3 9 4.34315 9 6Z",
            }
            path {
                d: "M17 6C17 7.65685 15.6569 9 14 9C12.3431 9 11 7.65685 11 6C11 4.34315 12.3431 3 14 3C15.6569 3 17 4.34315 17 6Z",
            }
            path {
                d: "M12.9291 17C12.9758 16.6734 13 16.3395 13 16C13 14.3648 12.4393 12.8606 11.4998 11.6691C12.2352 11.2435 13.0892 11 14 11C16.7614 11 19 13.2386 19 16V17H12.9291Z",
            }
            path {
                d: "M6 11C8.76142 11 11 13.2386 11 16V17H1V16C1 13.2386 3.23858 11 6 11Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiVariable;
impl IconShape for HiVariable {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.6485 3.08366C5.15459 3.30478 5.3856 3.89429 5.16448 4.40038C4.41582 6.11389 4 8.00707 4 10C4 11.9929 4.41582 13.8861 5.16448 15.5996C5.3856 16.1057 5.15459 16.6952 4.6485 16.9164C4.14242 17.1375 3.5529 16.9065 3.33178 16.4004C2.47486 14.4391 2 12.2737 2 10C2 7.72632 2.47486 5.56091 3.33178 3.59964C3.5529 3.09355 4.14242 2.86254 4.6485 3.08366ZM12.9613 7C12.0499 7 11.188 7.41427 10.6186 8.12592L10.2911 8.53528L10.1799 8.25722C9.87619 7.4979 9.14078 7 8.32297 7H8C7.44772 7 7 7.44772 7 8C7 8.55228 7.44772 9 8 9H8.32297L8.8551 10.3303L7.81962 11.6247C7.62985 11.8619 7.34253 12 7.03875 12H7C6.44772 12 6 12.4477 6 13C6 13.5523 6.44772 14 7 14H7.03875C7.9501 14 8.81204 13.5857 9.38136 12.8741L9.70885 12.4647L9.82008 12.7428C10.1238 13.5021 10.8592 14 11.677 14H12C12.5523 14 13 13.5523 13 13C13 12.4477 12.5523 12 12 12H11.677L11.1449 10.6697L12.1804 9.37531C12.3702 9.13809 12.6575 9 12.9613 9H13C13.5523 9 14 8.55228 14 8C14 7.44772 13.5523 7 13 7H12.9613ZM14.8355 4.40038C14.6144 3.89429 14.8454 3.30478 15.3515 3.08366C15.8576 2.86254 16.4471 3.09355 16.6682 3.59964C17.5251 5.56091 18 7.72632 18 10C18 12.2737 17.5251 14.4391 16.6682 16.4004C16.4471 16.9065 15.8576 17.1375 15.3515 16.9164C14.8454 16.6952 14.6144 16.1057 14.8355 15.5996C15.5842 13.8861 16 11.9929 16 10C16 8.00707 15.5842 6.11389 14.8355 4.40038Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiVideoCamera;
impl IconShape for HiVideoCamera {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 6C2 4.89543 2.89543 4 4 4H10C11.1046 4 12 4.89543 12 6V14C12 15.1046 11.1046 16 10 16H4C2.89543 16 2 15.1046 2 14V6Z",
            }
            path {
                d: "M14.5528 7.10557C14.214 7.27497 14 7.62123 14 8V12C14 12.3788 14.214 12.725 14.5528 12.8944L16.5528 13.8944C16.8628 14.0494 17.2309 14.0329 17.5257 13.8507C17.8205 13.6684 18 13.3466 18 13V7C18 6.65342 17.8205 6.33156 17.5257 6.14935C17.2309 5.96714 16.8628 5.95058 16.5528 6.10557L14.5528 7.10557Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiViewBoards;
impl IconShape for HiViewBoards {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M2 4C2 3.44772 2.44772 3 3 3H5C5.55228 3 6 3.44772 6 4V16C6 16.5523 5.55228 17 5 17H3C2.44772 17 2 16.5523 2 16V4Z",
            }
            path {
                d: "M8 4C8 3.44772 8.44772 3 9 3H11C11.5523 3 12 3.44772 12 4V16C12 16.5523 11.5523 17 11 17H9C8.44772 17 8 16.5523 8 16V4Z",
            }
            path {
                d: "M15 3C14.4477 3 14 3.44772 14 4V16C14 16.5523 14.4477 17 15 17H17C17.5523 17 18 16.5523 18 16V4C18 3.44772 17.5523 3 17 3H15Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiViewGridAdd;
impl IconShape for HiViewGridAdd {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 3C3.89543 3 3 3.89543 3 5V7C3 8.10457 3.89543 9 5 9H7C8.10457 9 9 8.10457 9 7V5C9 3.89543 8.10457 3 7 3H5Z",
            }
            path {
                d: "M5 11C3.89543 11 3 11.8954 3 13V15C3 16.1046 3.89543 17 5 17H7C8.10457 17 9 16.1046 9 15V13C9 11.8954 8.10457 11 7 11H5Z",
            }
            path {
                d: "M11 5C11 3.89543 11.8954 3 13 3H15C16.1046 3 17 3.89543 17 5V7C17 8.10457 16.1046 9 15 9H13C11.8954 9 11 8.10457 11 7V5Z",
            }
            path {
                d: "M14 11C14.5523 11 15 11.4477 15 12V13H16C16.5523 13 17 13.4477 17 14C17 14.5523 16.5523 15 16 15H15V16C15 16.5523 14.5523 17 14 17C13.4477 17 13 16.5523 13 16V15H12C11.4477 15 11 14.5523 11 14C11 13.4477 11.4477 13 12 13H13V12C13 11.4477 13.4477 11 14 11Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiViewGrid;
impl IconShape for HiViewGrid {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 3C3.89543 3 3 3.89543 3 5V7C3 8.10457 3.89543 9 5 9H7C8.10457 9 9 8.10457 9 7V5C9 3.89543 8.10457 3 7 3H5Z",
            }
            path {
                d: "M5 11C3.89543 11 3 11.8954 3 13V15C3 16.1046 3.89543 17 5 17H7C8.10457 17 9 16.1046 9 15V13C9 11.8954 8.10457 11 7 11H5Z",
            }
            path {
                d: "M11 5C11 3.89543 11.8954 3 13 3H15C16.1046 3 17 3.89543 17 5V7C17 8.10457 16.1046 9 15 9H13C11.8954 9 11 8.10457 11 7V5Z",
            }
            path {
                d: "M11 13C11 11.8954 11.8954 11 13 11H15C16.1046 11 17 11.8954 17 13V15C17 16.1046 16.1046 17 15 17H13C11.8954 17 11 16.1046 11 15V13Z",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiViewList;
impl IconShape for HiViewList {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M3 4C3 3.44772 3.44772 3 4 3H16C16.5523 3 17 3.44772 17 4C17 4.55228 16.5523 5 16 5H4C3.44772 5 3 4.55228 3 4ZM3 8C3 7.44772 3.44772 7 4 7H16C16.5523 7 17 7.44772 17 8C17 8.55228 16.5523 9 16 9H4C3.44772 9 3 8.55228 3 8ZM3 12C3 11.4477 3.44772 11 4 11H16C16.5523 11 17 11.4477 17 12C17 12.5523 16.5523 13 16 13H4C3.44772 13 3 12.5523 3 12ZM3 16C3 15.4477 3.44772 15 4 15H16C16.5523 15 17 15.4477 17 16C17 16.5523 16.5523 17 16 17H4C3.44772 17 3 16.5523 3 16Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiVolumeOff;
impl IconShape for HiVolumeOff {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.38268 3.07615C9.75636 3.23093 10 3.59557 10 4.00003V16C10 16.4045 9.75636 16.7691 9.38268 16.9239C9.00901 17.0787 8.57889 16.9931 8.29289 16.7071L4.58579 13H2C1.44772 13 1 12.5523 1 12V8.00003C1 7.44774 1.44772 7.00003 2 7.00003H4.58579L8.29289 3.29292C8.57889 3.00692 9.00901 2.92137 9.38268 3.07615Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M12.2929 7.29289C12.6834 6.90237 13.3166 6.90237 13.7071 7.29289L15 8.58579L16.2929 7.29289C16.6834 6.90237 17.3166 6.90237 17.7071 7.29289C18.0976 7.68342 18.0976 8.31658 17.7071 8.70711L16.4142 10L17.7071 11.2929C18.0976 11.6834 18.0976 12.3166 17.7071 12.7071C17.3166 13.0976 16.6834 13.0976 16.2929 12.7071L15 11.4142L13.7071 12.7071C13.3166 13.0976 12.6834 13.0976 12.2929 12.7071C11.9024 12.3166 11.9024 11.6834 12.2929 11.2929L13.5858 10L12.2929 8.70711C11.9024 8.31658 11.9024 7.68342 12.2929 7.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiVolumeUp;
impl IconShape for HiVolumeUp {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M9.38268 3.07615C9.75636 3.23093 10 3.59557 10 4.00003V16C10 16.4045 9.75636 16.7691 9.38268 16.9239C9.00901 17.0787 8.57889 16.9931 8.29289 16.7071L4.58579 13H2C1.44772 13 1 12.5523 1 12V8.00003C1 7.44774 1.44772 7.00003 2 7.00003H4.58579L8.29289 3.29292C8.57889 3.00692 9.00901 2.92137 9.38268 3.07615Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M14.6568 2.92888C15.0474 2.53836 15.6805 2.53836 16.0711 2.92888C17.8796 4.73743 19 7.2388 19 9.99995C19 12.7611 17.8796 15.2625 16.0711 17.071C15.6805 17.4615 15.0474 17.4615 14.6568 17.071C14.2663 16.6805 14.2663 16.0473 14.6568 15.6568C16.1057 14.208 17 12.2094 17 9.99995C17 7.79053 16.1057 5.7919 14.6568 4.34309C14.2663 3.95257 14.2663 3.3194 14.6568 2.92888ZM11.8284 5.75731C12.2189 5.36678 12.8521 5.36678 13.2426 5.75731C13.7685 6.28319 14.1976 6.90687 14.5003 7.59958C14.822 8.33592 15 9.14847 15 9.99995C15 11.6565 14.3273 13.1579 13.2426 14.2426C12.8521 14.6331 12.2189 14.6331 11.8284 14.2426C11.4379 13.8521 11.4379 13.2189 11.8284 12.8284C12.5534 12.1034 13 11.1048 13 9.99995C13 9.42922 12.8811 8.8889 12.6676 8.40032C12.4663 7.93958 12.1802 7.52327 11.8284 7.17152C11.4379 6.781 11.4379 6.14783 11.8284 5.75731Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiWifi;
impl IconShape for HiWifi {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M17.7781 8.22183C13.4823 3.92606 6.51752 3.92606 2.22176 8.22183C1.83123 8.61235 1.19807 8.61235 0.807542 8.22183C0.417017 7.8313 0.417017 7.19814 0.807542 6.80761C5.88436 1.7308 14.1155 1.7308 19.1923 6.80761C19.5828 7.19814 19.5828 7.8313 19.1923 8.22183C18.8018 8.61235 18.1686 8.61235 17.7781 8.22183ZM14.9497 11.0503C12.216 8.31659 7.78385 8.31659 5.05018 11.0503C4.65966 11.4408 4.02649 11.4408 3.63597 11.0503C3.24544 10.6597 3.24544 10.0266 3.63597 9.63605C7.15069 6.12133 12.8492 6.12133 16.3639 9.63605C16.7544 10.0266 16.7544 10.6597 16.3639 11.0503C15.9734 11.4408 15.3402 11.4408 14.9497 11.0503ZM12.1213 13.8787C10.9497 12.7071 9.05018 12.7071 7.87861 13.8787C7.48809 14.2692 6.85492 14.2692 6.4644 13.8787C6.07387 13.4882 6.07387 12.855 6.4644 12.4645C8.41702 10.5119 11.5828 10.5119 13.5355 12.4645C13.926 12.855 13.926 13.4882 13.5355 13.8787C13.1449 14.2692 12.5118 14.2692 12.1213 13.8787ZM8.99993 16C8.99993 15.4477 9.44765 15 9.99993 15H10.0099C10.5622 15 11.0099 15.4477 11.0099 16C11.0099 16.5523 10.5622 17 10.0099 17H9.99993C9.44765 17 8.99993 16.5523 8.99993 16Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiXCircle;
impl IconShape for HiXCircle {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M10 18C14.4183 18 18 14.4183 18 10C18 5.58172 14.4183 2 10 2C5.58172 2 2 5.58172 2 10C2 14.4183 5.58172 18 10 18ZM8.70711 7.29289C8.31658 6.90237 7.68342 6.90237 7.29289 7.29289C6.90237 7.68342 6.90237 8.31658 7.29289 8.70711L8.58579 10L7.29289 11.2929C6.90237 11.6834 6.90237 12.3166 7.29289 12.7071C7.68342 13.0976 8.31658 13.0976 8.70711 12.7071L10 11.4142L11.2929 12.7071C11.6834 13.0976 12.3166 13.0976 12.7071 12.7071C13.0976 12.3166 13.0976 11.6834 12.7071 11.2929L11.4142 10L12.7071 8.70711C13.0976 8.31658 13.0976 7.68342 12.7071 7.29289C12.3166 6.90237 11.6834 6.90237 11.2929 7.29289L10 8.58579L8.70711 7.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiX;
impl IconShape for HiX {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M4.29289 4.29289C4.68342 3.90237 5.31658 3.90237 5.70711 4.29289L10 8.58579L14.2929 4.29289C14.6834 3.90237 15.3166 3.90237 15.7071 4.29289C16.0976 4.68342 16.0976 5.31658 15.7071 5.70711L11.4142 10L15.7071 14.2929C16.0976 14.6834 16.0976 15.3166 15.7071 15.7071C15.3166 16.0976 14.6834 16.0976 14.2929 15.7071L10 11.4142L5.70711 15.7071C5.31658 16.0976 4.68342 16.0976 4.29289 15.7071C3.90237 15.3166 3.90237 14.6834 4.29289 14.2929L8.58579 10L4.29289 5.70711C3.90237 5.31658 3.90237 4.68342 4.29289 4.29289Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiZoomIn;
impl IconShape for HiZoomIn {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                d: "M5 8C5 7.44772 5.44772 7 6 7H7V6C7 5.44772 7.44772 5 8 5C8.55228 5 9 5.44772 9 6V7H10C10.5523 7 11 7.44772 11 8C11 8.55228 10.5523 9 10 9H9V10C9 10.5523 8.55228 11 8 11C7.44771 11 7 10.5523 7 10V9H6C5.44772 9 5 8.55228 5 8Z",
            }
            path {
                clip_rule: "evenodd",
                d: "M2 8C2 4.68629 4.68629 2 8 2C11.3137 2 14 4.68629 14 8C14 9.29583 13.5892 10.4957 12.8907 11.4765L17.7071 16.2929C18.0976 16.6834 18.0976 17.3166 17.7071 17.7071C17.3166 18.0976 16.6834 18.0976 16.2929 17.7071L11.4765 12.8907C10.4957 13.5892 9.29583 14 8 14C4.68629 14 2 11.3137 2 8ZM8 4C5.79086 4 4 5.79086 4 8C4 10.2091 5.79086 12 8 12C10.2091 12 12 10.2091 12 8C12 5.79086 10.2091 4 8 4Z",
                fill_rule: "evenodd",
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HiZoomOut;
impl IconShape for HiZoomOut {
    fn view_box(&self) -> &str {
        "0 0 20 20"
    }
    fn width(&self) -> &str {
        "20"
    }
    fn height(&self) -> &str {
        "20"
    }
    fn xmlns(&self) -> &str {
        "http://www.w3.org/2000/svg"
    }
    fn fill(&self) -> &str {
        "none"
    }
    fn stroke(&self) -> &str {
        "none"
    }
    fn stroke_width(&self) -> &str {
        "1"
    }
    fn stroke_linecap(&self) -> &str {
        "butt"
    }
    fn stroke_linejoin(&self) -> &str {
        "miter"
    }
    fn child_elements(&self) -> Element {
        rsx! {
            path {
                clip_rule: "evenodd",
                d: "M8 4C5.79086 4 4 5.79086 4 8C4 10.2091 5.79086 12 8 12C10.2091 12 12 10.2091 12 8C12 5.79086 10.2091 4 8 4ZM2 8C2 4.68629 4.68629 2 8 2C11.3137 2 14 4.68629 14 8C14 9.29583 13.5892 10.4957 12.8907 11.4765L17.7071 16.2929C18.0976 16.6834 18.0976 17.3166 17.7071 17.7071C17.3166 18.0976 16.6834 18.0976 16.2929 17.7071L11.4765 12.8907C10.4957 13.5892 9.29583 14 8 14C4.68629 14 2 11.3137 2 8Z",
                fill_rule: "evenodd",
            }
            path {
                clip_rule: "evenodd",
                d: "M5 8C5 7.44772 5.44772 7 6 7H10C10.5523 7 11 7.44772 11 8C11 8.55228 10.5523 9 10 9H6C5.44772 9 5 8.55228 5 8Z",
                fill_rule: "evenodd",
            }
        }
    }
}
