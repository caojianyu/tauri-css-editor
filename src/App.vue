<script setup lang="ts">
import { ref, onMounted, nextTick, onBeforeUnmount } from "vue";
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import FileTree from "./components/FileTree.vue";
import { appWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/api/dialog";
import { homeDir } from "@tauri-apps/api/path";
import { emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

import { readTextFile, BaseDirectory } from "@tauri-apps/api/fs";

import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
import cssWorker from "monaco-editor/esm/vs/language/css/css.worker?worker";
import htmlWorker from "monaco-editor/esm/vs/language/html/html.worker?worker";
import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";
import EditorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import * as monaco from "monaco-editor";

const text = ref("");
// const route = useRoute();
const language = ref("css");
const msg = ref();
const loading = ref(false);

const codeEditBox = ref();

const currentfilePath = ref("");
//
// MonacoEditor start
//
onBeforeUnmount(() => {
  editor.dispose();
});
// @ts-ignore
self.MonacoEnvironment = {
  getWorker(_: string, label: string) {
    if (label === "json") {
      return new jsonWorker();
    }
    if (label === "css" || label === "scss" || label === "less") {
      return new cssWorker();
    }
    if (label === "html" || label === "handlebars" || label === "razor") {
      return new htmlWorker();
    }
    if (["typescript", "javascript"].includes(label)) {
      return new tsWorker();
    }
    return new EditorWorker();
  },
};
let editor: any;

const editorInit = () => {
  monaco.languages.typescript.javascriptDefaults.setDiagnosticsOptions({
    noSemanticValidation: true,
    noSyntaxValidation: false,
  });
  monaco.languages.typescript.javascriptDefaults.setCompilerOptions({
    target: monaco.languages.typescript.ScriptTarget.ES2016,
    allowNonTsExtensions: true,
  });

  !editor
    ? (editor = monaco.editor.create(codeEditBox.value as HTMLElement, {
        language: "css",
        automaticLayout: true,
        theme: "vs-dark",
        foldingStrategy: "indentation",
        renderLineHighlight: "all",
        selectOnLineNumbers: true,
        readOnly: false,
        fontSize: 14,
      }))
    : editor.setValue("");

  editor.onDidChangeModelContent((val: any) => {
    text.value = editor.getValue();
    console.log(text.value);
    console.log(editor.getValue());
  });
};

const list = ref([] as any[]);

const templateList = ref([] as any[]);

const current = ref("");

const isShowSubmenu = ref(false);
const codeCount = ref(1);

// const code = ref();

const isShowMenu = ref(false);
const rightMenuX = ref(0);
const rightMenuY = ref(0);
const updateDate = ref();

onMounted(() => {
  editorInit();

  templateInit();
  listen("open_folder", (event) => {
    list.value = event.payload as any[];
    updateDate.value = new Date();
  });

  templateList.value = [
    {
      name: "Flex Layout",
      code: '<p><span style="color:#9CDCF0;"> display</span>: <span style="color:#CE9178;">flex</span>;</p>',
      list: [
        {
          name: "Left",
          code: '<p><span style="color:#9CDCF0;"> justify-content</span>: <span style="color:#CE9178;">left</span>;</p>',
        },
        {
          name: "Right",
          code: '<p><span style="color:#9CDCF0;"> justify-content</span>: <span style="color:#CE9178;">right</span>;</p>',
        },
        {
          name: "Center",
          code: '<p><span style="color:#9CDCF0;"> justify-content</span>: <span style="color:#CE9178;">center</span>;</p>',
        },
        {
          name: "Center",
          code: '<p><span style="color:#9CDCF0;"> justify-content</span>: <span style="color:#CE9178;">space-between</span>;</p>',
        },
      ],
    },
  ];
});

const templateInit = async () => {
  const css = await readTextFile("template\\css.json", {
    dir: BaseDirectory.Resource,
  });
  templateList.value = JSON.parse(css);
};

const minimizeWindow = () => {
  appWindow.minimize();
};

const toggleMaximize = () => {
  appWindow.toggleMaximize();
};

const closeWindow = () => {
  appWindow.close();
};

const openFile = () => {
  isShowSubmenu.value = !isShowSubmenu.value;
};

const closeMenu = () => {
  if (isShowSubmenu.value) {
    isShowSubmenu.value = false;
  }
  if (isShowMenu.value) {
    isShowMenu.value = false;
  }
};

const openFolder = async () => {
  const selected = await open({
    directory: true,
    multiple: false,
    defaultPath: await homeDir(),
  });

  if (Array.isArray(selected)) {
    // user selected multiple directories
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    // user selected a single directory
    console.log(selected);
  }
  closeMenu();
  list.value = await invoke("open_folder", { dir: selected });
};

const saveFile = () => {
  console.log(text.value);
  invoke("write_text_file", { dir: currentfilePath.value, text: text.value });
  closeMenu();
};

// const textChange = () => {
//   getFocus();
//   const nodes = code.value.childNodes;
//   // codeCount.value = nodes.length > 0 ? nodes.length : 1;
//   let codeHeight = 0;
//   if (nodes.length > 0) {
//     nodes.forEach((item: any) => {
//       if (item && item.clientHeight) {
//         codeHeight += item.clientHeight;
//       }
//     });
//     codeCount.value = codeHeight > 0 ? codeHeight / 20 : 1;
//   }
// };

// let lastEditRange: any;
// const getFocus = () => {
//   var selection = getSelection();
//   lastEditRange = selection?.getRangeAt(0);
// };

// const insertText = (text: string) => {
//   let selection: any = getSelection();
//   if (lastEditRange) {
//     selection?.removeAllRanges();
//     selection?.addRange(lastEditRange);
//   }

//   let node = document.createElement("div");
//   node.innerHTML = text;
//   selection?.getRangeAt(0).insertNode(node);

//   let range = document.createRange();
//   range.selectNodeContents(node);
//   range.setStart(node, 0);
//   range.collapse(true);
//   selection?.removeAllRanges();
//   selection?.addRange(range);
//   textChange();
// };

// const rightClick = (e: any) => {
//   rightMenuX.value = e.offsetX + 60;
//   rightMenuY.value = e.offsetY;
//   isShowMenu.value = true;
//   console.log(e);
// };

// const log = console.log;

const readFile = (filePath: string, content: string) => {
  // listen("read_file", (event) => {
  //   if(event.payload != text.value){
  //     editor.setValue(event.payload);
  //   }
  // });
  currentfilePath.value = filePath;
  editor.setValue(content);
};

const insertText = (text: string) => {
  let position = editor.getPosition();
  editor.executeEdits("", [
    {
      range: {
        startLineNumber: position.lineNumber,
        startColumn: position.column,
        endLineNumber: position.lineNumber,
        endColumn: position.column,
      },
      text: text,
      forceMoveMarkers: true,
    },
  ]);
};
</script>

<template>
  <div @click="closeMenu()" data-tauri-drag-region class="titlebar">
    <div class="menu">
      <div
        @click.stop="openFile()"
        :style="isShowSubmenu ? 'background: #454646;' : ''"
      >
        <span>File</span>
        <div v-show="isShowSubmenu" class="submenu">
          <p @click.stop="openFolder()">Open Folder</p>
          <p @click.stop="saveFile()">Save</p>
        </div>
      </div>
    </div>
    <div>
      <div @click="minimizeWindow()" class="titlebar-button">
        <i class="iconfont icon-subtract"></i>
      </div>
      <div @click="toggleMaximize()" class="titlebar-button">
        <i class="iconfont icon-24gl-minimize2"></i>
      </div>
      <div @click="closeWindow()" class="titlebar-button close-btn">
        <i class="iconfont icon-close1"></i>
      </div>
    </div>
  </div>
  <div @click="closeMenu()" class="content">
    <div class="sidebar">
      <FileTree @read-text-file="readFile" :list="list"></FileTree>
    </div>
    <div class="editor-area">
      <!-- <div class="code-box">
        <div class="code-count">
          <p v-for="num in codeCount" :key="num">{{ num }}</p>
        </div>
        <div
          ref="code"
          @click="getFocus()"
          contenteditable="true"
          @keyup="textChange"
          @keydown="textChange"
          @contextmenu.prevent="rightClick"
          class="code-area"
        ></div>
        <div
          class="right-menu"
          :style="{ left: rightMenuX + 'px', top: rightMenuY + 'px' }"
          v-show="isShowMenu"
        >
          <p>Format Code</p>
        </div>
      </div> -->
      <div ref="codeEditBox" class="code-area"></div>
    </div>
    <div class="template">
      <div v-for="(item, index) in templateList" :key="index">
        <div class="title">{{ item.name }}</div>
        <div class="list">
          <div class="option" v-for="(child, i) in item.list" :key="i">
            <span @click="insertText(item.code + child.code)">
              {{ child.name }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div @click="closeMenu()" class="footer">
    <div>directory update time: {{ updateDate }}</div>
    <div></div>
  </div>
</template>

<style scoped>
.titlebar {
  height: 30px;
  background: #323233;
  user-select: none;
  display: flex;
  justify-content: space-between;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  color: #cccccc;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 46px;
  height: 30px;
}
.titlebar-button:hover {
  background: #474748;
}
.close-btn:hover {
  background: #d61425;
  color: #fefcfc;
}
.icon-24gl-minimize2 {
  font-size: 14px;
}

.menu {
  display: flex;
  align-items: center;
  margin-left: 15px;
}

.menu > div {
  font-size: 12px;
  padding: 0 8px;
  height: 22px;
  border-radius: 5px;
  position: relative;
  line-height: 22px;
}

.menu > div:hover {
  background: #454646;
}

.right-menu,
.submenu {
  position: absolute;
  left: 0;
  background: #303031;
  border-radius: 4px;
  white-space: nowrap;
  padding: 5px 0;
  box-shadow: 0 1px 7px 0 rgba(0, 0, 0, 0.4);
  color: #cccccc;
}

.right-menu p,
.submenu p {
  padding: 0 20px;
  line-height: 26px;
}

.right-menu p:hover,
.submenu p:hover {
  background: #04395e;
}

.content {
  margin-top: 30px;
  height: calc(100vh - 52px);
  display: flex;
}

.sidebar {
  width: 20%;
  height: 100%;
  background: #252526;
  font-size: 13px;
  overflow: auto;
}

.sidebar::-webkit-scrollbar {
  width: 12px;
  height: 12px;
}

.sidebar::-webkit-scrollbar-thumb {
  height: 5px;
  background-color: rgba(255, 255, 255, 0.34);
}
.editor-area {
  width: 60%;
  background: #1e1e1e;
}

/* .editor-area {
  width: 100%;
  height: 100%;
  display: flex;
  font-size: 14px;
  line-height: 20px;
  font-family: Consolas, "Courier New", monospace;
} */

/* .editor-area div {
  line-height: 20px;
} */
/* .code-box {
  width: 100%;
  display: flex;
  overflow-y: auto;
  position: relative;
} */

.template {
  width: 20%;
  height: 100%;
  color: #cccccc;
  background: #1e1e1e;
  overflow-y: auto;
}

.template::-webkit-scrollbar {
  width: 12px;
  height: 12px;
}

.template::-webkit-scrollbar-thumb {
  height: 5px;
  background-color: rgba(255, 255, 255, 0.34);
}

.title {
  padding: 0 5px;
}

.list {
  display: flex;
  flex-wrap: wrap;
}

.list > div {
  width: 100px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
}
.list > div span {
  width: 90px;
  height: 30px;
  line-height: 30px;
  background: #37373d;
  text-align: center;
  cursor: default;
  font-size: 12px;
}

.option span:hover {
  background: #44444b;
}

/* .code-area {
  width: 100%;
  outline: none;
  caret-color: #aeafad;
  color: #d4d4d4;
  white-space: pre-wrap;
} */

.code-area {
  width: 100%;
  height: 100%;
}

.code-count {
  width: 50px;
  flex-shrink: 0;
  color: #858585;
  text-align: right;
  padding-right: 10px;
  font-size: 14px;
}

.editor-area > div::-webkit-scrollbar {
  width: 14px;
  height: 6px;
}
.editor-area > div::-webkit-scrollbar-thumb:vertical {
  height: 5px;
  background-color: #424242;
}

.editor-area > div::-webkit-scrollbar-thumb:hover {
  background-color: #4f4f4f;
}

.footer {
  height: 22px;
  background: #007acc;
  display: flex;
  justify-content: space-between;
  color: #fffff7;
  font-size: 12px;
  line-height: 22px;
  padding: 0 15px;
}
</style>
