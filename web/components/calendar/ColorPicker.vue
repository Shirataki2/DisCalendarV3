<script lang="ts" setup>
import { mdiSquareRounded } from '@mdi/js'

type Prop = {
  modelValue?: string
  disabled?: boolean
  label?: string
}

const props = withDefaults(defineProps<Prop>(), {
  modelValue: '#FF0000',
  disabled: false,
  label: '色の選択',
})

const emit = defineEmits<{
  (e: 'update:modelValue', v: string): void
}>()

const color = computed({
  get: () => props.modelValue,
  set: v => {
    emit('update:modelValue', v)
  },
})

const show = ref(false)
const modes = ['rgb', 'hsl', 'hex']
const mode = ref(modes[0])

const swatches = [
  ['#FFCCCC', '#FFECC8', '#FFFDC8', '#D4FDC8', '#CCFDF2', '#CCCAFB', '#EECAFB'],
  ['#FF6666', '#FFCB64', '#FFFE64', '#7FFE64', '#66FEE3', '#6665FD', '#CC65FD'],
  ['#FF0000', '#FFAA00', '#FFFF00', '#2AFF00', '#00FFD4', '#0000FF', '#AA00FF'],
  ['#CC0000', '#CC8800', '#CCCC00', '#22CC00', '#00CCAA', '#0000CC', '#8800CC'],
  ['#660000', '#664400', '#666600', '#116600', '#006655', '#000066', '#440066'],
  ['#FFFFFF'],
  ['#B8B8B8'],
  ['#7F7F7F'],
  ['#474747'],
  ['#000000'],
]
</script>

<template>
  <!-- eslint-disable vue/no-v-model-argument -->
  <div>
    <v-menu v-model="show" :close-on-content-click="false" min-width="260px">
      <template #activator="{ props: menuProps }">
        <v-text-field
          class="color"
          :model-value="color"
          :label="label"
          density="comfortable"
          variant="underlined"
          readonly
          v-bind="menuProps"
          hide-details="auto"
          :disabled="props.disabled"
        >
          <template #prepend>
            <v-icon :color="color">{{ mdiSquareRounded }}</v-icon>
          </template>
        </v-text-field>
      </template>
      <v-card>
        <v-color-picker
          v-model="color"
          v-model:mode="mode"
          elevation="0"
          :modes="modes"
          :swatches="swatches"
          show-swatches
        ></v-color-picker>
        <v-card-actions>
          <v-spacer />
          <v-btn color="secondary" text @click="show = false"> 決定 </v-btn>
        </v-card-actions>
      </v-card>
    </v-menu>
  </div>
</template>

<style scoped lang="scss"></style>
