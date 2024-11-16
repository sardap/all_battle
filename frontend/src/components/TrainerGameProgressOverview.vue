<script setup lang="ts">
import { type RouteStep } from '@/api'
import { get_trainer_by_idName, type Trainer } from '@/gen_3'
import { onMounted, ref } from 'vue'

const props = defineProps<{
  trainer: Trainer
  steps: RouteStep[]
}>()

function percent_complete(): string {
  if (!props.steps) {
    return ''
  }

  let wins = 0
  while (wins < props.steps.length && props.steps[wins].won) {
    wins++
  }
  return `${Math.round((wins / props.steps.length) * 100)}%`
}

const badges = ref<{ number: string; won: boolean }[]>([])
const elite_four = ref<{ icon: string; won: boolean }[]>([])
const champion = ref<boolean>(false)

onMounted(() => {
  for (let i = 0; i < props.steps.length; i++) {
    let number = null
    switch (props.steps[i].other_trainer_id) {
      case get_trainer_by_idName('ROXANNE_1').id:
        number = '1'
        break
      case get_trainer_by_idName('BRAWLY_1').id:
        number = '2'
        break
      case get_trainer_by_idName('WATTSON_1').id:
        number = '3'
        break
      case get_trainer_by_idName('FLANNERY_1').id:
        number = '4'
        break
      case get_trainer_by_idName('NORMAN_1').id:
        number = '5'
        break
      case get_trainer_by_idName('WINONA_1').id:
        number = '6'
        break
      case get_trainer_by_idName('TATE_AND_LIZA_1').id:
        number = '7'
        break
      case get_trainer_by_idName('JUAN_1').id:
        number = '8'
        break
    }

    if (number) {
      badges.value.push({
        number: number,
        won: props.steps[i].won
      })
      continue
    }

    switch (props.steps[i].other_trainer_id) {
      case get_trainer_by_idName('SIDNEY').id:
        number = 'sidney'
        break
      case get_trainer_by_idName('PHOEBE').id:
        number = 'phoebe'
        break
      case get_trainer_by_idName('GLACIA').id:
        number = 'glacia'
        break
      case get_trainer_by_idName('DRAKE').id:
        number = 'drake'
        break
    }

    if (number) {
      elite_four.value.push({
        icon: number,
        won: props.steps[i].won
      })
      continue
    }

    if (props.steps[i].other_trainer_id === get_trainer_by_idName('WALLACE').id) {
      champion.value = props.steps[i].won
    }
  }
})
</script>

<template>
  <div>
    <p>{{ $t('com.tgpo.percent_complete', { percent: percent_complete() }) }}</p>
    <br />
    <p>{{ $t('com.tgpo.badges_earned') }}</p>
    <div class="row-container centered">
      <div v-for="badge in badges" :key="badge.number">
        <img
          :class="badge.won ? `` : `grayed`"
          width="32"
          :src="`/badges/badge_${badge.number}.png`"
          :alt="`Badge ${badge.number}`"
        />
      </div>
    </div>
    <br />
    <p>{{ $t('com.tgpo.elite_four_defeated') }}</p>
    <div class="row-container centered">
      <div v-for="elite in elite_four" :key="elite.icon">
        <img
          :class="elite.won ? `` : `grayed`"
          width="32"
          :src="`/trainer_overworld/${elite.icon}.png`"
        />
      </div>
    </div>
    <br />
    <p>{{ $t('com.tgpo.champion_defeated') }}</p>
    <img :class="champion ? `` : `grayed`" width="32" src="/trainer_overworld/wallace.png" />
  </div>
</template>

<style scoped>
p {
  margin-bottom: 3px;
}

.grayed {
  filter: brightness(0%);
}

img {
  margin-left: 3px;
  margin-right: 3px;
}
</style>
