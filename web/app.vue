<script setup lang="ts">
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
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>
