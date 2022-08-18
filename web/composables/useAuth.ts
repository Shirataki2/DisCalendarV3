import { getApiUrl } from '@/server'
import { Discord } from '@/types'

const CDN_BASE_URL = 'https://cdn.discordapp.com'
const DEFALUT_AVATAR_PATH = '/images/default-guild-icon.png'

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

  const currentUserAvatarUrl = computed(() => {
    if (!user.value?.avatar) {
      return DEFALUT_AVATAR_PATH
    }
    return `${CDN_BASE_URL}/avatars/${user.value.id}/${user.value.avatar}.png`
  })

  return {
    user,
    userGuilds,
    isLoggedin,
    logout,
    fetchUser,
    fetchUserGuilds,
    currentUserAvatarUrl,
  }
}
