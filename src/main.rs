use std::io::prelude::*;
use std::process::Command;
fn main() {
    let mut npm = Command::new("npm");
    let mut npm1 = Command::new("npm");
    let mut npx = Command::new("npx");
    let mut git = Command::new("git");
    let mut git_init = git.arg("init").spawn().unwrap();
    git_init.wait().unwrap();

    let gts = npx.arg("gts").arg("init").arg("-y");
    let jest = npm.arg("install").arg("--save-dev").arg("jest");
    let babel = npm1
        .arg("install")
        .arg("--save-dev")
        .arg("babel-jest")
        .arg("@babel/core")
        .arg("@babel/preset-env")
        .arg("@babel/preset-typescript");

    let mut init_gts = gts.spawn().unwrap();
    init_gts.wait().unwrap();
    let mut init_jest = jest.spawn().unwrap();
    init_jest.wait().unwrap();
    let mut init_babel = babel.spawn().unwrap();
    init_babel.wait().unwrap();

    write_files();
}

fn show_message(bytes_written: usize, filename: &str) {
    println!("written {bytes_written} bytes to {filename}.");
}
fn write_files() {
    let gitignore_contents = "node_modules/\nbuild/";
    let write_gitignore = std::fs::File::create(".gitignore")
        .unwrap()
        .write(gitignore_contents.as_bytes())
        .unwrap();
    show_message(write_gitignore, ".gitignore");

    let babel_contents = "module.exports = {
      presets: [
        ['@babel/preset-env', {targets: {node: 'current'}}],
        '@babel/preset-typescript',
      ],
    };";
    let write_babel = std::fs::File::create("babel.config.js")
        .unwrap()
        .write(babel_contents.as_bytes())
        .unwrap();
    show_message(write_babel, "babel.config.js");

    let gitattributes_content = "src/* linguist-vendored=false";
    let write_gitattributes = std::fs::File::create(".gitattributes")
        .unwrap()
        .write(gitattributes_content.as_bytes())
        .unwrap();
    show_message(write_gitattributes, ".gitattributes");
}
