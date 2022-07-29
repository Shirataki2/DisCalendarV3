import { defineNuxtConfig } from 'nuxt'

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
  typescript: {
    shim: false,
    strict: true,
  },
  css: ['@/assets/styles/main.scss', 'vuetify/styles'],
  build: {
    transpile: ['vuetify'],
  },
  modules: ['nuxt-proxy'],
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
    ],
  },
})
