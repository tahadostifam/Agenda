<template>
  <v-app>
    <v-main>
      <v-container height="100vh" class="inbox-container">
        <h1 class="text-h4 border-b mb-3 pb-2">Inbox</h1>
        <div class="inbox-list">
          <div :key="index" v-for="(item, index) in list" class="inbox-item">
            <div>
              <h3>{{ item.subject }}</h3>
            </div>
            <div class="inbox-item-actions">
              <v-btn color="transparent" elevation="0" density="comfortable" icon="mdi-pen"></v-btn>
              <v-btn color="transparent" elevation="0" density="comfortable" icon="mdi-delete"></v-btn>
            </div>
          </div>
        </div>
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
export default {
  data: () => ({
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

      &, .inbox-item-actions {
        transition: 0.3s ease;
      }

      &:hover  {
        background-color: rgba(var(--v-border-color), 0.05);
        .inbox-item-actions {
          opacity: 1 !important;
        }
      }
    }
  }
}
</style>
