<script lang="ts" setup>
import VueCal from 'vue-cal'
import { mdiPlus } from '@mdi/js'

const { height } = useWindowSize()
const { date, setDate, view: dateView } = useDate()

const calendarHeight = computed(() => {
  return `${height.value - 137}px`
})

const events = [
  {
    start: '2022-07-31 12:00',
    end: '2022-07-31 18:00',
    title: 'Event 1',
    class: 'red',
    content: 'Test',
  },
]

const viewChange = ({
  startDate,
  endDate,
}: {
  startDate: Date
  endDate: Date
}) => {
  const now = new Date()
  const isIncludesNow =
    startDate.getTime() <= now.getTime() && endDate.getTime() >= now.getTime()
  if (isIncludesNow) {
    setDate(now)
  } else {
    setDate(startDate)
  }
}

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
</script>

<template>
  <div style="height: 100%">
    <v-container>
      <v-row>
        <v-col lg="10" offset-lg="1" xxl="8" offset-xxl="2">
          <v-card class="pa-2">
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
              @view-change="viewChange($event)"
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
            <v-tooltip location="left">
              <template #activator="{ props }">
                <v-btn
                  :icon="mdiPlus"
                  color="primary"
                  size="x-large"
                  class="floating-btn"
                  v-bind="props"
                >
                </v-btn>
              </template>
              新規作成
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
</style>
