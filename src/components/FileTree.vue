<script setup lang="ts">
import { ref, onMounted } from "vue";
import { store } from "../store/index";
import { invoke } from "@tauri-apps/api/tauri";

const openArr = ref([] as number[]);
onMounted(() => {});

const props = defineProps<{ list: any[] }>();

const toggle = async (index: number, item: any) => {
  store.setCurrentId(item.id);
  const i = openArr.value.indexOf(index);
  if (i != -1) {
    openArr.value.splice(i, 1);
  } else {
    if (props.list[index].chidren) {
      openArr.value.push(index);
    }
  }
  if (!item.is_dir) {
    const content: string = await invoke("read_text_file", {
      dir: item.file_path,
    });
    emit("readTextFile", item.file_path, content);
  }
};

const emit = defineEmits<{
  (e: "readTextFile", filePath: string, content: string): void;
}>();

const readTextFile = (filePath: string, content: string) => {
  emit("readTextFile", filePath, content);
};
</script>

<template>
  <ul onselectstart="return false">
    <li v-for="(item, index) of list" :key="index">
      <div
        class="node-name"
        :style="'padding-left: ' + item.level * 15 + 'px'"
        :class="item.id == store.currentId ? 'active' : ''"
        @click="toggle(index, item)"
      >
        <i
          v-if="item.is_dir"
          class="iconfont icon-arrow-down icon-arrow"
          :class="
            openArr.includes(index) ? 'icon-arrow-down' : 'icon-arrow-right'
          "
        ></i>
        <span v-else-if="item.extension == 'css'" class="icon-file">#</span>
        <span v-else>&lt;&gt;</span>
        {{ item.name }}
      </div>
      <ul
        class="node-list"
        v-show="openArr.includes(index)"
        v-if="item.chidren"
      >
        <li>
          <FileTree @read-text-file="readTextFile" :list="item.chidren" />
        </li>
      </ul>
    </li>
  </ul>
</template>

<style scoped>
ul {
  list-style-type: none;
  margin: 0;
  padding: 0;
}
.node-list {
  line-height: 24px;
  color: #ccc;
}
.node-name {
  line-height: 24px;
  color: #ccc;
  padding-left: 20px;
  cursor: pointer;
  white-space: nowrap;
}

.node-name:hover {
  background: #2a2d2e;
}
.active {
  background: #37373d;
}

.icon-arrow {
  font-size: 14px;
}

.icon-file {
  font-weight: 600;
  color: #3799d6;
}
</style>
