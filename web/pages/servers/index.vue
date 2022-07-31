<script lang="ts" setup>
import LoadingIndicator from '@/components/LoadingIndicator.vue'
import ServerList from '@/components/servers/ServerList.vue'
const { loading, setLoading } = useLoading()
const { fetchUserGuilds } = useAuth()

definePageMeta({
  middleware: ['auth'],
})

onMounted(async () => {
  setLoading(true)
  await fetchUserGuilds()
  setLoading(false)
})
</script>

<template>
  <div>
    <Head>
      <title>Servers - DisCalendar v3</title>
    </Head>
    <v-container>
      <v-row>
        <v-col
          cols="12"
          sm="10"
          md="8"
          lg="6"
          xl="4"
          offset-sm="1"
          offset-md="2"
          offset-lg="3"
          offset-xl="4"
        >
          <v-card class="pa-3">
            <v-card-title class="align-center">
              <strong> サーバーを選択してください </strong>
            </v-card-title>
            <v-card-text>
              <LoadingIndicator
                v-if="loading"
                :size="120"
                :thickness="2"
                :duration="0.7"
              />
              <ServerList v-else />
            </v-card-text>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>

<style scoped>
.guild-hr {
  border-top: 1px solid rgba(71, 71, 71, 0.404);
}
</style>
