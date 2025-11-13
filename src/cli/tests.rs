use super::*;
use std::path::PathBuf;

#[test]
fn test_parse_args_with_file() {
    // ファイルパスを指定した場合、OpenFileを返す
    let mode = parse_args(vec!["lala", "a.md"]);
    assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("a.md")));
}

#[test]
fn test_parse_args_with_directory() {
    // ディレクトリパスを指定した場合、OpenDirを返す
    let mode = parse_args(vec!["lala", "./src"]);
    assert_eq!(mode, StartupMode::OpenDir(PathBuf::from("./src")));
}

#[test]
fn test_parse_args_with_current_directory() {
    // カレントディレクトリ（.）を指定した場合、OpenDirを返す
    let mode = parse_args(vec!["lala", "."]);
    assert_eq!(mode, StartupMode::OpenDir(PathBuf::from(".")));
}

#[test]
fn test_parse_args_with_home_directory() {
    // ホームディレクトリ（~/）を指定した場合、OpenDirを返す
    let mode = parse_args(vec!["lala", "~/"]);
    assert_eq!(mode, StartupMode::OpenDir(PathBuf::from("~/")));
}

#[test]
fn test_parse_args_empty() {
    // 引数なしの場合、Emptyを返す
    let mode = parse_args(vec!["lala"]);
    assert_eq!(mode, StartupMode::Empty);
}

#[test]
fn test_parse_args_with_rust_file() {
    // .rs ファイルの場合、OpenFileを返す
    let mode = parse_args(vec!["lala", "main.rs"]);
    assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("main.rs")));
}

#[test]
fn test_parse_args_with_path_separator() {
    // パス区切り文字を含むファイルパスの場合、OpenFileを返す
    let mode = parse_args(vec!["lala", "src/main.rs"]);
    assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("src/main.rs")));
}

#[test]
fn test_parse_args_with_absolute_path() {
    // 絶対パスを指定した場合、拡張子があればOpenFile
    let mode = parse_args(vec!["lala", "/tmp/test.txt"]);
    assert_eq!(mode, StartupMode::OpenFile(PathBuf::from("/tmp/test.txt")));
}

#[test]
fn test_parse_args_with_no_extension() {
    // 拡張子のないパスの場合、ディレクトリと判断される
    let mode = parse_args(vec!["lala", "mydir"]);
    assert_eq!(mode, StartupMode::OpenDir(PathBuf::from("mydir")));
}

// 注意: -h と -v フラグのテストは、clapがstd::process::exitを呼び出すため、
// ユニットテストでは直接テストできません。これらは統合テストまたは
// 手動テストで確認する必要があります。
//
// これらのフラグの動作確認は、以下のコマンドで手動で実行してください：
// cargo run -- -h
// cargo run -- -v
