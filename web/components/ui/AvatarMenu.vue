<script lang="ts" setup>
const { currentUserAvatarUrl } = useAuth()

type Props = {
  size?: number
}

const props = withDefaults(defineProps<Props>(), {
  size: 40,
})
</script>

<template>
  <v-menu>
    <template #activator="{ props: menuProps }">
      <v-btn v-bind="menuProps" icon :size="props.size">
        <v-avatar :size="props.size">
          <v-img :src="currentUserAvatarUrl" />
        </v-avatar>
      </v-btn>
    </template>
  </v-menu>
</template>

<style scoped></style>
