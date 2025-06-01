# インストール方法

（パワーシェルではなく）コマンドプロンプトを使う。  

TODO 次回から、`npm` ではなく、 `pnpm` を使うようにしたい。  

```shell
npm install -g @tauri-apps/cli@^2.0
npm install @tauri-apps/api
npm install
# Tauri 2.0
npm install @tauri-apps/api@2

npm run tauri add dialog
npm run tauri add fs

cd src-tauri
cargo add tauri-plugin-dialog
cargo install tauri-cli --version "^2.0.0"
```

📖 [Tauri 2.0で「ファイルを開く」ダイアログ(plugin-dialog)を使ってみる](https://zenn.dev/playree/articles/5e2f1386dde48f)  
📖 [Tauri - how to open a new file from the native file explorer](https://stackoverflow.com/questions/77062721/tauri-how-to-open-a-new-file-from-the-native-file-explorer)  
📖 [Open a File Selector Dialog](https://tauri.app/plugin/dialog/#open-a-file-selector-dialog)  
