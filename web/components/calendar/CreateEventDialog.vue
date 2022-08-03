<script lang="ts" setup>
import { mdiCalendar, mdiVolumeHigh, mdiMapMarker } from '@mdi/js'
import dayjs from 'dayjs'
import { Discord } from '@/types'
import DaytimePicker from '@/components/ui/DaytimePicker.vue'

enum EventPlace {
  StageChannel = 'stage_channel',
  VoiceChannel = 'voice_channel',
  SomewhereElse = 'somewhere_else',
}

const form = ref()

const title = ref<string>()
const description = ref<string>()
const startDate = ref<Date>(dayjs().startOf('hour').add(1, 'hour').toDate())
const endDate = ref<Date>(dayjs().startOf('hour').add(2, 'hour').toDate())
const allDay = ref<boolean>()
const notifyOnCreate = ref<boolean>()
const createDiscordEvent = ref<boolean>()
const place = ref<EventPlace>(EventPlace.StageChannel)
const selectedStageChannel = ref<string>()
const selectedVoiceChannel = ref<string>()
const inputtedSomewhereElse = ref<string>()
const coverImageFile = ref<File[]>()

const route = useRoute()
const channelId = typeof route.params.id === 'string' ? route.params.id : ''
const { channels, fetchChannels } = useGuild(channelId)
const stageChannels = computed(() => {
  // 13: Stage Channel
  return channels.value.filter(channel => channel.type === 13)
})
const voiceChannels = computed(() => {
  // 2: Voice Channel
  return channels.value.filter(channel => channel.type === 2)
})
// const somewhereElse = ref<string>()

const { width } = useWindowSize()

const props = withDefaults(
  defineProps<{
    modelValue: boolean
  }>(),
  {
    modelValue: false,
  }
)

const emit = defineEmits<{
  (e: 'update:modelValue', v: boolean): void
}>()

const show = computed({
  get: () => props.modelValue,
  set: v => {
    emit('update:modelValue', v)
  },
})

const rules = {
  required: (v: string) => !!v || 'この項目は必須です',
  min: (min: number) => (v: string) =>
    v.length >= min || `${min}文字以上で入力してください`,
  max: (max: number) => (v: string) =>
    v.length <= max || `${max}文字以下で入力してください`,
  maxSize:
    (max: number, unit: 'KB' | 'MB' | 'GB' | 'KiB' | 'MiB' | 'GiB') =>
    (v: File[]) => {
      const size = v.reduce((acc, file) => acc + file.size, 0)
      let unitMultiplier = 1
      switch (unit) {
        case 'KiB':
          unitMultiplier = 1024
          break
        case 'MiB':
          unitMultiplier = 1024 * 1024
          break
        case 'GiB':
          unitMultiplier = 1024 * 1024 * 1024
          break
        case 'KB':
          unitMultiplier = 1000
          break
        case 'MB':
          unitMultiplier = 1000 * 1000
          break
        case 'GB':
          unitMultiplier = 1000 * 1000 * 1000
          break
      }
      const maxSize = max * unitMultiplier
      return (
        size <= maxSize || `${max}${unit}以上のファイルはアップロードできません`
      )
    },
}

onMounted(async () => {
  await fetchChannels()
})
</script>

