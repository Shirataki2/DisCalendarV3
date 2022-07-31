<script lang="ts" setup>
import { mdiCog } from '@mdi/js'
import { Discord } from '@/types'
const { userGuilds } = useAuth()

const isServerPage = computed(() => {
  const { path } = useRoute()
  return path.match(/^\/servers\/\d+$/) !== null
})

const currentGuild = computed(() => {
  const { path } = useRoute()
  const match = path.match(/^\/servers\/(\d+)$/)
  if (match) {
    const id = match[1]
    return userGuilds.value?.invited.find(guild => guild.id === id)
  }
  return undefined
})
</script>

<template>
  <div v-if="isServerPage && currentGuild">
    <v-card variant="flat" class="mx-n2 mt-n2 mb-3">
      <v-img
        :src="Discord.getIconUrl(currentGuild)"
        cover
        lazy-src="/images/default-guild-icon.png"
        height="90px"
      ></v-img>
      <v-card-title class="text-h6">
        <div>{{ currentGuild.name }}</div>
      </v-card-title>
      <v-card-actions>
        <v-spacer />
        <v-tooltip location="top">
          <template #activator="{ props }">
            <v-btn
              :icon="mdiCog"
              :to="`/servers/${currentGuild.id}/settings`"
              v-bind="props"
            ></v-btn>
          </template>
          <span>設定</span>
        </v-tooltip>
      </v-card-actions>
    </v-card>
    <div class="guild-hr nt-n2 mb-3" />
  </div>
</template>

<style scoped>
.guild-hr {
  border-top: 1px solid rgba(85, 85, 85, 0.6);
}
</style>
