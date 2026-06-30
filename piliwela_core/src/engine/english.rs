pub fn should_preserve(token: &str,) -> bool{
    preserve_single_char(token) || looks_like_english(token)
}

fn looks_like_english(token: &str,) -> bool{
    token.len() > 2
        && token.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || matches!(
                    c,
                    '-'
                        | '_'
                        | '.'
                        | '+'
                        | '#'
                )
    })
}

fn preserve_single_char(token: &str,) -> bool{
    matches!(
        token,
        "a"
            | "A"
            | "f"
            | "e"
            | "E"
            | "q"
            | "Q"
            | "s"
            | "S"
            | "d"
            | "D"
            | "H"
            | "%"
    )
}

/*
//Future use
fn looks_like_legacy(token: &str,) -> bool{
    const LEGACY_HINTS: [char; 7] =
    ['[', ']', ';', '%', '^', '~', '{'];

    token.chars().any(|c| {
        LEGACY_HINTS.contains(&c)
    })
}
*/