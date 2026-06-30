use crate::engine::MappingSet;

pub fn apply_rules(text: &str,mapping: &MappingSet,) -> String {
    if mapping.rules.is_empty() {
        return text.to_string();
    }

    let chars: Vec<char> = text.chars().collect();

    let mut result =
        String::with_capacity(text.len());

    let mut i = 0;

    while i < chars.len() {
        let mut matched = false;

        for len in
            (1..=mapping.max_rule_len).rev()
        {
            if i + len > chars.len() {
                continue;
            }

            let candidate: String =
                chars[i..i + len]
                    .iter()
                    .collect();

            if let Some(replacement) =
                mapping.rules.get(
                    candidate.as_str(),
                )
            {
                result.push_str(replacement);
                i += len;
                matched = true;
                break;
            }
        }

        if matched {
            continue;
        }

        result.push(chars[i]);
        i += 1;
    }

    result
}