use crate::mappings::{
    common::FontFamily,
    signatures::{
        FM_SIGNATURES,
        DL_SIGNATURES,
        WIJEYA_SIGNATURES,
        KAPUTA_SIGNATURES,
    },
};

fn score_chars(text: &str, signatures: &[char],) -> usize {
    text.chars()
        .filter(|c| signatures.contains(c))
        .count()
}

fn score_fm(text: &str,) -> usize {
    score_chars(
        text,
        FM_SIGNATURES,
    )
}

fn score_dl(text: &str,) -> usize {
    score_chars(
        text,
        DL_SIGNATURES,
    )
}

fn score_wijeya(text: &str,) -> usize {
    score_chars(
        text,
        WIJEYA_SIGNATURES,
    )
}

fn score_kaputa(text: &str,) -> usize {
    score_chars(
        text,
        KAPUTA_SIGNATURES,
    )
}

pub fn detect(text: &str,) -> FontFamily {
    let fm = score_fm(text);
    let dl = score_dl(text);
    let wijeya = score_wijeya(text);
    let kaputa = score_kaputa(text);

    let scores = [
        (FontFamily::FM, fm),
        (FontFamily::DL, dl),
        (FontFamily::Wijeya, wijeya),
        (FontFamily::Kaputa, kaputa),
    ];

    let (family, score) = scores
        .into_iter()
        .max_by_key(|(_, s)| *s)
        .unwrap();

    if score == 0 {
        FontFamily::Unknown
    } else {
        family
    }
}