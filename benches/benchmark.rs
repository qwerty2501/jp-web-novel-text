use criterion::{Criterion, criterion_group, criterion_main};
use jp_web_novel_text::{
    DictionaryPhrase, DictionaryWord, NewLinePhrase, Parser, Phrase, PlainPhrase, RubyPhrase,
    WhiteSpacePhrase, WhiteSpaceType,
};

fn benchmark_words() -> Vec<DictionaryWord> {
    let mut re = vec![];
    let base = vec![
        DictionaryWord::new("若々".into(), "わかわか".into(), "わ".into()),
        DictionaryWord::new("茶屋".into(), "ちゃや".into(), "茶店".into()),
        DictionaryWord::new("自分".into(), "じぶん".into(), "自ら".into()),
        DictionaryWord::new("好奇心".into(), "こうきしん".into(), "こ".into()),
        DictionaryWord::new("旅館".into(), "りょかん".into(), "こ".into()),
        DictionaryWord::new("避暑地".into(), "ひしょかん".into(), "こ".into()),
        DictionaryWord::new("留守".into(), "ひしょかん".into(), "こ".into()),
        DictionaryWord::new("どうして".into(), "ひしょかん".into(), "こ".into()),
        DictionaryWord::new("様式".into(), "ひしょかん".into(), "こ".into()),
        DictionaryWord::new("眼".into(), "ひしょかん".into(), "こ".into()),
        DictionaryWord::new("墓参り".into(), "ひしょかん".into(), "こ".into()),
        DictionaryWord::new("交際".into(), "ひしょかん".into(), "こ".into()),
        DictionaryWord::new("問答".into(), "ひしょかん".into(), "こ".into()),
    ];
    re.extend_from_slice(&base);
    for i in 0..10 {
        for d in base.iter() {
            re.push(DictionaryWord::new(
                d.key().to_string() + &i.to_string(),
                d.ruby().to_string(),
                d.description().to_owned(),
            ));
        }
    }
    re
}

fn parse_kokoro(c: &mut Criterion) {
    let kokoro_body = include_str!("test_data/kokoro_utf8.txt");
    let words = benchmark_words();
    c.bench_function("parse_kokoro", |b| {
        b.iter(|| {
            let parser = Parser::try_new_with_dic(words.clone()).unwrap();
            for _ in parser.parse_iter(kokoro_body) {}
        });
    });
}

fn parse_kokoro_without_dictionary(c: &mut Criterion) {
    let kokoro_body = include_str!("test_data/kokoro_utf8.txt");
    c.bench_function("parse_kokoro_without_dictionary", |b| {
        b.iter(|| {
            let parser = Parser::default();
            for _ in parser.parse_iter(kokoro_body) {}
        });
    });
}

fn parse_kokoro_and_gen_html(c: &mut Criterion) {
    let kokoro_body = include_str!("test_data/kokoro_utf8.txt");
    let words = benchmark_words();
    c.bench_function("parse_kokoro_and_gen_html", |b| {
        b.iter(|| {
            let mut buf = String::with_capacity(kokoro_body.len() * 2);
            let parser = Parser::try_new_with_dic(words.clone()).unwrap();
            for flag in parser.parse_iter(kokoro_body) {
                match flag.phrase() {
                    Phrase::Plain(plain) => emit_plain(&mut buf, plain),
                    Phrase::Ruby(ruby) => emit_ruby(&mut buf, ruby),
                    Phrase::NewLine(nl) => emit_newline(&mut buf, nl),
                    Phrase::WhiteSpace(sp) => emit_space(&mut buf, sp),
                    Phrase::DictionaryWord(dw) => emit_dictionary_word(&mut buf, dw),
                }
            }
        });
    });
}

fn emit_dictionary_word(buf: &mut String, dw: &DictionaryPhrase<&str, &DictionaryWord>) {
    buf.push_str("<ruby>");
    buf.push_str(dw.target());
    buf.push_str("<rp>(</rp><rt>");
    buf.push_str(dw.word().ruby().to_string().as_ref());
    buf.push_str("</rt><rp>)</rp>");
    buf.push_str("</ruby>");
}

fn emit_space(buf: &mut String, phrase: &WhiteSpacePhrase) {
    buf.push_str("<pre>");
    for _ in 0..*phrase.count() {
        match phrase.white_space_type() {
            WhiteSpaceType::Space => buf.push(' '),
            WhiteSpaceType::ZenkakuSpace => buf.push('　'),
            WhiteSpaceType::Tab => buf.push('\t'),
        }
    }
    buf.push_str("</pre>");
}

fn emit_newline(buf: &mut String, _: &NewLinePhrase) {
    buf.push_str("<br/>");
}

fn emit_plain(buf: &mut String, phrase: &PlainPhrase<&str>) {
    buf.push_str("<span>");
    buf.push_str(phrase.target());
    buf.push_str("</span>");
}
fn emit_ruby(buf: &mut String, ruby: &RubyPhrase<&str>) {
    buf.push_str("<ruby>");
    buf.push_str(ruby.target());
    buf.push_str("<rp>(</rp><rt>");
    buf.push_str(ruby.ruby());
    buf.push_str("</rt><rp>)</rp>");
    buf.push_str("</ruby>");
}

criterion_group!(
    benches,
    parse_kokoro,
    parse_kokoro_and_gen_html,
    parse_kokoro_without_dictionary
);
criterion_main!(benches);
