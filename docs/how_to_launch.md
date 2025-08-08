# 起動方法

（パワーシェルではなく）コマンドプロンプトを開く。  

## `node_modules` フォルダーがまだ無いとき

```shell
pnpm install
```


## 起動

```shell
# 画面の更新だけしたとき（ブラウザでだけ開く）：
pnpm dev

# バックエンドも更新したとき（デスクトップアプリも開く）：
pnpm tauri dev
```
