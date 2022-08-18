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

const { date } = useDate()
const { showSnackbar, message: snackMessage, color } = useSnackbar()

const errorMessages = ref<string[]>([])

const form = ref()

const valid = ref(false)
const sending = ref(false)
const completed = ref(false)

const title = ref<string>('')
const description = ref<string>('')
const startDate = ref<Date>(dayjs().startOf('hour').add(1, 'hour').toDate())
const endDate = ref<Date>(dayjs().startOf('hour').add(2, 'hour').toDate())
const allDay = ref<boolean>(false)
const notifyOnCreate = ref<boolean>(false)
const createDiscordEvent = ref<boolean>(false)
const createInvitationUrl = ref<boolean>(false)
const place = ref<EventPlace>(EventPlace.StageChannel)
const selectedStageChannel = ref<string>('')
const selectedVoiceChannel = ref<string>('')
const inputtedSomewhereElse = ref<string>('')
const coverImageFile = ref<File[]>([] as File[])

const invitationUrl = ref()

const route = useRoute()
const guildId = typeof route.params.id === 'string' ? route.params.id : ''
const { channels, fetchChannels } = useGuild(guildId)
const { createEvent } = useApi()
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

const onFocus = (ev: FocusEvent) => {
  const el = ev.target
  if (el instanceof HTMLInputElement) {
    el.select()
  }
}

onMounted(async () => {
  await fetchChannels()
})

const submit = async () => {
  if (!validate()) {
    return
  }
  sending.value = true
  let kind: 'stage_channel' | 'voice_channel' | 'somewhere_else' =
    'stage_channel'
  let placeName = ''
  let createInvitation = false
  switch (place.value) {
    case EventPlace.StageChannel:
      kind = 'stage_channel'
      placeName = selectedStageChannel.value
      createInvitation = createInvitationUrl.value
      break
    case EventPlace.VoiceChannel:
      kind = 'voice_channel'
      placeName = selectedVoiceChannel.value
      createInvitation = createInvitationUrl.value
      break
    case EventPlace.SomewhereElse:
      kind = 'somewhere_else'
      placeName = inputtedSomewhereElse.value
      break
  }
  const coverImage = coverImageFile.value ? coverImageFile.value[0] : null
  let imageBinary: string | undefined
  if (coverImage) {
    const reader = new FileReader()
    reader.readAsDataURL(coverImage)
    await new Promise<void>(resolve => {
      reader.onload = () => {
        imageBinary = reader.result as string
        resolve()
      }
    })
  }
  // eslint-disable-next-line camelcase
  const discord_event = createDiscordEvent.value
    ? {
        kind,
        place: placeName,
        create_invitation: createInvitation,
        cover: imageBinary,
      }
    : undefined
  const ev = await createEvent(guildId, {
    name: title.value,
    description: description.value,
    color: '#0099ff', // TODO: ユーザーによって変更できるようにする
    notifications: '[]', // TODO: Notifications
    start_at: dayjs(startDate.value).format('YYYY-MM-DDTHH:mm:ss'),
    end_at: dayjs(endDate.value).format('YYYY-MM-DDTHH:mm:ss'),
    is_all_day: allDay.value,
    notify_channel_id: undefined, // TODO: Notifications
    // eslint-disable-next-line camelcase
    discord_event,
  })
  // TODO: テスト用なので後で削除
  const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))
  await sleep(2000)
  invitationUrl.value = ev.invitation_url
  sending.value = false
  show.value = false
  showSnackbar.value = true
  snackMessage.value = 'イベントを作成しました'
  color.value = 'success'
  completed.value = true
}

const validate = () => {
  form.value.validate()
  errorMessages.value = []
  if ((title.value?.length ?? 0) === 0) {
    errorMessages.value.push('タイトルを入力してください')
  }
  if ((title.value?.length ?? 0) > 30) {
    errorMessages.value.push('タイトルは30文字以内で入力してください')
  }
  if (description.value?.length ?? 0 > 1000) {
    errorMessages.value.push('説明は1000文字以内で入力してください')
  }
  const now = dayjs()
  const start = dayjs(startDate.value)
  const end = dayjs(endDate.value)
  if (start.isBefore(now)) {
    errorMessages.value.push('開始時間は現在時刻以降で入力してください')
  }
  if (end.isBefore(start)) {
    errorMessages.value.push('終了時間は開始時間以降で入力してください')
  }
  if (end.isBefore(now)) {
    errorMessages.value.push('終了時間は現在時刻以降で入力してください')
  }
  const discordFlag = createDiscordEvent.value
  if (discordFlag) {
    if (place.value === EventPlace.StageChannel) {
      if (!selectedStageChannel.value) {
        errorMessages.value.push('ステージチャンネルを選択してください')
      }
    } else if (place.value === EventPlace.VoiceChannel) {
      if (!selectedVoiceChannel.value) {
        errorMessages.value.push('ボイスチャンネルを選択してください')
      }
    } else if (place.value === EventPlace.SomewhereElse) {
      if (!inputtedSomewhereElse.value) {
        errorMessages.value.push('場所を入力してください')
      }
    }
  }
  const coverImage = coverImageFile.value?.[0]
  if (coverImage) {
    if (coverImage.size > 8 * 1024 * 1024) {
      errorMessages.value.push('カバー画像は8MB以下で入力してください')
    }
  }
  return errorMessages.value.length === 0
}

