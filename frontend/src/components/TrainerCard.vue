<script setup lang="ts">
import { type TrainerHistory } from '@/api'
import {
  get_mon,
  get_trainer,
  get_trainers_of_class,
  item_to_icon_path,
  get_moves,
  mon_to_icon_pic_path,
  type Move,
  type Trainer,
  type TrainerMon,
  get_type_icon_path
} from '@/gen_3'
import { ref } from 'vue'
import TypesDisplay from './TypesDisplay.vue'

const props = defineProps<{
  trainer_id: Number
  trainer_history: TrainerHistory
  opponent_id: Number
  active_mon_index: Number
  fainted_mons: Number[]
}>()

const trainer = ref<Trainer>(get_trainer(props.trainer_id as number))
const opponent = ref<Trainer>(get_trainer(props.opponent_id as number))

function is_special_move(move: Move): boolean {
  // Grass, Fire, Electric, Water, Ice, Dragon, and Dark
  return (
    move.type === 'GRASS' ||
    move.type === 'FIRE' ||
    move.type === 'ELECTRIC' ||
    move.type === 'WATER' ||
    move.type === 'ICE' ||
    move.type === 'DRAGON' ||
    move.type === 'DARK'
  )
}

function get_win_vs_class(trainer_histroy: TrainerHistory, class_name: string): number {
  const trainers_of_class = get_trainers_of_class(class_name).map((trainer) => trainer.id)
  let total = 0
  let wins = 0

  for (let i = 0; i < trainer_histroy.history.length; i++) {
    let history = trainer_histroy.history[i]
    if (trainers_of_class.includes(history.opponent)) {
      total++
      if (history.won) {
        wins++
      }
    }
  }

  if (total === 0) {
    return 0
  }

  return (wins / total) * 100
}

function get_win_percent(trainer_histroy: TrainerHistory): number {
  if (trainer_histroy.total === 0) {
    return 0
  }

  return (trainer_histroy.wins / trainer_histroy.total) * 100
}

interface ReorderedTrainerMon extends TrainerMon {
  index: number
}

function ordered_mons(): ReorderedTrainerMon[] {
  const mon_order = []
  mon_order.push({
    ...trainer.value.party[props.active_mon_index as number],
    index: props.active_mon_index as number
  })
  for (let i = 0; i < trainer.value.party.length; i++) {
    if (i === props.active_mon_index) {
      continue
    }

    if (!props.fainted_mons.includes(i)) {
      mon_order.push({ ...trainer.value.party[i], index: i })
    }
  }

  for (let i = 0; i < trainer.value.party.length; i++) {
    if (props.fainted_mons.includes(i) && i !== props.active_mon_index) {
      mon_order.push({ ...trainer.value.party[i], index: i })
    }
  }

  return mon_order
}
</script>

<template>
  <div class="trainer-card-holder">
    <h2>#{{ trainer.id }} {{ trainer.trainerName }}</h2>
    <p>{{ trainer.trainerClass.replace('TRAINER_CLASS_', '').replace('_', ' ') }}</p>
    <div v-if="trainer_history">
      <p>Overall W {{ get_win_percent(trainer_history).toFixed(0) }}%</p>
      <p>VS Class {{ get_win_vs_class(trainer_history, opponent.trainerClass).toFixed(0) }}%</p>
    </div>
    <hr />
    <div
      v-for="mon in ordered_mons()"
      :key="mon.index"
      :class="fainted_mons.includes(mon.index) ? 'fainted' : 'alive'"
    >
      <p>
        <img
          v-if="active_mon_index !== mon.index"
          :src="mon_to_icon_pic_path(get_mon(mon.species))"
          width="25"
          height="25"
        />
        {{ mon.species }} Lv:{{ mon.lvl }}
        <TypesDisplay
          v-if="active_mon_index === mon.index"
          :types="get_mon(mon.species).monTypes"
        />
      </p>
      <div v-if="active_mon_index === mon.index">
        <p v-if="mon.heldItem"><img :src="item_to_icon_path(mon.heldItem)" /></p>
        <p v-for="move in get_moves(mon)" :key="move.id" class="move">
          <img :src="get_type_icon_path(move.type)" /> {{ is_special_move(move) ? '🧙' : '💥 ' }}
          {{ move.name }}
        </p>
        <hr />
      </div>
    </div>
  </div>
</template>

<style scoped>
.trainer-card-holder {
  min-width: 200px;
}

.move {
  text-align: left;
}

.fainted {
  color: #9b9b9b;
}
</style>
