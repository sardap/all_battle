<script setup lang="ts">
import { type SeriesResponse } from '@/api'
import { get_trainer, type Trainer } from '@/gen_3'
import { ref } from 'vue'

const props = defineProps<{
  trainer_id: Number
  battle_index?: Number
  series: SeriesResponse
  align: 'left' | 'right' | 'center'
}>()

const trainer = ref<Trainer>(get_trainer(props.trainer_id as number))

function wins_for_trainer(id: number): number {
  let battle_index
  if (props.battle_index === undefined) {
    battle_index = props.series.battles.length
  } else {
    battle_index = props.battle_index as number
  }
  let wins = 0
  for (let i = 0; i < battle_index; i++) {
    if (props.series.battles[i].player === id) {
      if (props.series.battles[i].player_won) {
        wins += 1
      }
    } else {
      if (!props.series.battles[i].player_won) {
        wins += 1
      }
    }
  }

  return wins
}

function gems(id: number): boolean[] {
  const wins = wins_for_trainer(id)

  const result = []

  for (let i = 0; i < props.series.first_to; i++) {
    if (i < wins) {
      result.push(true)
    } else {
      result.push(false)
    }
  }

  return result
}

function pick_div_class() {
  switch (props.align) {
    case 'left':
      return 'left'
    case 'right':
      return 'right'
    case 'center':
      return ''
  }
}
</script>

<template>
  <div :class="pick_div_class()">
    <img
      v-for="(gem, index) in gems(trainer.id)"
      :key="index"
      :src="gem ? '/items/gem.png' : '/items/gem_empty.png'"
      width="64"
      class="gem"
    />
  </div>
</template>

<style scoped>
.left .gem {
  float: left;
}

.right .gem {
  float: right;
}
</style>
