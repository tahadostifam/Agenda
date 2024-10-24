<template>
  <div data-tauri-drag-region class="titlebar">
    <h1 class="titlebar-title">Agenda</h1>

    <v-btn
      elevation="0"
      size="x-small"
      @click="closeWindow()"
      class="titlebar-button"
      icon="mdi-close"
    ></v-btn>
  </div>

  <div class="router-view">
    <slot />
  </div>
</template>

<script>
import { onMounted } from "vue";

const { getCurrentWindow } = window.__TAURI__.window;

const appWindow = getCurrentWindow();

export default {
  methods: {
    closeWindow() {
      appWindow.close();
    },
  },
  mounted() {
    document.getElementById("titlebar")?.addEventListener("mousedown", (e) => {
      if (e.buttons === 1) {
        // Primary (left) button
        e.detail === 2
          ? appWindow.toggleMaximize() // Maximize on double click
          : appWindow.startDragging(); // Else start dragging
      }
    });
  },
};
</script>

<style lang="scss">
$titlebar_height: 38px;

.router-view {
  height: calc(100% - $titlebar_height);
  margin-top: $titlebar_height;
}

.titlebar-title {
    font-size: 13px;
    margin-left: 15px;
}

.titlebar {
  height: $titlebar_height;
  user-select: none;
  display: flex;
  justify-content: space-between;
  align-items: center;
  position: fixed;
  z-index: 99;
  gap: 5px;
  padding-right: 5px;
  top: 0;
  left: 0;
  right: 0;
}

.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  border-radius: 50px;
}
</style>
