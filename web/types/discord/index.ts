export type { AuthData, AuthorizePayload } from './auth'
export type { ClientUser } from 'discord.js'

export type GuildInfo = {
  id: string
  icon?: string
  name: string
  owner: boolean
  permissions: string
}

export type InviteGuildResponse = {
  invited: Array<GuildInfo>
  invitable: Array<GuildInfo>
  others: Array<GuildInfo>
}

export const getIconUrl = (guild: GuildInfo) => {
  if (guild.icon) {
    return `https://cdn.discordapp.com/icons/${guild.id}/${guild.icon}.png`
  }
  return '/images/default-guild-icon.png'
}
