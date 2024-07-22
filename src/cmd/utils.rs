use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use ansi_term::ANSIGenericString;
use ansi_term::Color::{Red, Yellow};
use walkdir::WalkDir;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

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

fn get_path_detail(path: &PathBuf, size: Datasize, quota_size: &Option<Datasize>, origin: Option<&PathBuf>) -> (String, f64, String, String) {
    let (s, u) = size.with_unit_string(100.0, FORMAT_BIN);
    let display_path = if let Some(o) = origin {
        path.strip_prefix(o).unwrap().to_str().unwrap()
    } else { path.to_str().unwrap() };
    (display_path.to_string(), s, u, if let Some(q) = quota_size {
        round!((size.value as f64) / (q.value as f64) * 100.0, 3 ,f64).to_string() + "%"
    } else { String::new() })
}
fn format_path_detail(path: &PathBuf, size: Datasize, quota_size: &Option<Datasize>, origin: Option<&PathBuf>) -> String {
    let (display_path, s, u, usage) = get_path_detail(path, size, quota_size, origin);
    let mark = if path.is_dir() {
        "dir"
    } else if path.is_file() {
        "file"
    } else if path.is_symlink() {
        "sym"
    } else { "?" };

    format!("{:<5}{:<15} : {:>7}{:<3} {:>7}", mark, display_path, s, u, usage)
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
        let a = format_path_detail(path, Datasize::new(*size), quota_size, Some(origin));
        let output = match size_vec.len() - i - 1 {
            0 => Red.bold().paint(a),
            1 => Yellow.bold().paint(a),
            _ => ANSIGenericString::from(a)
        };
        println!("{}", output)
    }


    println!("Total:\n{}", format_path_detail(origin, Datasize::new(total_size), quota_size, None));
    size_vec = size_vec.into_iter().filter(|(p, _)| {
        p.is_dir()
    }).collect();
    if !size_vec.is_empty() {
        size_vec.reverse();

        let theme = ColorfulTheme::default();
        let mut selection_builder = FuzzySelect::with_theme(&theme)
            .with_prompt("更に探索するディレクトリを選択してください")
            .item("quit (終了)")
            .default(0)
            .max_length(5);
        for (p, s) in size_vec.clone() {
            let a = get_path_detail(p, Datasize::new(*s), quota_size, Some(origin));
            selection_builder = selection_builder.clone().item(format!("{} {}{} {}", a.0, a.1, a.2, a.3))
        }
        let result = selection_builder.interact_opt();
        match result {
            Ok(Some(i)) => {
                if i == 0 {
                    return;
                }
                size_cmd(ctx, size_vec[i - 1].0, include_symlink, quota_size);
            }
            _ => {}
        }
    }

}



