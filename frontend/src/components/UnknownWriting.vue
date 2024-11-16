<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  message: string
}>()

function unknown_entry(letter: string) {
  letter = letter.toLowerCase()
  if (letter === `?`) {
    letter = `question_mark`
  } else if (letter === `!` || letter === `.`) {
    letter = `exclamation_mark`
  } else if ('abcdefghijklmnopqrstuvwxyz'.indexOf(letter) === -1) {
    return `data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAMAAACdt4HsAAAAAXNSR0IArs4c6QAAADBQTFRFzc2s/wD//wD//wD/3t7e////EBAQpKSkc3NzUlJS/wD//wD//wD//wD//wD//wD/kw319QAAABB0Uk5TAP///////////////////8BQi0MAAAAbSURBVFiF7cGBAAAAAMOg+VPf4ARVAQAAAHwDEEAAAeK3i+QAAAAASUVORK5CYII=`
  }

  return `/pokemon/unown/${letter}/front.png`
}

const message = ref(props.message.split('').map((x) => unknown_entry(x)))
</script>

<template>
  <div class="row-container flex-wrap">
    <div v-for="(letter, i) in message" :key="i">
      <img :src="letter" />
    </div>
  </div>
</template>

<style scoped>
img {
  width: 64x;
}
.space {
  width: 30px;
}
</style>
