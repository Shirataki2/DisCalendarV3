<script lang="ts" setup>
import { useTheme } from 'vuetify'

type Props = {
  foreground?: boolean
  size?: number
  thickness?: number
  duration?: number
  color?: [number, number, number]
  text?: string
}

const prop = withDefaults(defineProps<Props>(), {
  foreground: true,
  size: 150,
  thickness: 3,
  duration: 1,
  color: () => [64, 246, 225],
  text: 'Loading...',
})

const size = computed(() => `${prop.size}px`)
const dur = computed(() => `${prop.duration}s`)
const padd = computed(() => `${prop.thickness}px`)
const grad = computed(
  () => `linear-gradient(
  0deg,
  rgba(${prop.color.join(', ')}, 0.1) 33%,
  rgba(${prop.color.join(', ')}, 1) 100%
)`
)
const theme = useTheme()
const bgColor = computed(() => {
  const colors = theme.current.value.colors
  return prop.foreground ? colors.surface : colors.background
})
</script>

<template>
  <div class="wrapper">
    <div class="spinner">
      <div class="circle-border">
        <div class="circle-core"></div>
      </div>
    </div>
    <div v-if="text.length" class="text-wrapper">
      <span class="text">{{ text }}</span>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@import url('https://fonts.googleapis.com/css2?family=Roboto+Mono:wght@300&display=swap');

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(359deg);
  }
}

.text {
  font-family: 'Roboto Mono', serif;
  font-size: 0.97rem;
  font-weight: 300;
}

.text-wrapper {
  position: absolute;
  text-align: center;
}

.wrapper {
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 9999;
  background: transparent;
  display: flex;
  justify-content: center;
  align-items: center;
}
.circle {
  &-border {
    width: v-bind(size);
    height: v-bind(size);
    padding: v-bind(padd);
    border-radius: 50%;
    background: v-bind(grad);
    animation: spin v-bind(dur) linear infinite;
  }
  &-core {
    width: 100%;
    height: 100%;
    border-radius: 50%;
    background: v-bind(bgColor);
  }
}
</style>
