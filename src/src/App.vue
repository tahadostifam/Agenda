<template>
  <v-app>
    <WindowFrame>
      <v-main>
        <v-dialog
          transition="dialog-bottom-transition"
          v-model="createDialog"
          max-width="700"
        >
          <v-card class="pa-3">
            <h1 class="text-h6">Add</h1>

            <v-text-field
              v-model="subjectInput"
              color="primary"
              label="Subject"
              variant="underlined"
            ></v-text-field>
            <v-textarea
              v-model="contextInput"
              color="primary"
              label="Context"
              variant="underlined"
            ></v-textarea>

            <v-row class="pa-3">
              <v-btn
                block
                size="large"
                text="Close"
                variant="plain"
                @click="createDialog = false"
                class="mb-1"
              ></v-btn>

              <v-btn
                block
                size="large"
                color="primary"
                text="Save"
                variant="tonal"
                @click="add_todo()"
              ></v-btn>
            </v-row>
          </v-card>
        </v-dialog>

        <v-container class="inbox-container">
          <div
            class="d-flex flex-row justify-space-between"
            style="width: 100%"
          >
            <h1 class="text-h4 mb-3 pb-2 pl-2" style="width: 100%">Inbox</h1>
            <v-btn
              color="transparent"
              elevation="0"
              density="comfortable"
              icon="mdi-plus"
              @click="createDialog = true"
            >
            </v-btn>
          </div>

          <div v-if="list.length > 0" class="inbox-list">
            <div :key="index" v-for="(item, index) in list" class="inbox-item">
              <div>
                <h3>{{ item.subject }}</h3>
              </div>
              <div class="inbox-item-actions">
                <v-btn
                  color="transparent"
                  elevation="0"
                  density="comfortable"
                  icon="mdi-pen"
                  @click="createDialog = true"
                ></v-btn>
                <v-btn
                  color="transparent"
                  elevation="0"
                  density="comfortable"
                  icon="mdi-delete"
                  @click="remove_todo(item.id)"
                ></v-btn>
              </div>
            </div>
          </div>
          <div v-else>
            <span class="pl-3">No todos for now :)</span>
          </div>
        </v-container>
      </v-main>
    </WindowFrame>
  </v-app>
</template>

<script lang="ts">
import WindowFrame from "./components/WindowFrame.vue";
import { invoke } from "@tauri-apps/api/core";
import { Todo } from "./models/Todo";

export default {
  components: { WindowFrame },
  mounted() {
    this.load_todos();
  },
  methods: {
    load_todos() {
      invoke("load_todos").then((list) => {
        this.$data.list = list as any;
      });
    },
    add_todo() {
      if (
        this.$data.subjectInput.trim().length > 0 &&
        this.$data.contextInput.trim().length > 0
      ) {
        invoke("add_todo", {
          subj: this.$data.subjectInput.trim(),
          contx: this.$data.contextInput.trim(),
        }).then((todo) => {
          this.$data.list.push(todo as Todo);
          this.clear_add_inputs();
        });
        this.createDialog = false;
      }
    },
    remove_todo(id: number) {
      invoke("remove_todo", { id }).then(() => {
        this.$data.list = this.$data.list.filter((item) => item.id !== id);
      });
    },
    clear_add_inputs() {
      this.$data.subjectInput = "";
      this.$data.contextInput = "";
    },
  },
  data: () => ({
    createDialog: false,
    subjectInput: "",
    contextInput: "",

    list: [] as Todo[],
  }),
};
</script>

<style lang="scss">
.inbox-container {
  .inbox-list {
    overflow: scroll;
    .inbox-item {
      padding: 3px 10px;
      width: 100%;
      display: flex;
      flex-direction: row;
      align-items: center;
      justify-content: space-between;
      border-radius: 8px;

      .inbox-item-actions {
        display: flex;
        flex-direction: row;
        gap: 10px;
        opacity: 0;
      }

      &,
      .inbox-item-actions {
        transition: 0.3s ease;
      }

      &:hover {
        background-color: rgba(var(--v-border-color), 0.05);
        .inbox-item-actions {
          opacity: 1 !important;
        }
      }
    }
  }
}
</style>
