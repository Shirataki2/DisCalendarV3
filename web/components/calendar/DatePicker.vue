<script lang="ts" setup>
import Datepicker from '@vuepic/vue-datepicker'
import '@vuepic/vue-datepicker/dist/main.css'
import { useTheme } from 'vuetify'

const { date, formattedDate, setDate } = useDate()
const { current } = useTheme()
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
        <v-btn class="d-inline" v-bind="props" variant="text" size="large">
          {{ formattedDate }}
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
  </div>
</template>

<style scoped></style>
