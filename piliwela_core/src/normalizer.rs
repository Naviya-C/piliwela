// src/normalizer.rs

use unicode_normalization::
    UnicodeNormalization;

fn remove_duplicate_zwj(
    text: &str,
) -> String {
    let mut result =
        String::new();

    let mut prev =
        None;

    for c in text.chars() {
        if c == '\u{200D}'
            && prev == Some(c)
        {
            continue;
        }

        result.push(c);
        prev = Some(c);
    }

    result
}

fn remove_duplicate_zwnj(
    text: &str,
) -> String {
    let mut result =
        String::new();

    let mut prev =
        None;

    for c in text.chars() {
        if c == '\u{200C}'
            && prev == Some(c)
        {
            continue;
        }

        result.push(c);
        prev = Some(c);
    }

    result
}

fn collapse_whitespace(
    text: &str,
) -> String {
    let mut result =
        String::new();

    let mut prev_ws =
        false;

    for c in text.chars() {
        if c.is_whitespace() {
            if !prev_ws {
                result.push(' ');
            }

            prev_ws = true;
        } else {
            prev_ws = false;
            result.push(c);
        }
    }

    result
}

pub fn normalize(
    text: &str,
) -> String {
    let text =
        remove_duplicate_zwj(
            text,
        );

    let text =
        remove_duplicate_zwnj(
            &text,
        );

    let text =
        collapse_whitespace(
            &text,
        );

    text.nfc().collect()
}