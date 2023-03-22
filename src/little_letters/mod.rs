pub fn to_little_letter(c: char) -> char {
    match c {
        '0' => '⁰',
        '1' => '¹',
        '2' => '²',
        '3' => '³',
        '4' => '⁴',
        '5' => '⁵',
        '6' => '⁶',
        '7' => '⁷',
        '8' => '⁸',
        '9' => '⁹',
        'a' => 'ᵃ',
        'ą' => 'ᵃ',
        'b' => 'ᵇ',
        'c' => 'ᶜ',
        'ć' => 'ᶜ',
        'd' => 'ᵈ',
        'e' => 'ᵉ',
        'ę' => 'ᵉ',
        'f' => 'ᶠ',
        'g' => 'ᵍ',
        'h' => 'ʰ',
        'i' => 'ᶦ',
        'j' => 'ʲ',
        'k' => 'ᵏ',
        'l' => 'ˡ',
        'ł' => 'ˡ',
        'm' => 'ᵐ',
        'n' => 'ⁿ',
        'ń' => 'ⁿ',
        'o' => 'ᵒ',
        'ó' => 'ᵒ',
        'p' => 'ᵖ',
        'r' => 'ʳ',
        's' => 'ˢ',
        'ś' => 'ˢ',
        't' => 'ᵗ',
        'u' => 'ᵘ',
        'v' => 'ᵛ',
        'w' => 'ʷ',
        'x' => 'ˣ',
        'y' => 'ʸ',
        'z' => 'ᶻ',
        'ź' => 'ᶻ',
        'ż' => 'ᶻ',
        'A' => 'ᴬ',
        'Ą' => 'ᴬ',
        'B' => 'ᴮ',
        'C' => 'ᶜ',
        'Ć' => 'ᶜ',
        'D' => 'ᴰ',
        'E' => 'ᴱ',
        'Ę' => 'ᴱ',
        'F' => 'ᶠ',
        'G' => 'ᴳ',
        'H' => 'ᴴ',
        'I' => 'ᴵ',
        'J' => 'ᴶ',
        'K' => 'ᴷ',
        'L' => 'ᴸ',
        'Ł' => 'ᴸ',
        'M' => 'ᴹ',
        'N' => 'ᴺ',
        'Ń' => 'ᴺ',
        'O' => 'ᴼ',
        'Ó' => 'ᴼ',
        'P' => 'ᴾ',
        'R' => 'ᴿ',
        'S' => 'ˢ',
        'Ś' => 'ˢ',
        'T' => 'ᵀ',
        'U' => 'ᵁ',
        'V' => 'ⱽ',
        'W' => 'ᵂ',
        'X' => 'ˣ',
        'Y' => 'ʸ',
        'Z' => 'ᶻ',
        'Ź' => 'ᶻ',
        'Ż' => 'ᶻ',
        '+' => '⁺',
        '-' => '⁻',
        '=' => '⁼',
        '(' => '⁽',
        ')' => '⁾',
        'q' => 'ᵠ',
        'Q' => 'ᵠ',
        '?' => 'ˀ',
        '!' => 'ᵎ',
        ' ' => ' ',
        '.' => '·',
        _ => c,
    }
}

pub fn string_to_little_letters(s: &str) -> String {
    s.chars().map(|c| to_little_letter(c)).collect()
}

#[cfg(test)]
mod tests {
    use crate::little_letters::string_to_little_letters;

    #[test]
    fn it_converts_single_letters() {
        let result = string_to_little_letters("L");
        assert_eq!(result, "ᴸ");
    }

    #[test]
    fn it_converts_words() {
        let result = string_to_little_letters("Lorem ipsum");
        assert_eq!(result, "ᴸᵒʳᵉᵐ ᶦᵖˢᵘᵐ");
    }
}
