<script setup lang="ts">
import {
  get_trainer_mons,
  get_mon,
  get_move,
  get_trainer_mon_moves,
  get_move_by_id,
  get_mon_by_id
} from '@/gen_3'
import { ref, watch } from 'vue'
import MonPartySelectorRow from './MonPartySelectorRow.vue'
import MoveListSelectorRow from './MoveListSelectorRow.vue'

export interface BattleSearchFilter {
  min_length_seconds?: number
  max_length_seconds?: number
  seed?: number
  player_perspective_won?: boolean
  min_kos?: number
  max_kos?: number
  min_damage_dealt?: number
  max_damage_dealt?: number
  mons_included?: string
  mons_excluded?: string
  moves_used?: string
  moves_not_used?: string
}

interface Props extends BattleSearchFilter {}

const props = withDefaults(defineProps<Props>(), {
  min_length_seconds: 0,
  max_length_seconds: 2000,
  seed: undefined,
  player_perspective_won: undefined,
  min_kos: 0,
  max_kos: 11,
  min_damage_dealt: 0,
  max_damage_dealt: 1000,
  mons_included: undefined,
  moves_used: undefined,
  moves_not_used: undefined
})

const emit = defineEmits<{
  (e: 'updated', filter: BattleSearchFilter): void
}>()

const min_length_seconds = ref<number>(props.min_length_seconds)
const max_length_seconds = ref<number>(props.max_length_seconds)
const seed = ref<string | undefined>(
  typeof props.seed === 'number' ? props.seed.toString() : props.seed
)
const player_perspective_won = ref<boolean | undefined>(props.player_perspective_won)
const min_kos = ref<number>(props.min_kos)
const max_kos = ref<number>(props.max_kos)
const min_damage_dealt = ref<number>(props.min_damage_dealt)
const max_damage_dealt = ref<number>(props.max_damage_dealt)
const include_mons = ref<string[] | null>(
  props.mons_included
    ? props.mons_included.split(',').map((mon_id) => get_mon_by_id(Number(mon_id)).idName)
    : null
)
const exclude_mons = ref<string[] | null>(
  props.mons_excluded
    ? props.mons_excluded.split(',').map((mon_id) => get_mon_by_id(Number(mon_id)).idName)
    : null
)
const moves_used = ref<string[] | null>(
  props.moves_used
    ? props.moves_used.split(',').map((move_id) => get_move_by_id(Number(move_id)).idName)
    : null
)
const moves_not_used = ref<string[] | null>(
  props.moves_not_used
    ? props.moves_not_used.split(',').map((move_id) => get_move_by_id(Number(move_id)).idName)
    : null
)

watch(
  [
    min_length_seconds,
    max_length_seconds,
    seed,
    player_perspective_won,
    min_kos,
    max_kos,
    min_damage_dealt,
    max_damage_dealt,
    include_mons,
    exclude_mons,
    moves_used,
    moves_not_used
  ],
  () => {
    emit('updated', {
      min_length_seconds: min_length_seconds.value,
      max_length_seconds: max_length_seconds.value,
      seed: seed.value === '' ? undefined : parseInt(seed.value as string),
      player_perspective_won: player_perspective_won.value,
      min_kos: min_kos.value,
      max_kos: max_kos.value,
      min_damage_dealt: min_damage_dealt.value,
      max_damage_dealt: max_damage_dealt.value,
      mons_included: include_mons.value
        ? include_mons.value.map((name) => get_mon(name).id).join(',')
        : '',
      mons_excluded: exclude_mons.value
        ? exclude_mons.value.map((name) => get_mon(name).id).join(',')
        : '',
      moves_used: moves_used.value
        ? moves_used.value.map((name) => get_move(name).id).join(',')
        : '',
      moves_not_used: moves_not_used.value
        ? moves_not_used.value.map((name) => get_move(name).id).join(',')
        : ''
    })
  }
)
</script>

<template>
  <table>
    <tr>
      <td>BATTLE LENGTH</td>
      <td>
        <input v-model="min_length_seconds" type="number" /> -
        <input v-model="max_length_seconds" type="number" />
      </td>
    </tr>
    <tr>
      <td>SEED</td>
      <td>
        <input v-model="seed" type="number" />
      </td>
    </tr>
    <tr>
      <td>PLAYER PER WON?</td>
      <td>
        <select v-model="player_perspective_won">
          <option :value="undefined">ANY</option>
          <option :value="true">YES</option>
          <option :value="false">NO</option>
        </select>
      </td>
    </tr>
    <tr>
      <td>KOS</td>
      <td><input v-model="min_kos" type="number" /> - <input v-model="max_kos" type="number" /></td>
    </tr>
    <tr>
      <td>DAMAGE DEALT</td>
      <td>
        <input v-model="min_damage_dealt" type="number" /> -
        <input v-model="max_damage_dealt" type="number" />
      </td>
    </tr>
    <MonPartySelectorRow
      row_title="INCLUDES MONS"
      :selected_mons="include_mons || []"
      :possible_mons="get_trainer_mons()"
      @update_selected="(selected) => (include_mons = selected)"
    />
    <MonPartySelectorRow
      row_title="EXCLUDES MONS"
      :selected_mons="exclude_mons || []"
      :possible_mons="get_trainer_mons()"
      @update_selected="(selected) => (exclude_mons = selected)"
    />
    <MoveListSelectorRow
      row_title="MOVES USED"
      :selected_moves="moves_used || []"
      :possible_moves="get_trainer_mon_moves()"
      @update_selected="(selected) => (moves_used = selected)"
    />
    <MoveListSelectorRow
      row_title="MOVES NOT USED"
      :selected_moves="moves_not_used || []"
      :possible_moves="get_trainer_mon_moves()"
      @update_selected="(selected) => (moves_not_used = selected)"
    />
  </table>
</template>

<style scoped lang="scss">
table {
  border-collapse: separate;
  border-spacing: 0 0.5em;
  text-align: left;
}

select {
  width: 50px;
}

input {
  width: 50px;
}
</style>
