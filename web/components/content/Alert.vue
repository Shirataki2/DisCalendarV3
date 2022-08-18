<!-- eslint-disable vue/multi-word-component-names -->
<script lang="ts" setup>
import {
  mdiInformation,
  mdiCheckCircle,
  mdiAlertCircle,
  mdiCloseCircle,
} from '@mdi/js'
type Props = {
  type?: 'success' | 'info' | 'warning' | 'error'
  density?: 'compact' | 'comfortable' | 'default'
  variant?: 'flat' | 'outlined' | 'text' | 'elevated' | 'tonal' | 'plain'
  prominent?: boolean
  border?: boolean | string
}

const props = withDefaults(defineProps<Props>(), {
  type: 'success',
  density: 'default',
  variant: 'tonal',
  prominent: false,
  border: false,
})

const icon = computed(() => {
  switch (props.type) {
    case 'success':
      return mdiCheckCircle
    case 'info':
      return mdiInformation
    case 'warning':
      return mdiAlertCircle
    case 'error':
      return mdiCloseCircle
  }
})
</script>

<template>
  <v-alert
    :color="props.type"
    :density="props.density"
    :variant="props.variant"
    :prominent="props.prominent"
    :border="props.border"
    :icon="icon"
  >
    <slot />
  </v-alert>
</template>

<style scoped></style>
