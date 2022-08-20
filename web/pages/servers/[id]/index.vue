<script lang="ts" setup>
import VueCal from 'vue-cal'
import { onBeforeRouteLeave } from 'vue-router'
import { mdiPlus } from '@mdi/js'
import CreateEventDialog from '@/components/calendar/CreateEventDialog.vue'

const { height } = useWindowSize()
const { date, setDate, view: dateView } = useDate()
const { dialog, setDialog } = useDialog()

const calendarHeight = computed(() => {
  return `${height.value - 138}px`
})

const events = ref([
  {
    start: '2022-07-31 12:00',
    end: '2022-07-31 18:00',
    title: 'Event 1',
    class: 'red',
    color: '#078990',
    content: 'Test',
  },
])

// const viewChange = ({
//   startDate,
//   endDate,
// }: {
//   startDate: Date
//   endDate: Date
// }) => {
//   const now = new Date()
//   const isIncludesNow =
//     startDate.getTime() <= now.getTime() && endDate.getTime() >= now.getTime()
//   if (isIncludesNow) {
//     setDate(now)
//   } else {
//     setDate(startDate)
//   }
//   console.log('b')
// }

const dateAttr = (date: string) => {
  switch (date) {
    case '日':
      return 'day-sunday'
    case '土':
      return 'day-saturday'
    default:
      return 'day-weekday'
  }
}

onBeforeRouteLeave((_to, _from) => {
  setDate(new Date())
})

// TODO: 型の定義を書く
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const getStyle = (style: any) => {
  const color = style.color
  const rgb = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(color)
  const r = rgb ? parseInt(rgb[1], 16) : 0
  const g = rgb ? parseInt(rgb[2], 16) : 0
  const b = rgb ? parseInt(rgb[3], 16) : 0
  const luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b
  return {
    backgroundColor: color,
    color: luminance > 160 ? '#000' : '#fff',
  }
}

const { userGuilds } = useAuth()
const { path } = useRoute()

const currentGuild = computed(() => {
  const match = path.match(/^\/servers\/(\d+)$/)
  if (match) {
    const id = match[1]
    return userGuilds.value?.invited.find(guild => guild.id === id)
  }
  return undefined
})

const head = computed(() => {
  const guild = currentGuild.value
  return { title: guild ? guild.name : null }
})

useHead(head)
</script>

<template>
  <div style="height: 100%">
    <v-container>
      <v-row>
        <v-col lg="10" offset-lg="1" xxl="8" offset-xxl="2">
          <v-card class="pa-1">
            <ClientOnly>
              <!-- eslint-disable vue/no-v-model-argument -->
              <vue-cal
                ref="vuecal"
                v-model:active-view="dateView"
                hide-view-selector
                class="calendar"
                locale="ja"
                events-on-month-view="short"
                start-week-on-sunday
                :selected-date="date"
                :events="events"
                :editable-events="{
                  title: false,
                  drag: true,
                  resize: true,
                  delete: true,
                  create: true,
                }"
                :snap-to-time="15"
                :show-allday-events="'short'"
                @cell-focus="setDate($event)"
              >
                <template #title="{ view }">
                  <span v-if="view.id === 'years'">
                    {{ view.startDate.getFullYear() }}年 -
                    {{ view.endDate.getFullYear() }}年
                  </span>
                  <!-- Using Vue Cal injected Date prototypes -->
                  <span v-else-if="view.id === 'year'">{{
                    view.startDate.format('YYYY年')
                  }}</span>
                  <span v-else-if="view.id === 'month'">{{
                    view.startDate.format('YYYY/M')
                  }}</span>
                  <span v-else-if="view.id === 'week'">
                    {{ view.startDate.format('YYYY年') }}
                    {{ view.startDate.format('M/D') }} -
                    {{
                      view.endDate.format('YYYY') ===
                      view.startDate.format('YYYY')
                        ? ''
                        : view.endDate.format('YYYY年')
                    }}
                    {{ view.endDate.format('M/D') }}
                  </span>
                  <span v-else-if="view.id === 'day'">{{
                    view.startDate.format('YYYY年M月D日 (dddd)')
                  }}</span>
                </template>
                <template #event="{ event, view }">
                  <div class="event-content" :style="getStyle(event)">
                    <div class="vuecal__event-title">
                      {{ event.title }}
                    </div>
                    <div v-if="view !== 'month'" class="vuecal__event-time">
                      {{ event.start.format('HH:mm') }} -
                      {{ event.end.format('HH:mm') }}
                    </div>
                  </div>
                </template>
                <template #weekday-heading="{ heading, view }">
                  <span :class="dateAttr(heading['label'])">
                    <span v-if="view.id == 'week'">
                      {{ heading.date.getDate() }}
                    </span>
                    <span v-if="view.id == 'month'">
                      {{ heading.label }}
                    </span>
                  </span>
                </template>
              </vue-cal>
            </ClientOnly>

            <v-tooltip location="left">
              <template #activator="{ props }">
                <v-btn
                  :icon="mdiPlus"
                  color="primary"
                  size="x-large"
                  class="floating-btn"
                  v-bind="props"
                  style="z-index: 1000"
                  @click="setDialog(true)"
                >
                </v-btn>
              </template>
              新規作成
              <CreateEventDialog v-model="dialog" />
            </v-tooltip>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>

<style scoped lang="scss">
.floating-btn {
  position: fixed;
  bottom: 50px;
  right: 20px;
}
.calendar {
  height: v-bind(calendarHeight);
}

.day {
  &-sunday {
    color: #f44336;
  }
  &-saturday {
    color: #2196f3;
  }
}

.vuecal--week-view,
.vuecal--day-view {
  .event-content {
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    bottom: 0;
    box-sizing: border-box;
    padding: 8px;
    border-radius: 6px;
  }
}

.vuecal--month-view {
  .event-content {
    border-radius: 2px;
  }
}
</style>
