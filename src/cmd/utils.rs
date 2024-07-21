use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use ansi_term::{ANSIGenericString, Color};
use ansi_term::Color::{Red, White, Yellow};
use inquire::Confirm;
use inquire::error::InquireResult;
use serde::__private::de::Content::F64;
use walkdir::WalkDir;
use crate::argument_parser::Argument;
use crate::datasize::{Datasize, FORMAT_BIN};

fn is_exists_file(val: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(val);
    match path.try_exists() {
        Ok(true) => Ok(path),
        Ok(false) => Err(String::from("存在しないファイルです")),
        Err(_) => Err(String::from("アクセスできないファイルです"))
    }
}

fn is_datasize(val: &str) -> Result<Datasize, String> {
    Datasize::try_from(val).map_err(|_| String::from("不正なデータサイズの形式です"))
}

#[derive(clap::Subcommand, Debug)]
pub enum UtilsCommand {
    Size {
        /// 対象のディレクトリ
        #[arg(
            value_parser = is_exists_file,
            value_hint = clap::ValueHint::DirPath,
            default_value_os_t = dirs::home_dir().unwrap_or(PathBuf::from("/")))
        ]
        path: PathBuf,

        /// シンボリックリンクもカウントする
        #[arg(long)]
        include_symlink: bool,
        #[arg(
            short = 's', long,
            value_parser = is_datasize
        )]
        quota_size: Option<Datasize>,
    },
    Subsub2,
}
pub fn utils_cmd(ctx: &Argument, cmd: &UtilsCommand) {
    match cmd {
        UtilsCommand::Size { path, include_symlink, quota_size } => {
            size_cmd(ctx, path, *include_symlink, quota_size)
        }
        UtilsCommand::Subsub2 => {}
    }
}

macro_rules! round {
    ($value:expr,$n_digits:expr,$t:ty) => {
        (((((10 as i32).pow($n_digits) as $t) * $value as $t).round() as $t) /((10 as i32).pow($n_digits) as $t))
    };
}

fn format_path_detail(path: &PathBuf, size: Datasize, quota_size: &Option<Datasize>, origin: Option<&PathBuf>) -> String {
    let (s, u) = size.with_unit_string(100.0, FORMAT_BIN);
    let mark = if path.is_dir() {
        "dir"
    } else if path.is_file() {
        "file"
    } else if path.is_symlink() {
        "sym"
    } else { "?" };
    let display_path = if let Some(o) = origin {
        path.strip_prefix(o).unwrap().display()
    } else { path.display() };
    format!("{:<5}{:<15}: {:>7}{:<3} {:>6}%", mark, display_path, s, u, if let Some(q) = quota_size {
        round!((size.value as f64) / (q.value as f64) * 100.0, 3 ,f64).to_string()
    } else { String::new() })
}
fn size_cmd(ctx: &Argument, origin: &PathBuf, include_symlink: bool, quota_size: &Option<Datasize>) {
    //let mut size: u64 = 0;
    let mut size_map: HashMap<PathBuf, u64> = HashMap::new();
    for entry in WalkDir::new(origin).follow_links(include_symlink).into_iter().filter_map(
        |e| e.ok())
    {
        let target = entry.path().display();
        match entry.metadata() {
            Ok(m) => {
                if entry.depth() == 1 {
                    size_map.insert(entry.clone().into_path(), m.size());
                } else {
                    if let Some(parent) = size_map.clone().iter().find(|(p, _)| {
                        entry.clone().into_path().starts_with(p)
                    }) {
                        *size_map.get_mut(parent.0).unwrap() += m.size()
                    }
                }

                print!("\r\x1b[0K{}: {}", target, m.size());
            }
            Err(_) => {
                println!("{}のサイズを取得できませんでした", target)
            }
        }
    }
    print!("\r\x1b[0K");
    let mut total_size: u64 = 0;
    let mut size_vec: Vec<(&PathBuf, &u64)> = size_map.iter().map(|e| e).collect();
    size_vec.sort_by(|&a, &b| a.1.cmp(b.1));

    for (i, &(path, size)) in size_vec.iter().enumerate() {
        total_size += size;
        let a = format_path_detail(path, Datasize::new(*size), quota_size,Some(origin));
        let output = match size_vec.len() - i - 1 {
            0 => Red.bold().paint(a),
            1 => Yellow.bold().paint(a),
            _ => ANSIGenericString::from(a)
        };
        println!("{}", output)
    }


    println!("Total:\n{}", format_path_detail(origin, Datasize::new(total_size), quota_size,None));
    let (largest_path, _) = size_vec.last().unwrap();
    if largest_path.is_dir() {
        let ans = Confirm::new(
            format!("{} 内を更に探索しますか？ (Would you like to explore further inside {} ?)",
                    largest_path.display(), largest_path.display()
            ).as_str()
        )
            .with_default(false)
            .with_help_message("yで続行、nで終了")
            .prompt();
        match ans {
            Ok(true) => {
                size_cmd(ctx, largest_path, include_symlink, quota_size);
            }
            _ => {}
        }
    }
}