<template>
  <v-dialog v-model="show" persistent>
    <v-card max-width="950px" :width="`${width * 0.94}px`">
      <v-card-title>
        <span class="headline"> 新規イベント作成 </span>
      </v-card-title>
      <v-card-text>
        <v-form ref="form">
          <v-container>
            <v-row>
              <v-col cols="12" class="mb-n8">
                <v-text-field
                  v-model="title"
                  density="comfortable"
                  variant="underlined"
                  label="イベント名*"
                  :counter="30"
                  :rules="[rules.required, rules.max(30)]"
                />
              </v-col>
              <v-col cols="6">
                <v-checkbox
                  v-model="allDay"
                  label="終日"
                  class="ml-n2"
                  hide-details="auto"
                ></v-checkbox>
              </v-col>
              <v-col cols="6">
                <v-checkbox
                  v-model="notifyOnCreate"
                  class="ml-n2"
                  label="作成時に通知"
                  hide-details="auto"
                ></v-checkbox>
              </v-col>
              <v-col cols="12" sm="6">
                <DaytimePicker
                  v-model="startDate"
                  label="開始時間*"
                  :prepend-icon="mdiCalendar"
                  :rules="[rules.required]"
                />
              </v-col>
              <v-col cols="12" sm="6">
                <DaytimePicker
                  v-model="endDate"
                  label="終了時間*"
                  :prepend-icon="mdiCalendar"
                  :rules="[rules.required]"
                />
              </v-col>
              <v-col cols="12" class="mb-n3">
                <v-textarea
                  v-model="description"
                  label="内容"
                  :rules="[rules.max(1000)]"
                  :counter="1000"
                  variant="underlined"
                ></v-textarea>
              </v-col>
              <v-col cols="12">
                <v-expansion-panels variant="default">
                  <v-expansion-panel>
                    <v-expansion-panel-title>
                      Discordのイベントに追加する
                    </v-expansion-panel-title>
                    <v-expansion-panel-text>
                      <v-checkbox
                        v-model="createDiscordEvent"
                        class="ml-n2"
                        label="有効にする"
                        hide-details="auto"
                      ></v-checkbox>
                      <p class="small-note pa-2">
                        この機能を正常に機能させるためには、
                        Discordのサーバーの設定からコミュニティの有効化を行う必要があります。
                      </p>
                      <p class="small-note px-2 pb-8">
                        詳しくは
                        <a
                          href="https://support.discord.com/hc/en-us/articles/360047132851"
                          target="_blank"
                        >
                          こちら
                        </a>
                        をご覧ください。
                      </p>
                      <div
                        :class="!createDiscordEvent ? 'pointer-disabled' : null"
                      >
                        <v-radio-group
                          v-model="place"
                          label="イベントの場所"
                          class="text-headline"
                          :disabled="!createDiscordEvent"
                          inline
                          hide-details="auto"
                        >
                          <v-radio
                            :value="EventPlace.StageChannel"
                            label="ステージチャンネル"
                            hide-details="auto"
                          ></v-radio>
                          <v-radio
                            :value="EventPlace.VoiceChannel"
                            label="ボイスチャンネル"
                            hide-details="auto"
                          ></v-radio>
                          <v-radio
                            :value="EventPlace.SomewhereElse"
                            label="その他"
                            hide-details="auto"
                          ></v-radio>
                        </v-radio-group>
                        <v-select
                          v-if="place === EventPlace.StageChannel"
                          v-model="selectedStageChannel"
                          :items="stageChannels"
                          :rules="[rules.required]"
                          :disabled="!createDiscordEvent"
                          :prepend-icon="Discord.stageChannelIconSvg"
                          hide-details
                          size="confortable"
                          title="name"
                          variant="underlined"
                          label="ステージチャンネルを選択"
                        ></v-select>
                        <v-select
                          v-else-if="place === EventPlace.VoiceChannel"
                          v-model="selectedVoiceChannel"
                          :items="voiceChannels"
                          :rules="[rules.required]"
                          :disabled="!createDiscordEvent"
                          :prepend-icon="mdiVolumeHigh"
                          hide-details
                          item-title="name"
                          item-value="id"
                          size="confortable"
                          variant="underlined"
                          label="ボイスチャンネルを選択"
                        ></v-select>
                        <v-text-field
                          v-else-if="place === EventPlace.SomewhereElse"
                          v-model="inputtedSomewhereElse"
                          :rules="[rules.required]"
                          :disabled="!createDiscordEvent"
                          :prepend-icon="mdiMapMarker"
                          hide-details
                          size="confortable"
                          variant="underlined"
                          placeholder="その他の場所を入力"
                        ></v-text-field>
                        <v-file-input
                          v-model="coverImageFile"
                          class="mt-5"
                          :disabled="!createDiscordEvent"
                          variant="underlined"
                          label="イベントのカバー画像 (8MBまでのJPEG/PNG/GIF)"
                          :rules="[rules.maxSize(8, 'MB')]"
                          accept="image/jpeg,image/png,image/gif"
                          show-size
                        ></v-file-input>
                      </div>
                    </v-expansion-panel-text>
                  </v-expansion-panel>
                </v-expansion-panels>
              </v-col>
            </v-row>
          </v-container>
        </v-form>
      </v-card-text>
      <v-card-actions>
        <v-spacer />
        <v-btn color="blue darken-1" text @click="show = false">
          キャンセル
        </v-btn>
        <v-btn color="blue darken-1" text @click="show = false"> 保存 </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<style scoped></style>