watch(date, () => {
  const dt = date.value
  dt.setHours(dayjs().hour())
  startDate.value = dayjs(dt).startOf('hour').add(1, 'hour').toDate()
  endDate.value = dayjs(dt).startOf('hour').add(2, 'hour').toDate()
})
</script>

<template>
  <div>
    <v-dialog v-model="show" persistent>
      <v-card max-width="950px" :width="`${width * 0.94}px`">
        <v-card-title>
          <span class="headline"> 新規イベント作成 </span>
        </v-card-title>
        <v-card-text>
          <v-alert
            v-if="errorMessages.length > 0"
            type="error"
            :elevation="1"
            class="mb-2"
          >
            <ul>
              <li v-for="message in errorMessages" :key="message">
                {{ message }}
              </li>
            </ul>
          </v-alert>
          <v-form ref="form" v-model="valid" @submit.prevent="submit">
            <v-container>
              <v-row>
                <v-col cols="12" class="mb-n8">
                  <v-text-field
                    v-model="title"
                    density="comfortable"
                    variant="underlined"
                    label="イベント名*"
                    :disabled="sending"
                    :counter="30"
                    :rules="[rules.required, rules.max(30)]"
                  />
                </v-col>
                <v-col cols="6">
                  <v-checkbox
                    v-model="allDay"
                    label="終日"
                    :disabled="sending"
                    class="ml-n2"
                    hide-details="auto"
                  ></v-checkbox>
                </v-col>
                <v-col cols="6">
                  <v-checkbox
                    v-model="notifyOnCreate"
                    class="ml-n2"
                    :disabled="sending"
                    label="作成時に通知"
                    hide-details="auto"
                  ></v-checkbox>
                </v-col>
                <v-col cols="12" sm="6">
                  <DaytimePicker
                    v-model="startDate"
                    label="開始時間*"
                    :disabled="sending"
                    :prepend-icon="mdiCalendar"
                    :rules="[rules.required]"
                    class="mt-4"
                  />
                </v-col>
                <v-col cols="12" sm="6">
                  <DaytimePicker
                    v-model="endDate"
                    label="終了時間*"
                    :disabled="sending"
                    :prepend-icon="mdiCalendar"
                    :rules="[rules.required]"
                    class="mt-2"
                  />
                </v-col>
                <v-col cols="12" class="mb-n3">
                  <v-textarea
                    v-model="description"
                    label="内容"
                    :disabled="sending"
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
                          :disabled="sending"
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
                          :class="
                            !createDiscordEvent ? 'pointer-disabled' : null
                          "
                        >
                          <v-radio-group
                            v-model="place"
                            label="イベントの場所"
                            class="text-headline"
                            :disabled="!createDiscordEvent || sending"
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
                            :disabled="!createDiscordEvent || sending"
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
                            :disabled="!createDiscordEvent || sending"
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
                            :rules="[rules.required, rules.max(30)]"
                            :disabled="!createDiscordEvent || sending"
                            :prepend-icon="mdiMapMarker"
                            hide-details
                            size="confortable"
                            variant="underlined"
                            placeholder="その他の場所を入力"
                            :counter="30"
                          ></v-text-field>
                          <v-checkbox
                            v-if="place !== EventPlace.SomewhereElse"
                            v-model="createInvitationUrl"
                            class="ml-n2"
                            label="招待URLを作成する"
                            :disabled="!createDiscordEvent || sending"
                            hide-details="auto"
                          ></v-checkbox>
                          <v-file-input
                            v-model="coverImageFile"
                            class="mt-5"
                            :disabled="!createDiscordEvent || sending"
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
          <v-btn color="blue darken-1" text @click="submit"> 送信 </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <v-dialog v-model="completed">
      <v-card :width="`${width * 0.94}px`" max-width="500px">
        <v-card-title class="headline"> 送信完了 </v-card-title>
        <v-card-text>
          <p>イベントを作成しました。</p>
          <v-text-field
            v-if="invitationUrl"
            id="url"
            v-model="invitationUrl"
            class="mt-5"
            readonly
            hide-details
            variant="outlined"
            size="compact"
            label="招待URL"
            @focus="onFocus($event)"
          ></v-text-field>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn color="blue darken-1" text @click="completed = false">
            OK
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<style scoped></style>
