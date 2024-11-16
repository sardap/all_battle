<script setup lang="ts">
import { mon_to_anime_front_pic_path, type Mon } from '@/gen_3'
import { ref } from 'vue'

defineProps<{
  mon: Mon
}>()

const refresh = ref(0)

function custom_style(mon: Mon) {
  return `background-image: url('` + mon_to_anime_front_pic_path(mon) + `');`
}
</script>

<template>
  <div class="element" :key="refresh">
    <div class="sprite" :style="custom_style(mon)" @click="() => refresh++"></div>
  </div>
</template>

<style scoped lang="scss">
.element {
  max-width: fit-content;
  margin-inline: auto;
}

.sprite {
  height: 64px;
  width: 64px;
  background-repeat: no-repeat;
  animation-name: play;
  animation-timing-function: steps(2);
  animation-duration: 1s;
  animation-iteration-count: 3;
}

@keyframes play {
  0% {
    background-position: 0 0;
  }
  100% {
    background-position: 0 -128px;
  }
}
</style>
