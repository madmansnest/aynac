use std::borrow::Cow;
use regex::Regex;
use lazy_static::lazy_static;

#[allow(non_upper_case_globals)]
fn cyrillic_to_latin(input: &str) -> Cow<str> {
    lazy_static! {
        static ref QazaqCapitals: Regex = Regex::new("^[А-ЯІЁҒҚҢҮҰҺӘӨ]+$").unwrap();
    }
    let is_all_caps = QazaqCapitals.is_match(input);
    let mut output = String::new();
    let mut last_vowel_was_jinicke = false;
    let mut last_was_vowel = false;
    for character in input.chars() {
        let nochange = &character.to_string();
        let replacement = match character {
            'А' => "A",
            'а' => "a",
            'Ә' => "Ä",
            'ә' => "ä",
            'Б' => "B",
            'б' => "b",
            'В' => "V",
            'в' => "v",
            'Г' => "G",
            'г' => "g",
            'Ғ' => "Ğ",
            'ғ' => "ğ",
            'Д' => "D",
            'д' => "d",
            'Е' => "E",
            'е' => "e",
            'Ё' => "Yo",
            'ё' => "yo",
            'Ж' => "J",
            'ж' => "j",
            'З' => "Z",
            'з' => "z",
            'И' => {
                if last_vowel_was_jinicke { 
                    if is_all_caps { "IY" }
                    else { "Iy" }
                }
                else { 
                    if is_all_caps { "IY" }
                    else { "Iy" }
                }
            }
            'и' => {
                if last_vowel_was_jinicke { "iy" }
                else { "ıy" }
            }
            'Й' => "Y",
            'й' => "y",
            'К' => "K",
            'к' => "k",
            'Қ' => "Q",
            'қ' => "q",
            'Л' => "L",
            'л' => "l",
            'М' => "M",
            'м' => "m",
            'Н' => "N",
            'н' => "n",
            'Ң' => "Ñ",
            'ң' => "ñ",
            'О' => "O",
            'о' => "o",
            'Ө' => "Ö",
            'ө' => "ö",
            'П' => "P",
            'п' => "p",
            'Р' => "R",
            'р' => "r",
            'С' => "S",
            'с' => "s",
            'Т' => "T",
            'т' => "t",
            'У' => {
                if last_was_vowel { "W" }
                else {
                    if last_vowel_was_jinicke { 
                        if is_all_caps { "IW" }
                        else { "Iw" }
                    }
                    else { 
                        if is_all_caps { "UW" }
                        else { "Uw" }
                    }
                }
            }
            'у' => {
                if last_was_vowel { "w" }
                else {
                    if last_vowel_was_jinicke { "iw" }
                    else { "uw" }
                }
            }
            'Ұ' => "U",
            'ұ' => "u",
            'Ү' => "Ü",
            'ү' => "ü",
            'Ф' => "F",
            'ф' => "f",
            'Х' => "H",
            'х' => "h",
            'Һ' => "H",
            'һ' => "h",
            'Ц' => "S",
            'ц' => "s",
            'Ч' => "Ch",
            'ч' => "ch",
            'Ш' => "C",
            'ш' => "c",
            'Щ' => "Cc",
            'щ' => "cc",
            'Ъ' => "",
            'ъ' => "",
            'Ы' => "I",
            'ы' => "ı",
            'І' => "I",
            'і' => "i",
            'Ь' => "",
            'ь' => "",
            'Э' => "Ä",
            'э' => "ä",
            'Ю' => "Yu",
            'ю' => "yu",
            'Я' => "Ya",
            'я' => "ya",
            _ => nochange
        };
        output.push_str(&replacement);
        last_was_vowel = false;
        let juan_vowels = ['А','а','Ё','ё','О','о','У','у','Ұ','ұ','Ы','ы','Ю','ю','Я','я'];
        if juan_vowels.contains(&character) {
            last_was_vowel = true;
            last_vowel_was_jinicke = false;
        }
        let jinicke_vowels = ['Ә','ә','Е','е','И','и','Ө','ө','Ү','ү','І','і','Э','э',];
        if jinicke_vowels.contains(&character) {
            last_was_vowel = true;
            last_vowel_was_jinicke = true;
        }
    }
    Cow::from(output)
}

#[allow(non_upper_case_globals)]
pub fn convert(input: &str) -> Cow<str> {
    lazy_static! {
        static ref QazaqLetters: Regex = Regex::new("[А-яІіЁёҒғҚқҢңҮүҰұҺһӘәӨө]+").unwrap();
    }
    let mut output = String::new();
    let mut start = 0;
    for m in QazaqLetters.find_iter(input) {
        output.push_str(&input[start..m.start()]);
        output.push_str(&cyrillic_to_latin(m.as_str()));
        start = m.end();
    }
    output.push_str(&input[start..]);
    Cow::from(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_from_cyrillic_to_latin() {
        assert_eq!("Ayttım sälem, Qalamqas!", convert("Айттым сәлем, Қаламқас!"));
    }

    #[test]
    fn conversion_from_cyrillic_to_latin_mıy() {
        assert_eq!("mıy", convert("ми"));
    }

    #[test]
    fn conversion_from_cyrillic_to_latin_suw() {
        assert_eq!("suw", convert("су"));
    }

    #[test]
    fn conversion_from_cyrillic_to_latin_jüziwci() {
        assert_eq!("jüziwci", convert("жүзуші"));
    }

    #[test]
    fn conversion_from_cyrillic_to_latin_jasaw() {
        assert_eq!("jasaw", convert("жасау"));
    }

    #[test]
    fn conversion_from_cyrillic_to_latin_all_caps() {
        assert_eq!("JAZUW", convert("ЖАЗУ"));
    }
}
