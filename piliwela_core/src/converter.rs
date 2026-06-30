use crate::detector;
use crate::engine;
use crate::options::ConvertOptions;

use crate::mappings::{
    common::FontFamily,
    fm,
    //dl,
    //wijeya,
    //kaputa,
};

pub fn convert(
    text: &str,
    family: FontFamily,
    options: &ConvertOptions,
) -> String {
    match family {
        FontFamily::FM => {
            engine::convert(
                text,
                &fm::FM_MAPPING,
                options,
            )
        }

        FontFamily::DL => {
            text.to_string()
        }

        FontFamily::Wijeya => {
            text.to_string()
        }

        FontFamily::Kaputa => {
            text.to_string()
        }

        FontFamily::Unknown => {
            text.to_string()
        }
    }
}

pub fn convert_auto(
    text: &str,
    options: &ConvertOptions,
) -> String {
    let family =
        detector::detect(text);

    convert(
        text,
        family,
        options,
    )
}