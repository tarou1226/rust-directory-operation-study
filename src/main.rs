use std::path::Path;
use std::env;
fn main() {
    // ルートディレクトリの宣言
    let root = Path::new("/");
    println!("show root -> {}", root.display());

    // カレントディレクトリを変更する前の表示
    let old_current = env::current_dir().unwrap();
    println!("old current dir -> {}", old_current.display());

    // カレントディレクトリ変更処理
    env::set_current_dir(root).unwrap();

    // カレントディレクトリを変更した後の表示
    let young_current = env::current_dir().unwrap();
    println!("young current dir -> {}", young_current.display());
}
