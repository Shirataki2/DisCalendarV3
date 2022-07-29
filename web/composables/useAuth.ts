import { getApiUrl } from '@/server'
import { Discord } from '@/types'

export const useAuth = () => {
  const user = useUser()
  const isLoggedin = computed(() => user.value !== null)
  const fetchUser = async () => {
    const userData = await $fetch<Discord.ClientUser>(getApiUrl('/users/me'))
    if (userData) {
      user.value = userData
    }
  }

  return {
    user,
    isLoggedin,
    fetchUser,
  }
}
