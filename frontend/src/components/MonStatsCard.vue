<script setup lang="ts">
import { ref } from 'vue'
import { type SingleMonStats, type MoveStats } from '@/api'
import {
  function_trainers_count_with_mon,
  type Mon,
  get_move_by_id,
  type Move,
  gen3,
  get_type_icon_path
} from '@/gen_3'
import AnimeMon from './AnimeMon.vue'

const props = defineProps<{
  mon: Mon
  stats: SingleMonStats
  to_show: RandomStats
}>()

export interface RandomStats {
  kd?: boolean
  damage_dealt?: boolean
  damage_taken?: boolean
  times_released?: boolean
  owned_by?: boolean
  deaths?: boolean
  murders?: boolean
  move_to_show?: number
}

const populated_mon = ref<{
  mon: Mon
  stats: SingleMonStats
  featured_move: {
    move: Move
    stats: MoveStats
  } | null
}>({
  mon: props.mon,
  stats: props.stats,
  featured_move: props.to_show.move_to_show
    ? {
        move: get_move_by_id(props.to_show.move_to_show),
        stats: props.stats.moves.find(
          (move) => move.move_id === props.to_show.move_to_show
        ) as MoveStats
      }
    : null
})

function safe_kd(): number {
  if (populated_mon.value.stats.deaths === 0) {
    return populated_mon.value.stats.murders
  }
  return populated_mon.value.stats.murders / populated_mon.value.stats.deaths
}
</script>

<template>
  <div>
    <div v-if="populated_mon">
      <p>{{ populated_mon.mon.idName }}</p>
      <AnimeMon :mon="populated_mon.mon" />
      <p v-if="to_show.times_released">TIMES RELEASED: {{ populated_mon.stats.times_released }}</p>
      <p v-if="to_show.kd">KD RATIO:{{ safe_kd().toFixed(2) }}</p>
      <p v-if="to_show.deaths">FAINTS: {{ populated_mon.stats.deaths }}</p>
      <p v-if="to_show.murders">K'O: {{ populated_mon.stats.murders }}</p>
      <p v-if="to_show.owned_by">
        OWNED BY
        {{
          (
            (function_trainers_count_with_mon(populated_mon.mon.idName) / gen3.trainers.length) *
            100
          ).toFixed(3)
        }}% ({{ function_trainers_count_with_mon(populated_mon.mon.idName) }} )
      </p>
      <p v-if="to_show.damage_dealt">DAMAGE DEALT:{{ populated_mon.stats.damage_dealt }}</p>
      <p v-if="to_show.damage_taken">DAMAGE TAKEN:{{ populated_mon.stats.damage_taken }}</p>
      <div v-if="populated_mon.featured_move">
        <p>
          MOVE: {{ populated_mon.featured_move.move.name }}
          <img :src="get_type_icon_path(populated_mon.featured_move.move.type)" />
        </p>
        <p v-if="to_show.move_to_show">
          USED {{ populated_mon.featured_move.stats.times_used }} TIMES
        </p>
        <p>DEALT {{ populated_mon.featured_move.stats.damage_dealt }} DAMAGE</p>
        <p v-if="populated_mon.featured_move.stats.damage_dealt > 0">
          K'O {{ populated_mon.featured_move.stats.murders }} MONS
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
