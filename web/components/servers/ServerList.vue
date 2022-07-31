<script lang="ts" setup>
import { Discord } from '@/types'
const { userGuilds } = useAuth()
const getIconUrl = (guild: Discord.GuildInfo) => {
  return Discord.getIconUrl(guild)
}

const inviteGuild = (_guild: Discord.GuildInfo) => {
  // TODO: Open invitation dialog
}
</script>

<template>
  <v-list>
    <v-list-item
      v-for="guild in userGuilds?.invited"
      :key="guild.id"
      :to="`/servers/${guild.id}`"
      :title="guild.name"
    >
      <template #prepend>
        <v-avatar class="mr-4">
          <v-img
            :src="getIconUrl(guild)"
            lazy-src="/images/default-guild-icon.png"
          />
        </v-avatar>
      </template>
      <template #append>
        <v-btn
          variant="text"
          color="success"
          to="/servers/${guild.id}"
          class="mr-2"
        >
          Go
        </v-btn>
      </template>
    </v-list-item>
    <div class="guild-hr my-3" />
    <v-list-item
      v-for="guild in userGuilds?.invitable"
      :key="guild.id"
      :title="guild.name"
      @click="inviteGuild(guild)"
    >
      <template #prepend>
        <v-avatar class="mr-4">
          <v-img
            :src="getIconUrl(guild)"
            lazy-src="/images/default-guild-icon.png"
          />
        </v-avatar>
      </template>
      <template #append>
        <v-btn
          variant="text"
          color="secondary"
          to="/servers/${guild.id}"
          class="mr-2"
        >
          招待
        </v-btn>
      </template>
    </v-list-item>
  </v-list>
</template>
