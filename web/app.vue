<script setup lang="ts">
const { showSnackbar, message, color, timeout } = useSnackbar()

useHead({
  titleTemplate: titleChunk => {
    return titleChunk ? `${titleChunk} - DisCalendar` : 'DisCalendar'
  },
})
const { theme } = useUI()

onMounted(() => {
  const dark = localStorage.getItem('theme:dark') === 'true'
  theme.global.name.value = dark ? 'myDarkTheme' : 'myLightTheme'
})
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
