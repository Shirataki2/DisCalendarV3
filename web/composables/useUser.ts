import { Discord } from '@/types'

export const useUser = () => {
  return useState<Discord.ClientUser | null>('user')
}
