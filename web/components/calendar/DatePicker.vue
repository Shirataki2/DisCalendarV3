<script lang="ts" setup>
import Datepicker from '@vuepic/vue-datepicker'
import '@vuepic/vue-datepicker/dist/main.css'

const { date, formattedDate, setDate } = useDate()
</script>

<template>
  <div>
    <v-menu location="bottom">
      <template #activator="{ props }">
        <v-btn v-bind="props" variant="text" size="large">
          {{ formattedDate }}
        </v-btn>
      </template>
      <Datepicker
        inline
        auto-apply
        locale="ja"
        :value="date"
        week-start="0"
        @update:modelValue="d => setDate(d)"
      />
    </v-menu>
  </div>
</template>

<style scoped></style>
