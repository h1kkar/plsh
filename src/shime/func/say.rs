use rand::Rng;
use ansi_colors::*;

pub fn bye() -> String {
    match n() {
        0 => {
            let mut eng = ColouredStr::new("Goodbye");
            eng.blue();
            eng.to_string()
        }, //eng
        1 => {
            let mut ara = ColouredStr::new("مع السلامة");
            ara.green();
            ara.to_string()
            }, //ara
        2 => {
            let mut afr = ColouredStr::new("Totsiens");
            afr.yellow();
            afr.to_string()
        }, //afr
        3 => {
            let mut bel = ColouredStr::new("Да пабачэння");
            bel.red();
            bel.to_string()
        }, //bel
        4 => {
            let mut bul = ColouredStr::new("Довиждане");
            bul.light_green();
            bul.to_string()
        }, //bul
        5 => {
            let mut cym = ColouredStr::new("Hwyl fawr");
            cym.cyan();
            cym.to_string()
        }, //cym/wel
        6 => {
            let mut vie = ColouredStr::new("Tạm biệt");
            vie.red();
            vie.to_string()
        }, //vie
        7 => {
            let mut haw = ColouredStr::new("Aloha");
            haw.blue();
            haw.to_string()
        }, //haw
        8 => {
            let mut gre = ColouredStr::new("Αντιο σας");
            gre.light_blue();
            gre.to_string()
        }, //ell/gre
        9 => {
            let mut geo = ColouredStr::new("ნახვამდის");
            geo.red();
            geo.to_string()
        }, //geo/kat
        10 => {
            let mut zul = ColouredStr::new("Hamba kahle");
            zul.light_yellow();
            zul.to_string()
        }, //zul
        11 => {
            let mut ibo = ColouredStr::new("Ka ọ dị");
            ibo.light_green();
            ibo.to_string()
        }, //ibo
        12 => {
            let mut ind = ColouredStr::new("Selamat tinggal");
            ind.red();
            ind.to_string()
        }, //ind
        13 => {
            let mut yor = ColouredStr::new("O dabọ");
            yor.yellow();
            yor.to_string()
        }, //yor
        14 => {
            let mut kaz = ColouredStr::new("Сау болыңыз");
            kaz.cyan();
            kaz.to_string()
        }, //kaz
        15 => {
            let mut chi = ColouredStr::new("再见");
            chi.red();
            chi.to_string()
        }, //chi/zho
        16 => {
            let mut kor = ColouredStr::new("안녕히 가세요");
            kor.light_blue();
            kor.to_string()
        }, //kor
        17 => {
            let mut khm = ColouredStr::new("លាហើយ");
            khm.blue();
            khm.to_string()
        }, //khm
        18 => {
            let mut lit = ColouredStr::new("Viso gero");
            lit.light_yellow();
            lit.to_string()
        }, //lit
        19 => {
            let mut mal = ColouredStr::new("വിട");
            mal.light_yellow();
            mal.to_string()
        }, //mal
        20 => {
            let mut mao = ColouredStr::new("Kia ora");
            mao.blue();
            mao.to_string()
        }, //mao/mri
        21 => {
            let mut ger = ColouredStr::new("Auf Wiedersehen");
            ger.red();
            ger.to_string()
        }, //deu/ger
        22 => {
            let mut pan = ColouredStr::new("ਅਲਵਿਦਾ");
            pan.green();
            pan.to_string()
        }, //pan
        23 => {
            let mut rus = ColouredStr::new("До свидания");
            rus.light_blue();
            rus.to_string()
        }, //rus
        24 => {
            let mut scc = ColouredStr::new("збогом");
            scc.blue();
            scc.to_string()
        }, //scc
        25 => {
            let mut swa = ColouredStr::new("Kwaheri");
            swa.light_cyan();
            swa.to_string()
        }, //swa
        26 => {
            let mut tgk = ColouredStr::new("Хайр");
            tgk.green();
            tgk.to_string()
        }, //tgk
        27 => {
            let mut uzb = ColouredStr::new("Xayr. Salomat bo'ling");
            uzb.cyan();
            uzb.to_string()
        }, //uzb
        28 => {
            let mut ukr = ColouredStr::new("До побачення");
            ukr.light_yellow();
            ukr.to_string()
        }, //ukr
        29 => {
            let mut fry = ColouredStr::new("Oant sjen");
            fry.blue();
            fry.to_string()
        }, //fry
        30 => {
            let mut hmn = ColouredStr::new("Nyob zoo");
            hmn.cyan();
            hmn.to_string()
        }, //hmn
        31 => {
            let mut cze = ColouredStr::new("Ahoj");
            cze.light_blue();
            cze.to_string()
        }, //ces/cze
        32 => {
            let mut swe = ColouredStr::new("Adjö");
            swe.light_yellow();
            swe.to_string()
        }, //sve/swe
        33 => {
            let mut sna = ColouredStr::new("Sara mushe");
            sna.green();
            sna.to_string()
        }, //sna
        34 => {
            let mut epo = ColouredStr::new("Adiaŭ");
            epo.light_green();
            epo.to_string()
        }, //epo
        35 => {
            let mut jpn = ColouredStr::new("その間");
            jpn.light_red();
            jpn.to_string()
        }, //jpn
        _ => {
            let mut err = ColouredStr::new("numbers not found in the database of values");
            err.back_red();
            err.to_string()
        }
    }
}

