import { getApiUrl } from '@/server'

export type DiscordEventCreate = {
  kind: 'stage_channel' | 'voice_channel' | 'somewhere_else'
  place: string
  cover?: string
  create_invitation: boolean
}

export type EventCreate = {
  name: string
  description?: string
  notifications?: string
  is_all_day: boolean
  start_at: string
  end_at: string
  color?: string
  notify_channel_id?: string
  discord_event?: DiscordEventCreate
}

export type EventCreatedResponse = {
  id?: string
  invitation_url?: string
}

export const useApi = () => {
  const createEvent = async (guildId: string, event: EventCreate) => {
    const eventData = await $fetch<EventCreatedResponse>(
      getApiUrl(`/guilds/${guildId}/events`),
      {
        method: 'POST',
        body: JSON.stringify(event),
      }
    )
    return eventData
  }
  return {
    createEvent,
  }
}
