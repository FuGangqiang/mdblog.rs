use std::error::Error as StdError;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use log::error;
use pulldown_cmark::{html, Options, Parser};

use crate::error::Result;

/// create the file of `path` and append content
///
/// if parent of `path` does not existed, create it first.
pub fn write_file(path: &Path, buf: &[u8]) -> Result<()> {
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p)?;
    }
    let mut file = File::create(path)?;
    file.write_all(buf)?;
    Ok(())
}

/// read the file content of `path` to `buf`
pub fn read_file<P: AsRef<Path>>(path: P, buf: &mut Vec<u8>) -> Result<()> {
    let mut f = File::open(path.as_ref())?;
    f.read_to_end(buf)?;
    Ok(())
}

/// the rendered html content of post body port
pub fn markdown_to_html(content: &str) -> String {
    let mut katexed = replace_delimiters(content, "$$", false);
    katexed = replace_delimiters(&katexed, "$", true);

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    let mut s = String::with_capacity(katexed.len() * 3 / 2);
    let p = Parser::new_ext(&katexed, opts);
    html::push_html(&mut s, p);
    s
}

// replace delimiters with html math markers
fn replace_delimiters(raw_content: &str, delimiters: &str, escape_backslash: bool) -> String {
    let mut replaced_content = String::new();
    let mut inside_delimiters = false;
    let pre_marker = if delimiters == "$" {
        "<language-inline-math>"
    } else {
        "<language-math>"
    };
    let post_marker = if delimiters == "$" {
        "</language-inline-math>"
    } else {
        "</language-math>"
    };
    for item in split(&raw_content, &delimiters, escape_backslash) {
        if inside_delimiters {
            replaced_content.push_str(&pre_marker);
            replaced_content.push_str(&item);
            replaced_content.push_str(&post_marker);
        } else {
            replaced_content.push_str(&item)
        }
        inside_delimiters = !inside_delimiters;
    }
    replaced_content
}

/// https://github.com/lzanini/mdbook-katex
fn split(string: &str, separator: &str, escape_backslash: bool) -> Vec<String> {
    let mut result = Vec::new();
    let mut splits = string.split(separator);
    let mut current_split = splits.next();
    // iterate over splits
    while let Some(substring) = current_split {
        let mut result_split = String::from(substring);
        if escape_backslash {
            // while the current split ends with a backslash
            while let Some('\\') = current_split.unwrap().chars().last() {
                // removes the backslash, add the separator back, and add the next split
                result_split.pop();
                result_split.push_str(separator);
                current_split = splits.next();
                if let Some(split) = current_split {
                    result_split.push_str(split);
                }
            }
        }
        result.push(result_split);
        current_split = splits.next()
    }
    result
}

/// basic error reporting, including the "cause chain".
pub(crate) fn log_error_chain(mut e: &dyn StdError) {
    error!("error: {}", e);
    while let Some(source) = e.source() {
        error!("caused by: {}", source);
        e = source;
    }
}
