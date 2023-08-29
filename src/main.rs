use rand::{seq::SliceRandom, thread_rng};
use std::io;

const HIRAGANA: [(&str, &str); 46] = [
    ("あ", "a"),
    ("い", "i"),
    ("う", "u"),
    ("え", "e"),
    ("お", "o"),
    ("か", "ka"),
    ("き", "ki"),
    ("く", "ku"),
    ("け", "ke"),
    ("こ", "ko"),
    ("さ", "sa"),
    ("し", "shi"),
    ("す", "su"),
    ("せ", "se"),
    ("そ", "so"),
    ("た", "ta"),
    ("ち", "chi"),
    ("つ", "tsu"),
    ("て", "te"),
    ("と", "to"),
    ("な", "na"),
    ("に", "ni"),
    ("ぬ", "nu"),
    ("ね", "ne"),
    ("の", "no"),
    ("は", "ha"),
    ("ひ", "hi"),
    ("ふ", "fu"),
    ("へ", "he"),
    ("ほ", "ho"),
    ("ま", "ma"),
    ("み", "mi"),
    ("む", "mu"),
    ("め", "me"),
    ("も", "mo"),
    ("や", "ya"),
    ("ゆ", "yu"),
    ("よ", "yo"),
    ("ら", "ra"),
    ("り", "ri"),
    ("る", "ru"),
    ("れ", "re"),
    ("ろ", "ro"),
    ("わ", "wa"),
    ("を", "o"),
    ("ん", "n"),
];
const KATAKANA: [(&str, &str); 46] = [
    ("ア", "a"),
    ("イ", "i"),
    ("ウ", "u"),
    ("エ", "e"),
    ("オ", "o"),
    ("カ", "ka"),
    ("キ", "ki"),
    ("ク", "ku"),
    ("ケ", "ke"),
    ("コ", "ko"),
    ("サ", "sa"),
    ("シ", "shi"),
    ("ス", "su"),
    ("セ", "se"),
    ("ソ", "so"),
    ("タ", "ta"),
    ("チ", "chi"),
    ("ツ", "tsu"),
    ("テ", "te"),
    ("ト", "to"),
    ("ナ", "na"),
    ("ニ", "ni"),
    ("ヌ", "nu"),
    ("ネ", "ne"),
    ("ノ", "no"),
    ("ハ", "ha"),
    ("ヒ", "hi"),
    ("フ", "fu"),
    ("ヘ", "he"),
    ("ホ", "ho"),
    ("マ", "ma"),
    ("ミ", "mi"),
    ("ム", "mu"),
    ("メ", "me"),
    ("モ", "mo"),
    ("ヤ", "ya"),
    ("ユ", "yu"),
    ("ヨ", "yo"),
    ("ラ", "ra"),
    ("リ", "ri"),
    ("ル", "ru"),
    ("レ", "re"),
    ("ロ", "ro"),
    ("ワ", "wa"),
    ("ヲ", "o"),
    ("ン", "n"),
];

fn main() {
    let mut choice_input = String::new();
    println!("Practice hiragana (1), katakana (2), or both (3)?");
    io::stdin().read_line(&mut choice_input).unwrap();
    let chars = match choice_input.trim() {
        "1" => {
            println!("Hiragana selected.");
            HIRAGANA.to_vec()
        }
        "2" => {
            println!("Katakana selected.");
            KATAKANA.to_vec()
        }
        _ => {
            println!("Both selected.");
            [HIRAGANA, KATAKANA].concat()
        }
    };
    println!();

    let mut rng = thread_rng();
    let mut right = 0;
    let mut wrong = 0;
    let mut seen_kana = vec![];

    loop {
        let kana;
        let answer;

        loop {
            let (kana_, answer_) = chars.choose(&mut rng).unwrap();
            if !seen_kana.contains(kana_) {
                kana = *kana_;
                answer = answer_;
                break;
            }
        }

        seen_kana.insert(0, kana);
        if seen_kana.len() > 5 {
            seen_kana.pop();
        }
        println!("{kana}");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if &input.trim() == answer {
            right += 1;
            println!("Correct!");
        } else {
            wrong += 1;
            println!("You fool! That's {answer}!");
        }
        println!("{right}/{} correct this session.\n", right + wrong);
    }
}
