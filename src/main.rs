use std::path;
use std::path::Path;
use std::env;
use std::fs;

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

    // 環境変数の取得 （PATH）の取得を行っている
    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            // 複数の場合は改行しながら、splitで分割、全て表示
            for path in env::split_paths(&paths) {
                println!("'{}'", path.display());
            }
        }
        None => println!("{key} is not defined in the environment.")
    }

    // old_currentを参照して、ディレクトリの中身を取得
    let dir = fs::read_dir(&old_current).unwrap();
    // 複数を取得するためにfor文で回す
    for file in dir {
        // エラーハンドリングしてから、PathBufに変換、displayで表示
        println!("{}", file.unwrap().path().display());
    }

    // パスを区切る文字なのかを判定する linuxの場合は'/'
    match path::is_separator('💕') {
        true => println!("ok"),
        false => println!("no"),
    }

    // 処理の終了方法 linux 0(00000000) windows 256(11111111)
    println!("finished process");
    std::process::exit(0)

}
