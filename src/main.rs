use ariadne::{Color, Label, Report, ReportKind, Source};
use camino::{Utf8Path, Utf8PathBuf};
//use std::env;
use std::fs;

fn main() {
    let file_path = "/tmp/hello.graphql".to_string();
    if Utf8Path::exists(&Utf8PathBuf::from(file_path.clone())) {
        let contents = fs::read_to_string(file_path.clone());
        //println!("{}", contents.ok().unwrap());

        let report = Report::build(ReportKind::Error, file_path.clone(), 0)
                    .with_message("This is an error")
                    .with_label(
                        Label::new((file_path.clone(), 5..10))
                            .with_message("This is an error")
                            .with_color(Color::Magenta),
                    )
                    .finish();

        let _ = report.print(
            (file_path, Source::from(contents.ok().unwrap())),
        );
    } else {
        println!("Invalid path. No file found at {}",
            file_path
        );
        return;
    }
}
