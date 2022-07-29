<script lang="ts" setup>
const user = useUser()

onMounted(() => {
  user.value = null
  window.location.href = 'api/logout'
})
</script>

<template>
  <div>Processing</div>
</template>

<style scoped></style>
