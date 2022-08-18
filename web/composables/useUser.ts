import type { Ref } from 'vue'
import { Discord } from '@/types'

export const usePersistedState = <T>(
  identifier: string,
  defaultOptions: T
): Ref<T> => {
  const store = useCookie<T | undefined>(identifier)
  const persistedObject = useState<T>(identifier, (): T => {
    const item = store.value
    if (!item) {
      return defaultOptions
    }
    return item ?? defaultOptions
  })

  watch(
    persistedObject,
    object => {
      store.value = object
    },
    { deep: true }
  )

  return persistedObject
}

export const useUser = () => {
  const user = usePersistedState<Discord.ClientUser | null>('user', null)
  const userGuilds = usePersistedState<Discord.InviteGuildResponse | null>(
    'user.guilds',
    null
  )
  return {
    user,
    userGuilds,
  }
}
