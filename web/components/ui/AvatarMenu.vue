<script lang="ts" setup>
import MenuList from '@/components/ui/MenuList.vue'
import { Item } from '@/types/listItem'
const { currentUserAvatarUrl } = useAuth()
const { toggleTheme } = useUI()

type Props = {
  size?: number
}

const props = withDefaults(defineProps<Props>(), {
  size: 40,
})

const items: Item[] = [
  {
    ty: 'func',
    title: 'ホーム',
    action: () => {
      useRouter().push('/')
    },
  },
  {
    ty: 'func',
    title: 'テーマ切り替え',
    action: toggleTheme,
  },
  {
    ty: 'func',
    title: 'アカウント設定',
    action: () => {
      useRouter().push('/account')
    },
  },
  {
    ty: 'func',
    title: 'サポートサーバー',
    action: () => {
      window.open('https://discord.gg/MyaZRuze23', '_blank')
    },
  },
  {
    ty: 'func',
    title: 'ログアウト',
    action: () => {
      useRouter().push('/logout')
    },
  },
]
</script>

<template>
  <div>
    <v-menu>
      <template #activator="{ props: menuProps }">
        <v-btn v-bind="menuProps" icon :size="props.size">
          <v-avatar :size="props.size">
            <v-img :src="currentUserAvatarUrl" />
          </v-avatar>
        </v-btn>
      </template>
      <v-card class="px-2">
        <MenuList :items="items" density="compact" nav />
      </v-card>
    </v-menu>
  </div>
</template>

<style scoped></style>
