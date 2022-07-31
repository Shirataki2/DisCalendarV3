import { Discord } from '@/types'

export const useUser = () => {
  const user = useState<Discord.ClientUser | null>('user')
  const userGuilds = useState<Discord.InviteGuildResponse | null>('user.guilds')
  return {
    user,
    userGuilds,
  }
}
