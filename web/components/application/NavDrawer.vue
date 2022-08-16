<script lang="ts" setup>
import {
  mdiHome,
  mdiThemeLightDark,
  mdiConsole,
  mdiGithub,
  mdiLogin,
  mdiLogout,
  mdiGift,
} from '@mdi/js'
import { useTheme } from 'vuetify'
import { Discord } from '@/types'
import ServerNav from '@/components/application/ServerNav.vue'

const theme = useTheme()
const { isLoggedin } = useAuth()

type LinkItem = {
  title: string
  to: string
  icon: string
  external?: boolean
  login?: boolean
  logout?: boolean
  ty: 'link'
}

type FuncItem = {
  title: string
  icon: string
  login?: boolean
  logout?: boolean
  action: () => void
  ty: 'func'
}

type Item = LinkItem | FuncItem

const toggleTheme = () => {
  theme.global.name.value =
    theme.global.name.value === 'myLightTheme' ? 'myDarkTheme' : 'myLightTheme'
}

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
    title: '寄付',
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
])

const appendItems = ref<Item[]>([
  {
    ty: 'link',
    title: 'ログアウト',
    to: '/logout',
    icon: mdiLogout,
    login: true,
  },
  {
    ty: 'func',
    title: 'テーマ変更',
    icon: mdiThemeLightDark,
    action: toggleTheme,
  },
])

const isShow = (item: Item) => {
  if (item.login) {
    return isLoggedin.value
  }
  if (item.logout) {
    return !isLoggedin.value
  }
  return true
}

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
    <v-list nav>
      <ServerNav />
      <template v-for="item in items">
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
          :append-icon="item.ty === 'func' ? item.icon : undefined"
          @click="item.ty === 'func' && item.action()"
        ></v-list-item>
      </template>
    </v-list>
    <template #append>
      <v-list nav>
        <template v-for="item in appendItems">
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
            :rel="
              item.ty === 'link' && item.external ? 'noreferrer' : undefined
            "
            @click="item.ty === 'func' && item.action()"
          ></v-list-item>
        </template>
      </v-list>
    </template>
  </v-navigation-drawer>
</template>

<style scoped></style>
