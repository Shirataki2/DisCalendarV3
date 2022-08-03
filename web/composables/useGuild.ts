import { Discord } from '@/types'
import { getApiUrl } from '@/server'

export const useGuild = (id: string) => {
  const channels = useState<Discord.Channel[]>(`guild:${id}/channels`, () => [])

  const fetchChannels = async () => {
    const channelsData = await $fetch<Discord.Channel[]>(
      getApiUrl(`/guilds/${id}/channels`)
    )
    channels.value = channelsData
  }

  return {
    channels,
    fetchChannels,
  }
}
