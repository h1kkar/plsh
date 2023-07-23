use rand::Rng;
use colorized::*;

/// A function that prints goodbye in different languages when exiting.
pub fn say() -> (String, Colors) {
    match n() {
        0 => (String::from("bye bye"), Colors::BlueFg), //eng
        1 => (String::from("وداعا وداعا"), Colors::GreenFg), //ara
        2 => (String::from("totsiens"), Colors::YellowFg), //afr
        3 => (String::from("пакуль пакуль"), Colors::RedFg), //bel
        4 => (String::from("чао чао"), Colors::BrightGreenFg), //bul
        5 => (String::from("hwyl fawr"), Colors::CyanFg), //cym/wel
        6 => (String::from("tạm biệt"), Colors::RedFg), //vie
        7 => (String::from("aloha"), Colors::BlueFg), //haw
        8 => (String::from("Αντίο"), Colors::BrightBlueFg), //ell/gre
        9 => (String::from("ნახვამდის"), Colors::RedFg), //geo/kat
        10 => (String::from("uhambe kahle"), Colors::BrightYellowFg), //zul
        11 => (String::from("ka ọ dị"), Colors::BrightGreenFg), //ibo
        12 => (String::from("sampai jumpa"), Colors::RedFg), //ind
        13 => (String::from("o digba"), Colors::YellowFg), //yor
        14 => (String::from("сау бол"), Colors::CyanFg), //kaz
        15 => (String::from("再见"), Colors::RedFg), //chi/zho
        16 => (String::from("안녕"), Colors::BrightBlueFg), //kor
        17 => (String::from("លាហើយ"), Colors::BlueFg), //khm
        18 => (String::from("iki"), Colors::BrightYellowFg), //lit
        19 => (String::from("ബൈ ബൈ"), Colors::BrightYellowFg), //mal
        20 => (String::from("kia ora"), Colors::BlueFg), //mao/mri
        21 => (String::from("Tschüss"), Colors::RedFg), //deu/ger
        22 => (String::from("ਅਲਵਿਦਾ"), Colors::GreenFg), //pan
        23 => (String::from("пока-пока"), Colors::BrightBlueFg), //rus
        24 => (String::from("Ћао"), Colors::BlueFg), //scc
        25 => (String::from("kwaheri"), Colors::BrightCyanFg), //swa
        26 => (String::from("хайр хайр"), Colors::GreenFg), //tgk
        27 => (String::from("xayr"), Colors::CyanFg), //uzb
        28 => (String::from("бувай"), Colors::BrightYellowFg), //ukr
        29 => (String::from("Oant sjen"), Colors::BlueFg), //fry
        30 => (String::from("nyob zoo"), Colors::CyanFg), //hmn
        31 => (String::from("Ahoj"), Colors::BrightBlueFg), //ces/cze
        32 => (String::from("hejdå"), Colors::BrightYellowFg), //sve/swe
        33 => (String::from("chisarai"), Colors::GreenFg), //sna
        34 => (String::from("ĝis revido"), Colors::BrightGreenFg), //epo
        35 => (String::from("その間"), Colors::BrightRedFg), //jpn
        _ => (String::from("numbers not found in the database of values"), Colors::RedBg),
    }
}



#[doc(hidden)]
fn n() -> i8 {
    rand::thread_rng().gen_range(0..36)
}