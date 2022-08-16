<script lang="ts" setup>
import Datepicker from '@vuepic/vue-datepicker'
import '@vuepic/vue-datepicker/dist/main.css'
import { useTheme } from 'vuetify'
import { DateView } from '@/composables/useDate'

const { date, formattedDay, setDate, view } = useDate()
const { current } = useTheme()

const setView = (v: DateView) => {
  view.value = v
}

const viewToLabel = (v: DateView) => {
  switch (v) {
    case 'years':
      return '複数年'
    case 'year':
      return '年間'
    case 'month':
      return '月間'
    case 'week':
      return '週間'
    case 'day':
      return '日間'
  }
}
</script>

<template>
  <div>
    <v-btn
      size="small"
      class="d-inline"
      variant="outlined"
      @click="setDate(new Date())"
    >
      今日
    </v-btn>
    <v-menu location="bottom">
      <template #activator="{ props }">
        <v-btn class="d-inline px-n4" v-bind="props" variant="text">
          {{ formattedDay }}
        </v-btn>
      </template>
      <Datepicker
        inline
        auto-apply
        locale="ja"
        :dark="current.dark"
        :value="date"
        :enable-time-picker="false"
        week-start="0"
        @update:modelValue="d => setDate(d)"
      />
    </v-menu>
    <v-menu location="bottom">
      <template #activator="{ props }">
        <v-btn class="d-inline" v-bind="props" variant="outlined" size="small">
          {{ viewToLabel(view) }}
        </v-btn>
      </template>
      <v-list>
        <v-list-item
          v-for="v in DateView"
          :key="v.toString()"
          @click="setView(v)"
        >
          <v-list-item-title>{{ viewToLabel(v) }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>
  </div>
</template>

<style scoped></style>
