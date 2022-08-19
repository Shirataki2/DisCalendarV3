<script lang="ts" setup>
import { Item } from '@/types/listItem'

const { isLoggedin } = useAuth()

const isShow = (item: Item) => {
  if (item.login) {
    return isLoggedin.value
  }
  if (item.logout) {
    return !isLoggedin.value
  }
  return true
}

type Props = {
  items: Item[]
  nav?: boolean
  density?: 'compact' | 'default' | 'comfortable'
}

const props = withDefaults(defineProps<Props>(), {
  nav: false,
  density: 'default',
})
</script>

<template>
  <v-list :nav="props.nav" :density="props.density">
    <ClientOnly>
      <slot name="prepend" />
      <template v-for="item in props.items">
        <v-list-item
          v-if="isShow(item)"
          :key="item.title"
          color="blue"
          :to="item.ty === 'link' && !item.external ? item.to : undefined"
          :href="item.ty === 'link' && item.external ? item.to : undefined"
          :link="item.ty === 'link'"
          :prepend-icon="item.icon"
          :title="item.title"
          :target="item.ty === 'link' && item.external ? '_blank' : undefined"
          :rel="item.ty === 'link' && item.external ? 'noreferrer' : undefined"
          @click="item.ty === 'func' && item.action()"
        ></v-list-item>
      </template>
    </ClientOnly>
  </v-list>
</template>

<style scoped></style>
