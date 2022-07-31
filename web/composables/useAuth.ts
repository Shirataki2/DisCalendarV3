import { getApiUrl } from '@/server'
import { Discord } from '@/types'

export const useAuth = () => {
  const { user, userGuilds } = useUser()
  const isLoggedin = computed(() => user.value !== null)

  const logout = () => {
    user.value = null
    userGuilds.value = null
  }

  const fetchUser = async () => {
    const userData = await $fetch<Discord.ClientUser>(getApiUrl('/users/me'))
    if (userData) {
      user.value = userData
    }
  }

  const fetchUserGuilds = async () => {
    const guilds = await $fetch<Discord.InviteGuildResponse>(
      getApiUrl('/guilds/list')
    )
    if (guilds) {
      userGuilds.value = guilds
    }
  }

  return {
    user,
    userGuilds,
    isLoggedin,
    logout,
    fetchUser,
    fetchUserGuilds,
  }
}
