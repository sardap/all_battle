<script setup lang="ts">
import { type SingleMonStats } from '@/api'
import { get_move_by_id, type Mon } from '@/gen_3'
import AnimeMon from './AnimeMon.vue'
import TypesDisplay from './TypesDisplay.vue'
import { ref } from 'vue'

const props = defineProps<{
  mon: Mon
  stats: SingleMonStats
}>()

const filtered_moves = ref(filter_moves())

function filter_moves() {
  const moves = props.stats.moves
    .map((move_stats) => {
      return {
        stats: move_stats,
        move: get_move_by_id(move_stats.move_id)
      }
    })
    .filter((move) => {
      if (move.stats.times_used <= 0) {
        return false
      }

      if (move.move.power === 0) {
        return true
      }

      return move.stats.damage_dealt > 0
    })

  return moves
}
</script>

<template>
  <p>{{ mon.name }} <TypesDisplay :types="mon.monTypes" /></p>
  <AnimeMon :mon="mon" />
  <p v-if="stats.murders > 0">KO'S:{{ stats.murders }}</p>
  <p v-if="stats.damage_dealt > 0">DAMAGE DEALT:{{ stats.damage_dealt }}</p>
  <p v-if="stats.damage_taken">DAMAGE TAKEN:{{ stats.damage_taken }}</p>
  <div v-if="filtered_moves.length > 0">
    <p>MOVES</p>
    <div v-for="move in filtered_moves" :key="move.move.id">
      <p>
        <TypesDisplay :types="[move.move.type]" /> {{ move.move.name }} {{ move.stats.times_used }}x
      </p>
      <p v-if="move.stats.damage_dealt > 0">
        DMG:{{ move.stats.damage_dealt }} KO'S:{{ move.stats.murders }}
      </p>
    </div>
  </div>
</template>

<style scoped></style>
