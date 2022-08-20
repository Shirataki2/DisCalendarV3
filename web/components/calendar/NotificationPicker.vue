<script lang="ts" setup>
import { mdiClose, mdiChevronDown } from '@mdi/js'

type Props = {
  modelValue?: number
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: 300,
})

const emit = defineEmits<{
  (e: 'update:modelValue', v: number): void
  (e: 'remove'): void
}>()

const minutes = computed({
  get: () => props.modelValue,
  set: v => {
    emit('update:modelValue', v)
  },
})

if (minutes.value > 24 * 40320) {
  minutes.value = 24 * 40320
}

type Unit = {
  label: string
  multiplier: number
}

const units: Unit[] = [
  { label: '分前', multiplier: 1 },
  { label: '時間前', multiplier: 60 },
  { label: '日前', multiplier: 1440 },
  { label: '週間前', multiplier: 10080 },
  { label: 'ヶ月前', multiplier: 40320 },
]

const unit = ref(units[4])
const value = ref(1)

onMounted(() => {
  for (let i = 1; i < units.length; i++) {
    if (minutes.value < units[i].multiplier * 3) {
      unit.value = units[i - 1]
      value.value = Math.floor(minutes.value / units[i - 1].multiplier)
      break
    }
    value.value = Math.floor(minutes.value / units[units.length - 1].multiplier)
  }
})
</script>

<template>
  <v-chip size="large" label variant="tonal" class="pr-3 mr-1">
    <v-btn
      icon
      size="x-small"
      variant="text"
      class="ml-n3"
      @click="$emit('remove')"
    >
      <v-icon>{{ mdiClose }}</v-icon>
    </v-btn>
    <input
      v-model.number="value"
      type="number"
      min="1"
      max="180"
      class="value-input ml-n1"
      @focusout="$emit('update:modelValue', value * unit.multiplier)"
    />
    <v-menu>
      <template #activator="{ props: menuProps }">
        <span v-bind="menuProps">
          {{ unit.label }}
          <v-icon size="15">{{ mdiChevronDown }}</v-icon>
        </span>
      </template>
      <v-list density="compact">
        <v-list-item
          v-for="u of units"
          :key="u.label"
          @click="
            () => {
              unit = u
              $emit('update:modelValue', value * u.multiplier)
            }
          "
        >
          <v-list-item-title>{{ u.label }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>
  </v-chip>
</template>

<style scoped>
.value-input {
  width: 3rem;
  text-align: right;
}
</style>
