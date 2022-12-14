<script lang="ts" setup>
import {
  mdiCalendar,
  mdiVolumeHigh,
  mdiMapMarker,
  mdiBell,
  mdiPlus,
} from '@mdi/js'
import dayjs from 'dayjs'
import { Discord } from '@/types'
import DaytimePicker from '@/components/ui/DaytimePicker.vue'
import ColorPicker from '@/components/calendar/ColorPicker.vue'
import NotificationPicker from '@/components/calendar/NotificationPicker.vue'

enum EventPlace {
  StageChannel = 'stage_channel',
  VoiceChannel = 'voice_channel',
  SomewhereElse = 'somewhere_else',
}

const { date } = useDate()
const { showSnackbar, message: snackMessage, color } = useSnackbar()
const { theme } = useUI()

const iconColor = computed(() => {
  return theme.current.value.dark ? '#999999' : '#7a7a7a'
})

const errorMessages = ref<string[]>([])

const form = ref()

const valid = ref(false)
const sending = ref(false)
const completed = ref(false)

const title = ref<string>('')
const description = ref<string>('')
const eventColor = ref<string>('#FF0000')
const startDate = ref<Date>(dayjs().startOf('hour').add(1, 'hour').toDate())
const endDate = ref<Date>(dayjs().startOf('hour').add(2, 'hour').toDate())
const allDay = ref<boolean>(false)
const createDiscordEvent = ref<boolean>(false)
const createInvitationUrl = ref<boolean>(false)
const place = ref<EventPlace>(EventPlace.StageChannel)
const selectedStageChannel = ref<string>('')
const selectedVoiceChannel = ref<string>('')
const inputtedSomewhereElse = ref<string>('')
const coverImageFile = ref<File[]>([] as File[])
const notificationsArray = ref([60])

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
  required: (v: string) => !!v || '???????????????????????????',
  min: (min: number) => (v: string) =>
    v.length >= min || `${min}???????????????????????????????????????`,
  max: (max: number) => (v: string) =>
    v.length <= max || `${max}???????????????????????????????????????`,
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
        size <= maxSize || `${max}${unit}?????????????????????????????????????????????????????????`
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
    color: color.value,
    notifications: JSON.stringify(notificationsArray.value),
    start_at: dayjs(startDate.value).format('YYYY-MM-DDTHH:mm:ss'),
    end_at: dayjs(endDate.value).format('YYYY-MM-DDTHH:mm:ss'),
    is_all_day: allDay.value,
    notify_channel_id: undefined, // TODO: Notifications
    // eslint-disable-next-line camelcase
    discord_event,
  })
  // TODO: ?????????????????????????????????
  const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))
  await sleep(2000)
  invitationUrl.value = ev.invitation_url
  sending.value = false
  show.value = false
  showSnackbar.value = true
  snackMessage.value = '?????????????????????????????????'
  color.value = 'success'
  completed.value = true
}

const validate = () => {
  form.value.validate()
  errorMessages.value = []
  if ((title.value?.length ?? 0) === 0) {
    errorMessages.value.push('???????????????????????????????????????')
  }
  if ((title.value?.length ?? 0) > 30) {
    errorMessages.value.push('???????????????30???????????????????????????????????????')
  }
  if (description.value?.length ?? 0 > 1000) {
    errorMessages.value.push('?????????1000???????????????????????????????????????')
  }
  const now = dayjs()
  const start = dayjs(startDate.value)
  const end = dayjs(endDate.value)

  const discordFlag = createDiscordEvent.value
  if (end.isBefore(start)) {
    errorMessages.value.push('????????????????????????????????????????????????????????????')
  }
  if (discordFlag) {
    if (start.isBefore(now)) {
      errorMessages.value.push('????????????????????????????????????????????????????????????')
    }
    if (end.isBefore(now)) {
      errorMessages.value.push('????????????????????????????????????????????????????????????')
    }
  }
  if (discordFlag) {
    if (place.value === EventPlace.StageChannel) {
      if (!selectedStageChannel.value) {
        errorMessages.value.push('??????????????????????????????????????????????????????')
      }
    } else if (place.value === EventPlace.VoiceChannel) {
      if (!selectedVoiceChannel.value) {
        errorMessages.value.push('???????????????????????????????????????????????????')
      }
    } else if (place.value === EventPlace.SomewhereElse) {
      if (!inputtedSomewhereElse.value) {
        errorMessages.value.push('?????????????????????????????????')
      }
    }
  }
  const coverImage = coverImageFile.value?.[0]
  if (coverImage) {
    if (coverImage.size > 8 * 1024 * 1024) {
      errorMessages.value.push('??????????????????8MB?????????????????????????????????')
    }
  }
  return errorMessages.value.length === 0
}

const removeNotification = (index: number) => {
  const notifications = notificationsArray.value
  notifications.splice(index, 1)
  notificationsArray.value = notifications
}

