<script lang="ts" setup>
const { logout } = useAuth()

onMounted(() => {
  logout()
  window.location.href = 'api/logout'
})
</script>

<template>
  <div>Processing</div>
</template>

<style scoped></style>