pub fn hi() -> String {
    match n() {
        0 => {
            let mut eng = ColouredStr::new("Hello");
            eng.blue();
            eng.to_string()
        }, //eng
        1 => {
            let mut ara = ColouredStr::new("مرحبًا");
            ara.green();
            ara.to_string()
            }, //ara
        2 => {
            let mut afr = ColouredStr::new("Hallo");
            afr.yellow();
            afr.to_string()
        }, //afr
        3 => {
            let mut bel = ColouredStr::new("Прывітанне");
            bel.red();
            bel.to_string()
        }, //bel
        4 => {
            let mut bul = ColouredStr::new("Здравейте");
            bul.light_green();
            bul.to_string()
        }, //bul
        5 => {
            let mut cym = ColouredStr::new("Helo");
            cym.cyan();
            cym.to_string()
        }, //cym/wel
        6 => {
            let mut vie = ColouredStr::new("Xin chào");
            vie.red();
            vie.to_string()
        }, //vie
        7 => {
            let mut haw = ColouredStr::new("Aloha");
            haw.blue();
            haw.to_string()
        }, //haw
        8 => {
            let mut gre = ColouredStr::new("Γειά σου");
            gre.light_blue();
            gre.to_string()
        }, //ell/gre
        9 => {
            let mut geo = ColouredStr::new("გამარჯობა");
            geo.red();
            geo.to_string()
        }, //geo/kat
        10 => {
            let mut zul = ColouredStr::new("Sawubona");
            zul.light_yellow();
            zul.to_string()
        }, //zul
        11 => {
            let mut ibo = ColouredStr::new("Nnọọ");
            ibo.light_green();
            ibo.to_string()
        }, //ibo
        12 => {
            let mut ind = ColouredStr::new("Halo");
            ind.red();
            ind.to_string()
        }, //ind
        13 => {
            let mut yor = ColouredStr::new("Pẹlẹ o");
            yor.yellow();
            yor.to_string()
        }, //yor
        14 => {
            let mut kaz = ColouredStr::new("Сәлеметсіз бе");
            kaz.cyan();
            kaz.to_string()
        }, //kaz
        15 => {
            let mut chi = ColouredStr::new("你好");
            chi.red();
            chi.to_string()
        }, //chi/zho
        16 => {
            let mut kor = ColouredStr::new("안녕하세요");
            kor.light_blue();
            kor.to_string()
        }, //kor
        17 => {
            let mut khm = ColouredStr::new("ជំរាបសួរ");
            khm.blue();
            khm.to_string()
        }, //khm
        18 => {
            let mut lit = ColouredStr::new("Sveiki");
            lit.light_yellow();
            lit.to_string()
        }, //lit
        19 => {
            let mut mal = ColouredStr::new("ഹലോ");
            mal.light_yellow();
            mal.to_string()
        }, //mal
        20 => {
            let mut mao = ColouredStr::new("Kia ora");
            mao.blue();
            mao.to_string()
        }, //mao/mri
        21 => {
            let mut ger = ColouredStr::new("Hallo");
            ger.red();
            ger.to_string()
        }, //deu/ger
        22 => {
            let mut pan = ColouredStr::new("ਸਤ ਸ੍ਰੀ ਅਕਾਲ");
            pan.green();
            pan.to_string()
        }, //pan
        23 => {
            let mut rus = ColouredStr::new("Привет");
            rus.light_blue();
            rus.to_string()
        }, //rus
        24 => {
            let mut scc = ColouredStr::new("Здраво");
            scc.blue();
            scc.to_string()
        }, //scc
        25 => {
            let mut swa = ColouredStr::new("Habari");
            swa.light_cyan();
            swa.to_string()
        }, //swa
        26 => {
            let mut tgk = ColouredStr::new("Салом");
            tgk.green();
            tgk.to_string()
        }, //tgk
        27 => {
            let mut uzb = ColouredStr::new("Salom");
            uzb.cyan();
            uzb.to_string()
        }, //uzb
        28 => {
            let mut ukr = ColouredStr::new("Привіт");
            ukr.light_yellow();
            ukr.to_string()
        }, //ukr
        29 => {
            let mut fry = ColouredStr::new("Hallo");
            fry.blue();
            fry.to_string()
        }, //fry
        30 => {
            let mut hmn = ColouredStr::new("Nyob zoo");
            hmn.cyan();
            hmn.to_string()
        }, //hmn
        31 => {
            let mut cze = ColouredStr::new("Ahoj");
            cze.light_blue();
            cze.to_string()
        }, //ces/cze
        32 => {
            let mut swe = ColouredStr::new("Hallå");
            swe.light_yellow();
            swe.to_string()
        }, //sve/swe
        33 => {
            let mut sna = ColouredStr::new("Mhoro");
            sna.green();
            sna.to_string()
        }, //sna
        34 => {
            let mut epo = ColouredStr::new("Saluton");
            epo.light_green();
            epo.to_string()
        }, //epo
        35 => {
            let mut jpn = ColouredStr::new("こんにちは");
            jpn.light_red();
            jpn.to_string()
        }, //jpn
        _ => {
            let mut err = ColouredStr::new("numbers not found in the database of values");
            err.back_red();
            err.to_string()
        }
    }
}

fn n() -> i8 {
    rand::thread_rng().gen_range(0..36)
}