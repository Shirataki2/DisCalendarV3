<script lang="ts" setup>
import {
  mdiHome,
  mdiThemeLightDark,
  mdiConsole,
  mdiGithub,
  mdiLogin,
  mdiLogout,
  mdiGift,
  mdiBookOpenPageVariant,
  mdiAccountSupervisor,
} from '@mdi/js'
import { Item } from 'types/listItem'
import { Discord } from '@/types'

import ServerNav from '@/components/application/ServerNav.vue'
import MenuList from '@/components/ui/MenuList.vue'

const { toggleTheme } = useUI()

const items = ref<Item[]>([
  {
    ty: 'link',
    title: 'ホーム',
    to: '/',
    icon: mdiHome,
  },
  {
    ty: 'link',
    title: 'サーバー一覧',
    to: '/servers',
    icon: mdiConsole,
    login: true,
  },
  {
    ty: 'link',
    title: 'ログイン',
    to: '/login',
    icon: mdiLogin,
    logout: true,
  },
  {
    ty: 'link',
    title: '支援',
    to: '/donate',
    icon: mdiGift,
  },
  {
    ty: 'link',
    title: 'サポートサーバー',
    to: 'https://discord.gg/MyaZRuze23',
    icon: Discord.iconSvg,
    external: true,
  },
  {
    ty: 'link',
    title: 'GitHub',
    to: 'https://github.com/Shirataki2/DisCalendarV3',
    icon: mdiGithub,
    external: true,
  },
  {
    ty: 'link',
    title: '利用規約',
    to: '/tos',
    icon: mdiBookOpenPageVariant,
  },
  {
    ty: 'link',
    title: 'プライバシーポリシー',
    to: '/privacy',
    icon: mdiAccountSupervisor,
  },
  {
    ty: 'func',
    title: 'テーマ変更',
    icon: mdiThemeLightDark,
    action: toggleTheme,
  },
  {
    ty: 'link',
    title: 'ログアウト',
    to: '/logout',
    icon: mdiLogout,
    login: true,
  },
])

const appendItems = ref<Item[]>([])

const props = withDefaults(
  defineProps<{
    modelValue: boolean
  }>(),
  {
    modelValue: false,
  }
)

const emit = defineEmits<{
  (e: 'update:modelValue', v: boolean): void
}>()

const show = computed({
  get: () => props.modelValue,
  set: v => {
    emit('update:modelValue', v)
  },
})
</script>

<template>
  <v-navigation-drawer v-model="show">
    <MenuList :items="items" nav>
      <template #prepend>
        <ServerNav />
      </template>
    </MenuList>
    <template #append>
      <MenuList :items="appendItems"></MenuList>
    </template>
  </v-navigation-drawer>
</template>

<style scoped></style>
