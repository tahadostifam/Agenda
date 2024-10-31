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
              color="primary"
              label="Subject"
              variant="underlined"
            ></v-text-field>
            <v-textarea
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
                @click="createDialog = false"
              ></v-btn>
            </v-row>
          </v-card>
        </v-dialog>

        <CreateDialog />

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

          <div class="inbox-list">
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
                ></v-btn>
              </div>
            </div>
          </div>
        </v-container>
      </v-main>
    </WindowFrame>
  </v-app>
</template>

<script lang="ts">
import WindowFrame from "./components/WindowFrame.vue";

export default {
  components: { WindowFrame },
  data: () => ({
    createDialog: false,

    list: [
      {
        subject: "Meeting Reminder",
        description:
          "Adipisicing ut cillum reprehenderit magna tempor culpa et. Cillum cillum irure do enim mollit ea aute quis veniam. Quis incididunt magna aliqua est quis amet consequat pariatur ipsum minim ullamco. Nisi consequat laboris sunt ea irure elit pariatur occaecat nulla.",
        completed: false,
      },
      {
        subject: "Meeting Reminder",
        description:
          "Adipisicing ut cillum reprehenderit magna tempor culpa et. Cillum cillum irure do enim mollit ea aute quis veniam. Quis incididunt magna aliqua est quis amet consequat pariatur ipsum minim ullamco. Nisi consequat laboris sunt ea irure elit pariatur occaecat nulla.",
        completed: false,
      },
      {
        subject: "Meeting Reminder",
        description:
          "Adipisicing ut cillum reprehenderit magna tempor culpa et. Cillum cillum irure do enim mollit ea aute quis veniam. Quis incididunt magna aliqua est quis amet consequat pariatur ipsum minim ullamco. Nisi consequat laboris sunt ea irure elit pariatur occaecat nulla.",
        completed: false,
      },
      {
        subject: "Meeting Reminder",
        description:
          "Adipisicing ut cillum reprehenderit magna tempor culpa et. Cillum cillum irure do enim mollit ea aute quis veniam. Quis incididunt magna aliqua est quis amet consequat pariatur ipsum minim ullamco. Nisi consequat laboris sunt ea irure elit pariatur occaecat nulla.",
        completed: false,
      },
      {
        subject: "Meeting Reminder",
        description:
          "Adipisicing ut cillum reprehenderit magna tempor culpa et. Cillum cillum irure do enim mollit ea aute quis veniam. Quis incididunt magna aliqua est quis amet consequat pariatur ipsum minim ullamco. Nisi consequat laboris sunt ea irure elit pariatur occaecat nulla.",
        completed: false,
      },
    ],
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
