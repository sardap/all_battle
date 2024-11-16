<script setup lang="ts">
import type { MoveStats, SingleMonStats, TrainerRank } from '@/api'
import { kd_ratio } from '../utils'
import {
  get_mon,
  get_trainer,
  type Mon,
  type TrainerMon,
  get_type_icon_path,
  get_moves,
  type Move
} from '@/gen_3'
import { ref } from 'vue'
import AnimeMon from './AnimeMon.vue'

const props = defineProps<{
  trainer_id: number
  stats: TrainerRank
}>()

const trainer = ref(get_trainer(props.trainer_id))
const mons = ref<
  {
    mon: Mon
    trainer_mon: TrainerMon
    stats: SingleMonStats
    moves: { move: Move; stats: MoveStats }[]
  }[]
>(get_mons())

function get_mons(): {
  mon: Mon
  trainer_mon: TrainerMon
  stats: SingleMonStats
  moves: { move: Move; stats: MoveStats }[]
}[] {
  const result = []
  for (let i = 0; i < props.stats.mons.length; i++) {
    result.push({
      mon: get_mon(trainer.value.party[i].species),
      trainer_mon: trainer.value.party[i],
      stats: props.stats.mons[i],
      moves: get_moves(trainer.value.party[i]).map((move) => {
        return {
          move: move,
          stats: props.stats.mons[i].moves.find(
            (move_stats) => move_stats.move_id === move.id
          ) as MoveStats
        }
      })
    })
  }
  return result
}

function col_count() {
  var width = window.innerWidth > 0 ? window.innerWidth : screen.width
  if (width < 800) {
    return 1
  }

  if (mons.value.length >= 3) {
    return 3
  }
  return mons.value.length
}
</script>

<template>
  <div class="mon-grid-container" :style="`grid-template-columns: repeat(${col_count()}, 1fr);`">
    <div v-for="(mon, i) in mons" :key="i" class="grid-item">
      <p>
        #{{ mon.mon.id }} {{ mon.mon.name }}
        <img v-for="type in mon.mon.monTypes" :key="type" :src="get_type_icon_path(type)" />
      </p>
      <p>LVL:{{ mon.trainer_mon.lvl }} IV:{{ mon.trainer_mon.iv }}</p>
      <AnimeMon :mon="mon.mon" />
      <p>
        K:{{ mon.stats.murders }} D:{{ mon.stats.deaths }} KDR:{{
          kd_ratio(mon.stats.murders, mon.stats.deaths)
        }}
        ({{ mon.stats.murders }}, {{ mon.stats.deaths }})
      </p>
      <p>DAMAGE TAKEN:{{ mon.stats.damage_taken }}</p>
      <p>DAMAGE DEALT:{{ mon.stats.damage_dealt }}</p>
      <br />
      <div class="move-grid-container">
        <div class="grid-item" v-for="(move, j) in mon.moves" :key="j">
          <p>{{ move.move.name }} <img :src="get_type_icon_path(move.move.type)" /></p>
          <p>USED:{{ move.stats.times_used }}</p>
          <p v-if="move.stats.times_used > 0">DAMAGE:{{ move.stats.damage_dealt }}</p>
          <p v-if="move.stats.times_used > 0">K'O:{{ move.stats.murders }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.mon-grid-container {
  display: grid;
  margin: auto;
}

.move-grid-container {
  display: grid;
  max-width: 400px;
  margin: auto;
}

.grid-item {
  text-align: center;
  margin-bottom: 10px;
}
</style>
