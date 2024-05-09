use lightningcss::{
    bundler::{Bundler, FileProvider},
    stylesheet::{MinifyOptions, ParserFlags, ParserOptions, PrinterOptions, StyleSheet},
    targets::Browsers,
};
use std::{env, fs::File};
use std::{io::Write, path::Path};

fn transform(file_input: &String, file_output: &String) {
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
    let style_sheet = bundler.bundle(Path::new(file_input)).unwrap();
    let opt = PrinterOptions {
        minify: false,
        ..PrinterOptions::default()
    };
    let styles = style_sheet.to_css(opt);
    let x = &styles.as_ref().unwrap().code;
    {
        let mut style_sheet = StyleSheet::parse(x, ParserOptions::default()).unwrap();
        style_sheet
            .minify(MinifyOptions {
                targets: targets.into(),
                ..MinifyOptions::default()
            })
            .unwrap();
        let res = style_sheet
            .to_css(PrinterOptions {
                targets: targets.into(),
                ..PrinterOptions::default()
            })
            .unwrap();
        let mut file = File::create(Path::new(file_output)).unwrap();
        file.write_all(res.code.as_bytes()).unwrap();
    }
    // let mut style_sheet = StyleSheet::parse(&source, ParserOptions::default()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_input = &args[1];
    let file_output = &args[2];
    transform(file_input, file_output)
}

#[cfg(test)]
mod tests {
    use crate::transform;
    use std::fs;

    #[test]
    fn it_works() {
        match fs::create_dir("tmp") {
            Ok(_) => {}
            Err(_) => {}
        }
        let file_input = String::from("css/style.css");
        let file_output = String::from("tmp/out.css");
        transform(&file_input, &file_output);
    }
}
