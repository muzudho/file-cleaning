# 作業日誌

## [2025-05-24_Sat]

TODO 次回から、`npm` ではなく、 `pnpm` を使うようにしたい。  

```shell
node --version
    v22.16.0

npm --version
    10.8.2

rustc --version
    rustc 1.87.0 (17067e9ac 2025-05-09)

rustup update stable
rustup default stable

npm install -g @tauri-apps/cli
    changed 2 packages in 4s

    1 package is looking for funding
    run `npm fund` for details

npm fund
    file-cleaning-tauri-desktopapp@0.1.0

tauri --version
    tauri-cli 2.5.0
```

```shell
rustup target add x86_64-pc-windows-msvc
```

```shell
npm install
npm fund
    file-cleaning-tauri-desktopapp@0.1.0
    ├── https://opencollective.com/tauri
    │   └── @tauri-apps/api@2.5.0, @tauri-apps/cli@2.5.0
    ├─┬ https://github.com/vitejs/vite?sponsor=1
    │ │ └── vite@6.3.5
    │ ├── https://github.com/sponsors/jonschlinkert
    │ │   └── picomatch@4.0.2
    │ └── https://github.com/sponsors/SuperchupuDev
    │     └── tinyglobby@0.2.13
    ├── https://github.com/fb55/entities?sponsor=1
    │   └── entities@4.5.0
    ├─┬ https://opencollective.com/postcss/
    │ │ └── postcss@8.5.3
    │ └── https://github.com/sponsors/ai
    │     └── nanoid@3.3.11
    ├── https://github.com/sponsors/antfu
    │   └── vue-demi@0.14.10
    └── https://github.com/sponsors/isaacs
        └── minimatch@9.0.5

npm run tauri dev
```

* [Ctrl + Shift + P] → Rust Analyzer: Reload Workspace

実行のため、（Powershell ではなく コマンドプロンプトを使って）以下のコマンドを打鍵：  

```shell
npm run tauri dev
```


## [2025-06-01_Sun]

（パワーシェルではなく）コマンドプロンプトを開く。  

👇 とりあえず以下を打鍵。  

```shell
npm run tauri dev
```

`npm` ではなく、 `pnpm` に替えたい。  

👇 以下のコマンドを打鍵する。  

```shell
# pnpm
pnpm install
```

📄 `src-tauri/tauri.conf.json` を以下のように置換。  

```
    // "beforeDevCommand": "npm run dev",
    "beforeDevCommand": "pnpm dev",

    // "beforeBuildCommand": "npm run build",
    "beforeBuildCommand": "pnpm build",
```

📄 `package-lock.json` は削除。  

👇 以下のコマンドを打鍵すると、開発モードで実行。  

```shell
pnpm dev
```
