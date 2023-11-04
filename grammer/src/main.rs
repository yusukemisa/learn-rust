fn main() { // プログラムのエントリーポイント
    // 文末の改行をエスケープで消す
    let penguin_data = "\
    common name ,length(cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    // record.trim().len()は、テキスト行(recordとして表現されています)を処理するRustコードの一部です。
    // この式は、recordから空白文字を取り除いてから文字列の長さ（文字数）を取得するために使用されています。
    // record.trim()は、recordから先頭と末尾の空白文字（スペース、タブ、改行など）を取り除いた新しい文字列を返します。
    // つまり、行の先頭と末尾にある不要な空白を削除します。
    // .len()は、文字列の長さ（文字数）を取得するメソッドです。
    // この場合、recordから先頭と末尾の空白を取り除いた後の文字列の長さを計算します。
    // このコードの具体的な利用方法は、CSV形式のデータを処理する際に、各行が空行でないかどうかを確認することです。
    // record.trim().len()は、行が空である場合、その長さが0になり、それ以外の場合は1以上の値を持つことになります。
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 { //ヘッダと改行をスキップ
            continue;
        }

        let fields: Vec<_> = record // テキスト業で処理を開始
            .split(',') // レコードをフィールドに分割
            .map(|field| field.trim()) // 各フィールドの空白を除去
            .collect(); // フィールドのコレクションを作る
        // cfg!マクロは、コンパイル時の条件判定に使用され、コンパイラにコードの一部を含めるかどうかを制御するのに役立ちます。
        // cfg!マクロは非常に柔軟で、環境変数、Rustのバージョン、プラットフォームなど、さまざまな条件を評価できます。
        //
        // debug_assertionsはcargo buildでビルドするとデフォルトでtrue
        // コンパイルオプションとして rustc --cfg debug_assertions と明示することも可能
        // cargo build --releaseとする場合falseになる
        if cfg!(debug_assertions) {
            // 標準エラー出力    
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // Okはfields[1]の文字列がf32に正常に変換された場合にのみtrue
        // 変換に成功した場合、lengthに値が格納される
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm",name,length)
        }
    }
}
