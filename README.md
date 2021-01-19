# モジュールとクレート入門

- モジュールは、pubでないと他のファイルから参照することができない
- 同じディレクトリに、ディレクトリと同名のrsモジュールを作成することで、下の階層にサブモジュールを作成することが出来る。

# cargo入門

## cargo build
```sh
cargo build [OPTIONS]
```
すべてのクレートの依存関係を解決し、必要なクレートをダウンロードした上で、バイナリの生成を行う。
デフォルトで、デバッグに向いたdevオプションでビルドされる。
```
--relase
```
オプションを追加することで、最適化されたバイナリが生成される。
（出力されるディレクトリも切り替わる）

## cargo check
```sh
cargo check [OPTIONS]
```
文法チェックを行ない、エラーと警告を出力する。
cargo buildでも行うことが出来るが、バイナリを生成しない分高速

## cargo run
ビルドが必要な場合には、cargo buildを行い、バイナリを実行する。
buildコマンドと同様、デフォルトでdevプロファイルで出力され、
```sh
--release
```
で最適化することが出来る。

## cargo test
```sh
cargo test [OPTIONS] [TESTNAME] [-- TEST-OPTIONS]
```
テスト用バイナリファイルは、#[test]アトリビュートが付いたすべての関数を複数のスレッドで実行する。TESTNAMEを指定することによって、特定のテストのみを実行することが出来る。

なんと、ドキュメント中のサンプルコードまで、チェックしてくれる！素晴らしい。


## cargo fix
このコマンドを実行すると、裏で cargo check が実行される。
受け取った警告の中で、自動修正が可能なものは、修正される。
- 使用していないクレートをuseしている場合、削除する
- 不要なmutがある場合、mutを消去する
- 使用していない変数がある場合、先頭にアンダーバーを追加する

## cargo clean
```sh
cargo clean [OPTIONS]
```
ビルドの生成物を削除する。
--releaseを指定すれば、releaseによる生成物のみが削除される

## cargo doc
```sh
cargo doc [OPTIONS]
```

パッケージに含まれているソースコードと、すべての依存関係のあるクレートについてのドキュメントを作成する。

- --open ドキュメント作成完了後に自動でブラウザを開いてくれる。
- --no-deps 依存関係のあるライブラリのドキュメントまでは作らない
- --document-private-items 非公開(pubではない)要素までドキュメント化する


## cargo install
公開されているパッケージをインストールする
```sh
cargo install [OPTIONS] [CRATE]
```
デフォルトでは、crates.ioからパッケージをダウンロードする
### OPTIONS
- --git, --path, --resistry デフォルトのダウンロード元から変更する。

--root [DIR] デフォルトの ~./cargo からインストール先を変える

## cargo uninstall
```
cargo uninstall [OPTIONS] [SPEC]
```
cargo installでインストールしたバイナリファイルをアンインストールする
### OPTIONS
- --root [DIR] パスを指定する

## cargo search
```sh
cargo search [OPTIONS] [QUERY]
```
crates.ioで公開されているクレートを検索できる。
基本的には、Cargo.tomlに直接コピー出来るようなフォーマットで結果を得ることが出来る。

## cargo publish
開発したクレートがcrates.ioに公開される。
ドキュメントは、Docs.rsに公開される。
(creates.ioに公開されているドキュメントを自動でビルドし、公開してくれるサービス)

### 項目
- authors
- license
- description
- homepage
- documentation
- repository
- readme

### OPTION
- --dry-run アップロードはせずに、すべてのチェックのみ行われる。事前確認のために用いる

## Cargo.tomlについて
TOMLフォーマットで書かれたパッケージの設定ファイル。
マニフェストとも呼ばれる。
内容はセクションごとに別れており、パッケージ情報、依存コード、コンパイラの設定などを記述することが出来る。

### packageセクション
packageセクションは、パッケージ自体の情報を記述する。
デフォルトでいくつかすでに情報が書かれているが、他にも色々な情報を追加することが出来る。

|key|description|
|:---:|:---|
|name|パッケージの名前|
|version|パッケージのバージョン|
|authors|パッケージの作者|
|edition|Rustのエディション|
|description|パッケージの簡潔な説明|
|documentation|ドキュメントのURL|
|readme|READMEファイルのパス|
|homepage|ホームページへのURL|
|repository|ソースコードリポジトリへのURL|
|license|ライセンス情報|
|license-file|ライセンスファイルへのパス|
|keywords|パッケージのキーワード|
|categories|パッケージのカテゴリ|
|workspace|パッケージのワークスペースの設定|
|build|ビルドスクリプトへのパス|
|links|パッケージが利用するネイティブライブラリへのリンク|
|exclude|publish時に除外するファイル|
|include|publish時に含めるファイル|
|publish|publishのコントロール|
|metadata|外部ツールのための設定|
|default-run|cargo runによって動作するバイナリファイル|

## dependencies セクション
実は[dependencies]だけじゃない！
|section|description|
|:---:|:---:|
|[dependencies] | パッケージの依存関係|
|[dev-dependencies] | examples,tests,benchmarkのための依存関係|
|[build-dependencies] | ビルドスクリプトの依存関係|
|[target]|プラットフォーム特有の依存関係|
|