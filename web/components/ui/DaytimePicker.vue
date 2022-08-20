<script setup lang="ts">
import dayjs from 'dayjs'
import Datepicker from '@vuepic/vue-datepicker'
import '@vuepic/vue-datepicker/dist/main.css'
import { useTheme } from 'vuetify'
export type Rule = (v: string) => true | string

const { current } = useTheme()

const props = withDefaults(
  defineProps<{
    modelValue: Date
    label: string | undefined
    prependIcon: string | undefined
    rules: Rule[]
    allday?: boolean
  }>(),
  {
    modelValue: () => new Date(),
    label: undefined,
    prependIcon: undefined,
    rules: () => [],
    allday: false,
  }
)

const emit = defineEmits<{
  (e: 'update:modelValue', v: Date): void
}>()

const show = ref(false)

const dateModel = computed({
  get: () => props.modelValue,
  set: v => {
    emit('update:modelValue', v)
  },
})
</script>

<template>
  <v-menu
    ref="menu"
    v-model="show"
    :close-on-back="true"
    :close-on-content-click="false"
    transition="scale-transition"
    class="date-picker"
    min-width="260px"
  >
    <template #activator="{ props: menuProps }">
      <v-text-field
        class="daytime"
        :model-value="
          dayjs(dateModel).format(`YYYY/MM/DD${allday ? '' : ' HH:mm'}`)
        "
        density="comfortable"
        variant="underlined"
        :label="label"
        :prepend-icon="prependIcon"
        readonly
        v-bind="menuProps"
        hide-details="auto"
        :rules="rules"
        @update:model-value="d => (dateModel = d)"
      ></v-text-field>
    </template>
    <div style="width: 260px">
      <Datepicker
        v-model="dateModel"
        :dark="current.dark"
        inline
        auto-apply
        locale="ja"
        week-start="0"
        alt-position
        :enable-time-picker="!allday"
        @closed="show = false"
      />
    </div>
  </v-menu>
</template>

<style scoped lang="scss">
.date-picker > * {
  min-width: 260px !important;
}
</style>
