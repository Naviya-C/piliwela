use crate::engine::MappingSet;

pub fn apply_combos_and_singles(text: &str,mapping: &MappingSet,) -> String {
    let chars: Vec<char> = text.chars().collect();

    let mut result = String::with_capacity(text.len());

    let mut i = 0;

    while i < chars.len() {
        let mut found_combo = false;

        for len in
            (2..=mapping.max_combo_len).rev()
        {
            if i + len > chars.len() {
                continue;
            }

            let candidate: String =
                chars[i..i + len].iter().collect();

            if let Some(mapped) =
                mapping.combos.get(
                    candidate.as_str(),
                )
            {
                result.push_str(mapped);
                i += len;
                found_combo = true;
                break;
            }
        }

        if found_combo {
            continue;
        }

        let c = chars[i];

        if let Some(mapped) =
            mapping.singles.get(&c)
        {
            result.push_str(mapped);
        } else {
            result.push(c);
        }

        i += 1;
    }

    result
}