export default defineNuxtRouteMiddleware(async (_to, _from) => {
  const { fetchUser } = useAuth()
  await fetchUser()
})
