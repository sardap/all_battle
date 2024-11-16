<script setup lang="ts">
import {
  trainer_pic_to_path,
  type Trainer,
  type Mon,
  type TrainerMon,
  get_mon,
  type Move,
  get_moves,
  mon_to_front_pic_path
} from '@/gen_3'
import { ref } from 'vue'
import TypesDisplay from './TypesDisplay.vue'

const props = defineProps<{
  trainer: Trainer
}>()

const party = ref<{ mon: Mon; party: TrainerMon; moves: Move[] }[]>(
  props.trainer.party.map((trainer_mon) => {
    return {
      mon: get_mon(trainer_mon.species),
      party: trainer_mon,
      moves: get_moves(trainer_mon)
    }
  })
)

function col_count() {
  var width = window.innerWidth > 0 ? window.innerWidth : screen.width
  if (width < 800) {
    return 1
  }

  if (party.value.length >= 3) {
    return 3
  }
  return party.value.length
}
</script>

<template>
  <div>
    <h3>#{{ trainer.id }} {{ trainer.trainerName }}</h3>
    <img class="trainer-pic" :src="trainer_pic_to_path(trainer.trainerPic)" />
    <div>
      <p>{{ $t('common.party') }}</p>
      <div
        class="mon-grid-container"
        :style="`grid-template-columns: repeat(${col_count()}, 1fr);`"
      >
        <div v-for="(mon, i) in party" :key="i">
          <p>{{ mon.mon.name }} <TypesDisplay :types="mon.mon.monTypes" /></p>
          <p>{{ $t('common.lvl', { lvl: mon.party.lvl }) }}</p>
          <img :src="mon_to_front_pic_path(mon.mon)" />
          <div>
            <p>{{ $t('common.moves') }}</p>
            <div v-for="(move, j) in mon.moves" :key="j">
              <p>{{ move.name }} <TypesDisplay :types="[move.type]" /></p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
button {
  margin-top: 0px;
  margin-left: 10px;
  margin-right: 10px;
}

.mon-grid-container {
  display: grid;
  margin: auto;
}
</style>
