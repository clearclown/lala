use lala::core::EditorCore;
use std::fs;
use std::io::Write;
use std::time::Instant;

#[tokio::main]
async fn main() {
    println!("=== パフォーマンステスト ===\n");

    // 大規模ファイル（約100MB）のテスト
    let large_file = "/tmp/large_test_file.txt";
    let file_size_mb = 100;

    println!("1. {}MBのテストファイルを作成中...", file_size_mb);
    let start = Instant::now();
    {
        let mut file = fs::File::create(large_file).unwrap();
        // 1行約100文字のテキストを生成
        let line = "The quick brown fox jumps over the lazy dog. This is a performance test for the text editor.\n";
        let iterations = (file_size_mb * 1024 * 1024) / line.len();
        for _ in 0..iterations {
            file.write_all(line.as_bytes()).unwrap();
        }
    }
    println!("   完了: {:?}\n", start.elapsed());

    // ファイル読み込みテスト
    println!("2. ファイル読み込みテスト...");
    let start = Instant::now();
    let mut editor = EditorCore::new();
    editor.load_file(large_file).await.unwrap();
    let load_time = start.elapsed();
    println!("   読み込み時間: {:?}", load_time);
    println!("   バッファサイズ: {} 文字\n", editor.len());

    // 挿入操作のパフォーマンステスト（先頭）
    println!("3. 先頭への挿入操作テスト...");
    let start = Instant::now();
    editor.insert(0, "HEADER: ").unwrap();
    let insert_time = start.elapsed();
    println!("   挿入時間: {:?}", insert_time);
    assert!(insert_time.as_millis() < 16, "挿入操作が16ms以上かかりました");

    // 中間への挿入操作のパフォーマンステスト
    println!("4. 中間への挿入操作テスト...");
    let middle = editor.len() / 2;
    let start = Instant::now();
    editor.insert(middle, "MIDDLE: ").unwrap();
    let insert_middle_time = start.elapsed();
    println!("   挿入時間: {:?}", insert_middle_time);
    assert!(insert_middle_time.as_millis() < 16, "挿入操作が16ms以上かかりました");

    // 削除操作のパフォーマンステスト
    println!("5. 削除操作テスト...");
    let start = Instant::now();
    editor.delete(0, 8).unwrap();
    let delete_time = start.elapsed();
    println!("   削除時間: {:?}", delete_time);
    assert!(delete_time.as_millis() < 16, "削除操作が16ms以上かかりました");

    // Undo操作のパフォーマンステスト
    println!("6. Undo操作テスト...");
    let start = Instant::now();
    editor.undo().unwrap();
    let undo_time = start.elapsed();
    println!("   Undo時間: {:?}", undo_time);
    assert!(undo_time.as_millis() < 16, "Undo操作が16ms以上かかりました");

    // Redo操作のパフォーマンステスト
    println!("7. Redo操作テスト...");
    let start = Instant::now();
    editor.redo().unwrap();
    let redo_time = start.elapsed();
    println!("   Redo時間: {:?}", redo_time);
    assert!(redo_time.as_millis() < 16, "Redo操作が16ms以上かかりました");

    // 複数の編集操作
    println!("8. 100回の連続挿入操作テスト...");
    let start = Instant::now();
    for i in 0..100 {
        editor.insert(i * 10, "X").unwrap();
    }
    let multi_insert_time = start.elapsed();
    println!("   合計時間: {:?}", multi_insert_time);
    println!("   平均時間: {:?}", multi_insert_time / 100);

    // ファイル保存テスト
    let output_file = "/tmp/large_test_output.txt";
    println!("\n9. ファイル保存テスト...");
    let start = Instant::now();
    editor.save_file(output_file).await.unwrap();
    let save_time = start.elapsed();
    println!("   保存時間: {:?}\n", save_time);

    // クリーンアップ
    fs::remove_file(large_file).ok();
    fs::remove_file(output_file).ok();

    println!("=== 全てのパフォーマンステストが成功しました ===");
    println!("\n要約:");
    println!("  - ファイル読み込み: {:?}", load_time);
    println!("  - 先頭への挿入: {:?} (< 16ms)", insert_time);
    println!("  - 中間への挿入: {:?} (< 16ms)", insert_middle_time);
    println!("  - 削除: {:?} (< 16ms)", delete_time);
    println!("  - Undo: {:?} (< 16ms)", undo_time);
    println!("  - Redo: {:?} (< 16ms)", redo_time);
    println!("  - ファイル保存: {:?}", save_time);

    if load_time.as_secs() > 5 {
        println!("\n警告: ファイル読み込みが5秒以上かかりました");
    } else {
        println!("\n✓ すべての操作がパフォーマンス要件を満たしています");
    }
}
