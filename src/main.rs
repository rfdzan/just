use std::collections::HashMap;
use std::io::{self, prelude::*};
use std::process::Command;
fn main() {
    match commands() {
        Err(e) => eprintln!("Error on executing command:\n{e}"),
        Ok(_) => (),
    }
}
fn commands() -> io::Result<()> {
    let mut npm = Command::new("npm");
    let mut npm1 = Command::new("npm");
    let mut npx = Command::new("npx");
    let mut git = Command::new("git");

    let git_init = git.arg("init");
    let gts = npx.arg("gts").arg("init").arg("-y");
    let jest = npm.arg("install").arg("--save-dev").arg("jest");
    let babel = npm1
        .arg("install")
        .arg("--save-dev")
        .arg("babel-jest")
        .arg("@babel/core")
        .arg("@babel/preset-env")
        .arg("@babel/preset-typescript");
    {
        //Can i parallelize this section?
        let mut dir_git = git_init.spawn()?;
        dir_git.wait()?;
        let mut init_gts = gts.spawn()?;
        init_gts.wait()?;
        let mut init_jest = jest.spawn()?;
        init_jest.wait()?;
        let mut init_babel = babel.spawn()?;
        init_babel.wait()?;
    }
    write_file_content();
    Ok(())
}

fn show_message(bytes_written: usize, filename: &str) {
    println!("written {bytes_written} bytes to {filename}.");
}
fn write_file_content() {
    let mut filename_and_content = HashMap::new();
    filename_and_content.insert(".gitignore", "node_modules/\nbuild/");
    filename_and_content.insert(".gitattributes", "src/* linguist-vendored=false");
    filename_and_content.insert(
        "babel.config.js",
        "module.exports = {
      presets: [
        ['@babel/preset-env', {targets: {node: 'current'}}],
        '@babel/preset-typescript',
      ],
    };",
    );
    for (filename, content) in filename_and_content.iter() {
        match write_files(filename, content) {
            Err(e) => eprintln!("Error writing to file:\n{e}"),
            Ok(_) => (),
        }
    }
}
fn write_files(filename: &str, content: &str) -> io::Result<()> {
    let written = std::fs::File::create(filename)?.write(content.as_bytes())?;
    show_message(written, filename);
    Ok(())
}
