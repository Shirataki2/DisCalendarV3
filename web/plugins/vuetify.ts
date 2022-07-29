import { createVuetify, ThemeDefinition } from 'vuetify'
import { aliases, mdi } from 'vuetify/iconsets/mdi-svg'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

const myLightTheme: ThemeDefinition = {
  dark: false,
  colors: {
    primary: '#455A64',
    secondary: '#8BC34A',
    background: '#F5F5F5',
    surface: '#FFFFFF',
    error: '#ED0D0D',
    warning: '#F0C800',
    info: '#03ABF5',
    success: '#43C047',
  },
}

const myDarkTheme: ThemeDefinition = {
  dark: true,
  colors: {
    primary: '#303F9F',
    secondary: '#FFC107',
    background: '#141414',
    surface: '#2A2A2A',
    error: '#CD0D0D',
    warning: '#FFC110',
    info: '#03A9F4',
    success: '#48BE3C',
  },
}

export default defineNuxtPlugin(nuxtApp => {
  const vuetify = createVuetify({
    components,
    directives,
    theme: {
      defaultTheme: 'myDarkTheme',
      themes: {
        myLightTheme,
        myDarkTheme,
      },
    },
    icons: {
      defaultSet: 'mdi',
      aliases,
      sets: {
        mdi,
      },
    },
  })
  nuxtApp.vueApp.use(vuetify)
})
