use lala::cli::{parse_args_default, StartupMode};

fn main() {
    // コマンドライン引数をパースして起動モードを取得
    let startup_mode = parse_args_default();

    // 起動モードに応じた処理を実行
    match startup_mode {
        StartupMode::OpenFile(path) => {
            println!("ファイルを開きます: {}", path.display());
            // TODO: 将来的にはcore-engineとgui-baseを統合して、実際にファイルを開く処理を実装
        }
        StartupMode::OpenDir(path) => {
            println!("ディレクトリを開きます: {}", path.display());
            // TODO: 将来的にはcore-engineとgui-baseを統合して、ファイルツリーを表示する処理を実装
        }
        StartupMode::Empty => {
            println!("空のエディタを起動します");
            // TODO: 将来的にはcore-engineとgui-baseを統合して、空のエディタを開く処理を実装
        }
    }
}
