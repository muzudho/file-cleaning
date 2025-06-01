<script setup lang="ts">
import { ref } from "vue";
//import { invoke } from '@tauri-apps/api/tauri';
import { invoke } from "@tauri-apps/api/core";

import { open } from "@tauri-apps/plugin-dialog";

// ビューモデル。
const greetMsgVM = ref("");
const openFolderPathVM = ref("");

const directoryPathVM = ref('');
const fileListVM = ref<string[]>([]);
const errorVM = ref('');

async function fetchFileList() {
  try {
    // 例： C:/Users/muzud/OneDrive/デスクトップ/新しいフォルダー
    console.log(`ディレクトリー：${directoryPathVM.value}`)
    fileListVM.value = await invoke('get_file_list', { directory_path: directoryPathVM.value });
    errorVM.value = '';
  } catch (err) {
    errorVM.value = 'エラー: ' + (err as string);
    fileListVM.value = [];
  }
}

// ［フォルダーを開く］ボタンクリック時。
async function on_openFolderButton_clicked() {
  try {
    console.log("［フォルダーを開く］ボタン押下")

    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    //greetMsgVM.value = await invoke("greet", { name: openFolderPathVM.value });
    //greetMsgVM.value = await invoke("greet", { name: "テスト１" });

    //*
    // Open a selection dialog for image files
    //const selectedFilePath = await invoke('open_file_dialog');
    const selectedFilePath = await open({
      multiple: false,
      directory: true,
    }); 
    console.log(selectedFilePath)

    if (Array.isArray(selectedFilePath)) {
      // user selected multiple files
    } else if (selectedFilePath === null) {
      // user cancelled the selection
    } else {
      // user selected a single file
    }
    // */
  } catch(error) {
    console.error("Error has occured when read file: ", error);
  }
}
</script>

<template>
  <main class="container">

  <div>
    <h1>ファイル一覧アプリ</h1>
    <input v-model="directoryPathVM" placeholder="ディレクトリパスを入力 (例: /home/user)" />
    <button @click="fetchFileList">ファイル一覧を取得</button>
    <ul v-if="fileListVM.length">
      <li v-for="file in fileListVM" :key="file">{{ file }}</li>
    </ul>
    <p v-if="errorVM">{{ errorVM }}</p>
  </div>

    <h1>ファイル・クリーニング</h1>

    <h2>手順１：</h2>
    <!-- class="row" -->
    <form @submit.prevent="on_openFolderButton_clicked">
      <p class="p2">フォルダーを選択してください。</p>
      <p class="p2">
        <input id="open-folder-path-input" v-model="openFolderPathVM" placeholder="Enter a folder path..." value="Z:\\Muzudho Backup" />
        <button type="submit">フォルダーを開く</button>
      </p>
    </form>
    <p>{{ greetMsgVM }}</p>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.text-fluent-blue { color: #0078D4; }
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: left;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  background-color: #111111;
  color: #EEEEEE;
  padding-top: 8px;
  padding-bottom: 4px;
}

h2 {
  background-color: #111111;
  color: #EEEEEE;
  margin-left: 60px;
  padding-left: 4px;
  padding-top: 6px;
  padding-bottom: 2px;
}

p.p2 {
  color: #111111;
  margin-left: 90px;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#open-folder-path-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

div {
  padding: 20px;
  font-family: Arial, sans-serif;
}
input {
  padding: 8px;
  margin-right: 10px;
}
button {
  padding: 8px 16px;
}
ul {
  list-style: none;
  padding: 0;
}
li {
  padding: 5px 0;
}
</style>