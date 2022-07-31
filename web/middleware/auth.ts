import { useUser } from '@/composables/useUser'

export default defineNuxtRouteMiddleware((to, _from) => {
  const { user } = useUser()
  if (!user.value && to.path !== '/login') {
    return navigateTo('/login')
  }
})
