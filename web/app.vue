<script setup lang="ts">
const { showSnackbar, message, color, timeout } = useSnackbar()
</script>

<template>
  <v-app>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
    <v-snackbar v-model="showSnackbar" app :color="color" :timeout="timeout">
      {{ message }}
      <template #actions>
        <v-btn text @click="showSnackbar = false"> OK </v-btn>
      </template>
    </v-snackbar>
  </v-app>
</template>
