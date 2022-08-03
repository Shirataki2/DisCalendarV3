<script lang="ts" setup>
import { mdiThemeLightDark } from '@mdi/js'
import { useTheme } from 'vuetify'
import DatePicker from '~~/components/calendar/DatePicker.vue'
import NavDrawer from '~~/components/application/NavDrawer.vue'

const theme = useTheme()
const { fetchUser, logout } = useAuth()
const showDrawer = ref<boolean | undefined>(undefined)
const toggleTheme = () => {
  theme.global.name.value =
    theme.global.name.value === 'myLightTheme' ? 'myDarkTheme' : 'myLightTheme'
}

const headerColor = computed(() => {
  return theme.global.name.value === 'myLightTheme' ? 'white' : 'primary'
})

const isCalendarPage = computed(() => {
  const route = useRoute()
  return route.path.match(/^\/servers\/\d+$/) !== null
})

onMounted(async () => {
  try {
    await fetchUser()
  } catch {
    logout()
  }
})
</script>

<template>
  <v-layout>
    <v-app-bar app flat density="compact" :color="headerColor">
      <template #prepend>
        <v-app-bar-nav-icon @click="showDrawer = !showDrawer" />
      </template>
      <v-app-bar-title>
        <NuxtLink id="title" to="/">DisCalendar v3</NuxtLink>
      </v-app-bar-title>
      <DatePicker v-if="isCalendarPage" />
      <v-spacer />
      <template #append>
        <v-btn :icon="mdiThemeLightDark" @click="toggleTheme"></v-btn>
      </template>
    </v-app-bar>
    <v-main>
      <slot />
    </v-main>
    <v-footer app>
      v3.0.0<v-spacer /> &copy; {{ new Date().getFullYear() }} FF
    </v-footer>
    <NavDrawer v-model="showDrawer" />
  </v-layout>
</template>

<style scoped lang="scss">
a#title {
  text-decoration: none;
  &:hover,
  &:visited {
    color: inherit;
  }
}

#title {
  font-family: discord, Arial, Helvetica, sans-serif;
}
</style>
