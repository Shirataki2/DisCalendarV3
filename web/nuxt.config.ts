import { defineNuxtConfig } from 'nuxt'

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
  typescript: {
    strict: true,
  },
  css: [
    '@/assets/styles/main.scss',
    '@/assets/styles/docs.scss',
    '@/assets/styles/calendar.scss',
    'vuetify/styles',
    'vue-cal/dist/vuecal.css',
  ],
  build: {
    transpile: ['vuetify'],
  },
  modules: ['nuxt-proxy', '@vueuse/nuxt', '@nuxt/content'],
  vite: {
    define: {
      'process.env.DEBUG': false,
    },
    server: {
      host: '0.0.0.0',
      port: 6655,
      watch: {
        usePolling: true,
      },
      // proxy: {
      //   '/v3/': {
      //     target: 'http://api:5000',
      //     secure: false,
      //     rewrite(path) {
      //       return path.replace(/^\/v3/, '')
      //     },
      //   },
      // },
    },
  },
  proxy: {
    options: [
      {
        target: 'http://api:5000',
        changeOrigin: true,
        pathRewrite: {
          '^/v3': '',
        },
        pathFilter: ['/v3/**'],
      },
      {
        target: 'http://localhost:15000',
        changeOrigin: true,
        pathRewrite: {
          '^/cv3': '',
        },
        pathFilter: ['/cv3/**'],
      },
    ],
  },
})
