<template>
  <h1>{{ msg }}</h1>
  
  <button type="button" @click="click_hw">click hello world</button>

  <button type="button" @click="click_parameter">click parameter</button>
</template>


<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

// 监听 rust 发来消息
listen("rust-event", rustEvent);
function rustEvent(v) {
  // 调用 rust 命令
  console.log("rust-event: ", v);
}

function click_hw() {
  // 调用 rust 命令
  let r = invoke("command_1");
  r.then((d) => {
    // 返回值
    console.log("invoke then: ", d);
  });
}

function click_parameter() {
  // 调用 rust 命令, 带参数
  invoke("command_2", { p: { v: "123asdasd" } });
  invoke("command_3", { p: { v: 22 } });
}

defineProps({
  msg: String,
});

</script>



<style scoped>
a {
  color: #42b983;
}
</style>
