use std::{env, fs::File, io::Write};

use jp_web_novel_text::{
    DictionaryPhrase, DictionaryWord, NewLinePhrase, Parser, Phrase, PlainPhrase, RubyPhrase,
    WhiteSpacePhrase, WhiteSpaceType,
};

fn words() -> Vec<DictionaryWord> {
    vec![
        DictionaryWord::new("若々しい".into(), "".into(), "年齢のわりに若く見えること、また、活力や気力が衰えていないこと".into()),
        DictionaryWord::new("茶屋".into(), "ちゃや".into(), "製した茶を売る店。茶舗。".into()),
        DictionaryWord::new("自分".into(), "じぶん".into(), "その人自身。".into()),
        DictionaryWord::new("好奇心".into(), "こうきしん".into(), "新しいことや未知のこと、珍しい物事に対して強い興味や関心を持ち、それを知りたい、体験したいと思う心".into()),
        DictionaryWord::new(
            "旅館".into(),
            "りょかん".into(),
            "旅行者を泊めることを業とする家。やどや。".into(),
        ),
        DictionaryWord::new(
            "避暑地".into(),
            "ひしょち".into(),
            "夏の暑さを避けるために訪れる、涼しい気候の土地".into(),
        ),
        DictionaryWord::new("留守".into(), "るす".into(), "外出して家にいないこと。".into()),
        DictionaryWord::new(
            "どうして".into(),
            "".into(),
            "どのようなわけで。なぜ。".into(),
        ),
        DictionaryWord::new("様式".into(), "ようしき".into(), "ある範囲の事物・しかたに共通に認められる、一定のありかた。".into()),
        DictionaryWord::new("眼".into(), "め".into(), "物を見るはたらきをする器官。め。".into()),
        DictionaryWord::new("墓参り".into(), "はかまい".into(), "（自分の家や縁故者の）墓にお参りすること。".into()),
        DictionaryWord::new("交際".into(), "こうさい".into(), "人間（国）どうしがつきあうこと。つきあい。".into()),
        DictionaryWord::new("問答".into(), "もんどう".into(), "問うことと答えること。また、問い答えのやりとり。".into()),
    ]
}

fn main() {
    let kokoro_body = include_str!("../benches/test_data/kokoro_utf8.txt");

    let mut buf = String::with_capacity(kokoro_body.len() * 2);
    buf.push_str(
        "<!doctype html>
<html lang=\"ja\">
<body>
<span>
<style>

.c-tooltip {
	display: inline-block;
	position: relative;
}

.c-tooltip::before,
.c-tooltip::after {
	-webkit-transition: all 0.2s;
	opacity: 0;
	position: absolute;
	transition: all 0.2s;
	visibility: hidden;
	z-index: 11;
}

.c-tooltip::before {
	-webkit-transform: translateY(-50%);
	border-color: transparent #5f6368 transparent transparent;
	border-style: solid;
	border-width: 3px 7px 3px 0;
	content: \"\";
	height: 0;
	left: calc(100% + 6px);
	top: 50%;
	transform: translateY(-50%);
	width: 0;
}

.c-tooltip::after {
	-webkit-transform: translateY(-50%);
	background: #5f6368;
	border-radius: 3px;
	color: #fff;
	content: attr(data-tooltip); 
	display: block;
	font-size: 11px;
	font-weight: 600;
	left: calc(100% + 13px);
	padding: 5px 10px;
	top: 50%;
	transform: translateY(-50%);
	white-space: nowrap;
}

.c-tooltip:hover {
	cursor: pointer;
}

.c-tooltip:hover::before {
	opacity: 1;
	visibility: visible;
}

.c-tooltip:hover::after {
	opacity: 1;
	visibility: visible;
}
</style>",
    );
    let parser = Parser::try_new_with_dic(words()).unwrap();
    for flag in parser.parse_iter(kokoro_body) {
        match flag.phrase() {
            Phrase::Plain(plain) => emit_plain(&mut buf, plain),
            Phrase::Ruby(ruby) => emit_ruby(&mut buf, ruby),
            Phrase::NewLine(nl) => emit_newline(&mut buf, nl),
            Phrase::WhiteSpace(sp) => emit_space(&mut buf, sp),
            Phrase::DictionaryWord(dw) => emit_dictionary_word(&mut buf, dw),
        }
    }
    buf.push_str("</span></body></html>");
    let current_dir = env::current_dir().unwrap();
    let output_path = current_dir.join("kokoro_utf8.html");
    let mut output_file = File::create(output_path).unwrap();
    output_file.write_all(buf.as_bytes()).unwrap();
}

fn emit_dictionary_word(buf: &mut String, dw: &DictionaryPhrase<&str, &DictionaryWord>) {
    buf.push_str("<span style=\"color:#0000FF\" class=\"c-tooltip\" data-tooltip=\"");
    buf.push_str(dw.word().description());
    buf.push_str("\">");
    if !dw.word().ruby().is_empty() {
        buf.push_str("<ruby>");
        buf.push_str(dw.target());
        buf.push_str("<rp>(</rp><rt>");
        buf.push_str(dw.word().ruby().to_string().as_ref());
        buf.push_str("</rt><rp>)</rp>");
        buf.push_str("</ruby>");
    } else {
        buf.push_str(dw.target());
    }
    buf.push_str("</span>");
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
