<script lang="ts" setup>
const { user } = useAuth()
</script>

<template>
  <div>
    Index Page
    <p>{{ user }}</p>
  </div>
</template>

<style scoped></style>
