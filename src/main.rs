use lightningcss::{
    stylesheet::{MinifyOptions, ParserFlags, StyleSheet},
    targets::Browsers,
};
use std::path::Path;

use lightningcss::{
    bundler::{self, Bundler, FileProvider},
    stylesheet::{ParserOptions, PrinterOptions},
};

fn main() {
    println!("Hello, world!");

    let targets = Browsers {
        safari: Some((13 << 16) | (2 << 8)),
        ..Browsers::default()
    };

    let parser_flags = ParserFlags::NESTING;
    let fs = FileProvider::new();
    let opt1 = ParserOptions {
        flags: parser_flags,
        ..ParserOptions::default()
    };
    let mut bundler = Bundler::new(&fs, None, opt1);
    let stylesheet = bundler.bundle(Path::new("css/style.css")).unwrap();
    let opt = PrinterOptions {
        minify: false,
        ..PrinterOptions::default()
    };
    let styles = stylesheet.to_css(opt);
    let x = &styles.as_ref().unwrap().code;
    {
        let mut stylesheet = StyleSheet::parse(x, ParserOptions::default()).unwrap();
        stylesheet
            .minify(MinifyOptions {
                targets: targets.into(),
                ..MinifyOptions::default()
            })
            .unwrap();
        let res = stylesheet
            .to_css(PrinterOptions {
                targets: targets.into(),
                ..PrinterOptions::default()
            })
            .unwrap();
        println!("{}", res.code)
    }
    // let mut stylesheet = StyleSheet::parse(&source, ParserOptions::default()).unwrap();
}
