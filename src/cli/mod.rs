//! # CLI モジュール
//!
//! このモジュールは、lalaエディタのコマンドライン引数をパースし、
//! 起動モード（StartupMode）を決定する責務を持つ。
//!
//! ## 主な機能
//! - コマンドライン引数の解析
//! - ヘルプメッセージの表示 (`-h` / `--help`)
//! - バージョン情報の表示 (`-v` / `--version`)
//! - ファイルパス、ディレクトリパス、または空のエディタ起動の判定

use clap::Parser;
use std::path::PathBuf;

#[cfg(test)]
mod tests;

/// lalaエディタの起動モードを表すenum
#[derive(Debug, PartialEq, Clone)]
pub enum StartupMode {
    /// 指定されたファイルを開く
    OpenFile(PathBuf),
    /// 指定されたディレクトリをルートとしてファイルツリーを表示する
    OpenDir(PathBuf),
    /// 空のエディタウィンドウを開く
    Empty,
}

/// lalaエディタのコマンドライン引数
#[derive(Parser, Debug)]
#[command(name = "lala")]
#[command(author = "lala team")]
#[command(version = "0.1.0")]
#[command(about = "軽量で高速なテキストエディタ", long_about = None)]
#[command(disable_version_flag = true)]
struct Args {
    /// 開くファイルまたはディレクトリのパス
    #[arg(value_name = "PATH")]
    path: Option<PathBuf>,

    /// バージョン情報を表示
    #[arg(short = 'v', long = "version", action = clap::ArgAction::Version)]
    _version: Option<bool>,
}

/// コマンドライン引数をパースして、StartupModeを返す
///
/// # 引数
/// * `args` - パースするコマンドライン引数のイテレータ（通常は `std::env::args()`）
///
/// # 戻り値
/// パースされた起動モード
///
/// # 例
/// ```
/// use lala::cli::parse_args;
///
/// let mode = parse_args(vec!["lala", "file.txt"]);
/// ```
pub fn parse_args<I, T>(args: I) -> StartupMode
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let args = Args::parse_from(args);

    match args.path {
        Some(path) => {
            // パスが存在するかどうかのチェックはcore-engineの責務なので、
            // ここでは単純にパスの種類を判定する
            // 将来的にファイルが存在する場合は、その種類（ファイル/ディレクトリ）を確認する
            // 現時点では、パスの文字列から推測する（拡張子があればファイル、なければディレクトリ）
            // ただし、この判定は完全ではないため、実際のファイルシステムチェックは
            // core-engineで行う

            // シンプルな判定: パスを文字列に変換して、拡張子があるかチェック
            if path.extension().is_some() {
                StartupMode::OpenFile(path)
            } else if path.is_dir() {
                StartupMode::OpenDir(path)
            } else if path.is_file() {
                StartupMode::OpenFile(path)
            } else {
                // パスが存在しない場合、拡張子があればファイル、なければディレクトリと推定
                if path.extension().is_some() {
                    StartupMode::OpenFile(path)
                } else {
                    StartupMode::OpenDir(path)
                }
            }
        }
        None => StartupMode::Empty,
    }
}

/// デフォルトのコマンドライン引数（std::env::args()）をパースする
///
/// # 戻り値
/// パースされた起動モード
pub fn parse_args_default() -> StartupMode {
    parse_args(std::env::args())
}