const addNotification = () => {
  const notifications = notificationsArray.value
  notifications.push(notificationsArray.value.length * 60 + 60)
  notificationsArray.value = notifications
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
          <span class="headline"> ???????????????????????? </span>
        </v-card-title>
        <v-card-text>
          <v-alert
            v-if="errorMessages.length > 0"
            type="error"
            class="mb-2"
            variant="flat"
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
                    label="???????????????*"
                    :disabled="sending"
                    :counter="30"
                    :rules="[rules.required, rules.max(30)]"
                  />
                </v-col>
                <v-col cols="12" sm="3">
                  <v-checkbox
                    v-model="allDay"
                    label="??????"
                    :disabled="sending"
                    class="ml-n2"
                    hide-details="auto"
                  ></v-checkbox>
                </v-col>
                <v-col cols="12" sm="9">
                  <ColorPicker v-model="eventColor" :disabled="sending" />
                </v-col>
                <v-col cols="12">
                  <v-icon class="mr-4" :color="iconColor">
                    {{ mdiBell }}
                  </v-icon>
                  <NotificationPicker
                    v-for="(ntfn, i) in notificationsArray"
                    :key="ntfn"
                    v-model="notificationsArray[i]"
                    @remove="removeNotification(i)"
                  />
                  <v-btn
                    v-if="notificationsArray.length < 10"
                    icon
                    variant="text"
                    @click="addNotification"
                  >
                    <v-icon>
                      {{ mdiPlus }}
                    </v-icon>
                  </v-btn>
                </v-col>
                <v-col cols="12" sm="6">
                  <DaytimePicker
                    v-model="startDate"
                    label="????????????*"
                    :disabled="sending"
                    :prepend-icon="mdiCalendar"
                    :rules="[rules.required]"
                    :allday="allDay"
                    class="mt-4"
                  />
                </v-col>
                <v-col cols="12" sm="6">
                  <DaytimePicker
                    v-model="endDate"
                    label="????????????*"
                    :disabled="sending"
                    :prepend-icon="mdiCalendar"
                    :rules="[rules.required]"
                    :allday="allDay"
                    class="mt-2"
                  />
                </v-col>
                <v-col cols="12" class="mb-n3">
                  <v-textarea
                    v-model="description"
                    label="??????"
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
                        Discord??????????????????????????????
                      </v-expansion-panel-title>
                      <v-expansion-panel-text>
                        <v-checkbox
                          v-model="createDiscordEvent"
                          class="ml-n2"
                          :disabled="sending"
                          label="???????????????"
                          hide-details="auto"
                        ></v-checkbox>
                        <p class="small-note pa-2">
                          ??????????????????????????????????????????????????????
                          Discord?????????????????????????????????????????????????????????????????????????????????????????????
                        </p>
                        <p class="small-note px-2">
                          ????????????
                          <a
                            href="https://support.discord.com/hc/en-us/articles/360047132851"
                            target="_blank"
                          >
                            ?????????
                          </a>
                          ????????????????????????
                        </p>
                        <p class="small-note px-2 py-5">
                          ?????????Discord???????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????
                        </p>
                        <div
                          :class="
                            !createDiscordEvent ? 'pointer-disabled' : null
                          "
                        >
                          <v-radio-group
                            v-model="place"
                            label="?????????????????????"
                            class="text-headline"
                            :disabled="!createDiscordEvent || sending"
                            inline
                            hide-details="auto"
                          >
                            <v-radio
                              :value="EventPlace.StageChannel"
                              label="???????????????????????????"
                              hide-details="auto"
                            ></v-radio>
                            <v-radio
                              :value="EventPlace.VoiceChannel"
                              label="????????????????????????"
                              hide-details="auto"
                            ></v-radio>
                            <v-radio
                              :value="EventPlace.SomewhereElse"
                              label="?????????"
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
                            label="????????????????????????????????????"
                            no-data-text="???????????????????????????????????????????????????"
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
                            label="?????????????????????????????????"
                            no-data-text="????????????????????????????????????????????????"
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
                            placeholder="???????????????????????????"
                            :counter="30"
                          ></v-text-field>
                          <v-checkbox
                            v-if="place !== EventPlace.SomewhereElse"
                            v-model="createInvitationUrl"
                            class="ml-n2"
                            label="??????URL???????????????"
                            :disabled="!createDiscordEvent || sending"
                            hide-details="auto"
                          ></v-checkbox>
                          <v-file-input
                            v-model="coverImageFile"
                            class="mt-5"
                            :disabled="!createDiscordEvent || sending"
                            variant="underlined"
                            label="?????????????????????????????? (8MB?????????JPEG/PNG/GIF)"
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
            ???????????????
          </v-btn>
          <v-btn color="blue darken-1" text @click="submit"> ?????? </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <v-dialog v-model="completed">
      <v-card :width="`${width * 0.94}px`" max-width="500px">
        <v-card-title class="headline"> ???????????? </v-card-title>
        <v-card-text>
          <p>????????????????????????????????????</p>
          <v-text-field
            v-if="invitationUrl"
            id="url"
            v-model="invitationUrl"
            class="mt-5"
            readonly
            hide-details
            variant="outlined"
            size="compact"
            label="??????URL"
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
