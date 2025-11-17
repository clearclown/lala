// パフォーマンステスト: パース処理が10ms以下で完了することを確認
use lala::cli::parse_args;
use std::time::Instant;

fn main() {
    let iterations = 1000;
    let mut total_time = std::time::Duration::ZERO;

    // ウォームアップ
    for _ in 0..100 {
        let _ = parse_args(vec!["lala", "test.txt"]);
    }

    // 実際の測定
    println!("パース処理のパフォーマンステスト（{iterations}回の平均）");

    // テストケース1: ファイルパス
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = parse_args(vec!["lala", "test.txt"]);
    }
    let elapsed = start.elapsed();
    let avg = elapsed / iterations;
    total_time += elapsed;
    println!("ファイルパス: 平均 {avg:?} / 回");

    // テストケース2: ディレクトリパス
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = parse_args(vec!["lala", "./src"]);
    }
    let elapsed = start.elapsed();
    let avg = elapsed / iterations;
    total_time += elapsed;
    println!("ディレクトリパス: 平均 {avg:?} / 回");

    // テストケース3: 引数なし
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = parse_args(vec!["lala"]);
    }
    let elapsed = start.elapsed();
    let avg = elapsed / iterations;
    total_time += elapsed;
    println!("引数なし: 平均 {avg:?} / 回");

    let overall_avg = total_time / (iterations * 3);
    println!("\n全体の平均: {overall_avg:?} / 回");

    if overall_avg.as_micros() < 10000 {
        println!("✓ パフォーマンス要件を満たしています（10ms以下）");
    } else {
        println!("✗ パフォーマンス要件を満たしていません（10ms以上）");
    }
}
