<script lang="ts" setup>
import { mdiThemeLightDark } from '@mdi/js'
import { useTheme } from 'vuetify'

const theme = useTheme()
const { fetchUser, isLoggedin } = useAuth()

const toggleTheme = () => {
  theme.global.name.value =
    theme.global.name.value === 'myLightTheme' ? 'myDarkTheme' : 'myLightTheme'
}

const login = () => {
  window.location.href = 'api/login'
}

const move = () => {
  const route = useRoute()
  const router = useRouter()
  if (route.path === '/') {
    router.push('/tmp')
  } else {
    router.push('/')
  }
}

onMounted(async () => {
  try {
    await fetchUser()
  } catch {
    const user = useUser()
    user.value = null
  }
})
</script>

<template>
  <v-layout>
    <v-app-bar app flat density="compact" color="primary">
      <v-app-bar-title>
        <NuxtLink id="title" to="/">DisCalendar v3</NuxtLink>
      </v-app-bar-title>
      <template #append>
        <v-btn :icon="mdiThemeLightDark" @click="toggleTheme"></v-btn>
        <v-btn v-if="!isLoggedin" flat @click="login">login</v-btn>
        <v-btn v-if="isLoggedin" flat to="/logout">logout</v-btn>
        <v-btn flat @click="move">move</v-btn>
      </template>
    </v-app-bar>
    <v-main>
      <slot />
    </v-main>
    <v-footer app>
      v3.0.0<v-spacer /> &copy; {{ new Date().getFullYear() }} FF
    </v-footer>
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
