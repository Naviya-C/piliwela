// src/engine/kombuva.rs

const PRE_BASE_VOWELS: [char; 4] = [
    'ෙ', // kombuva
    'ො',
    'ෝ',
    'ෞ',
];

fn is_pre_base_vowel(c: char) -> bool {
    PRE_BASE_VOWELS.contains(&c)
}

fn is_sinhala_consonant(c: char) -> bool {
    matches!(
        c as u32,
        0x0D9A..=0x0DC6
    )
}

fn is_sign(c: char) -> bool {
    matches!(
        c,
        '්'
            | 'ා'
            | 'ැ'
            | 'ෑ'
            | 'ි'
            | 'ී'
            | 'ු'
            | 'ූ'
            | 'ෘ'
            | 'ෲ'
            | 'ෟ'
            | 'ං'
            | 'ඃ'
    )
}

pub fn fix_kombuva(text: &str) -> String {
    let chars: Vec<char> =
        text.chars().collect();

    let mut result =
        String::with_capacity(text.len());

    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if is_pre_base_vowel(c)
            && i + 1 < chars.len()
        {
            let mut j = i + 1;

            while j < chars.len()
                && is_sign(chars[j])
            {
                j += 1;
            }

            if j < chars.len()
                && is_sinhala_consonant(
                    chars[j],
                )
            {
                result.push(chars[j]);

                for k in i + 1..j {
                    result.push(chars[k]);
                }

                result.push(c);

                i = j + 1;
                continue;
            }
        }

        result.push(c);
        i += 1;
    }

    result
}