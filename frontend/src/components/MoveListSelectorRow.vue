<script setup lang="ts">
import type { Move } from '@/gen_3'
import { ref } from 'vue'

const props = defineProps<{
  possible_moves: Move[]
  selected_moves?: string[]
  row_title: string
}>()

const emit = defineEmits<{
  (e: 'update_selected', selected: null | string[]): void
}>()

const filter_moves = ref<null | string[]>(props.selected_moves ? props.selected_moves : null)
const current_move = ref<null | string>(null)

function add_move() {
  if (!current_move.value) {
    filter_moves.value = null
  } else {
    let mon = current_move.value
    filter_moves.value = filter_moves.value ? [...filter_moves.value, mon] : [mon]
  }
  current_move.value = null
  emit('update_selected', filter_moves.value)
}

function remove_move(mon: string) {
  if (!filter_moves.value) {
    return
  }
  filter_moves.value = filter_moves.value.filter((m) => m !== mon)
  if (filter_moves.value.length === 0) {
    filter_moves.value = null
  }
  emit('update_selected', filter_moves.value)
}
</script>

<template>
  <tr>
    <td style="vertical-align: top">{{ row_title }}</td>
    <td style="width: 160px">
      <select v-model="current_move" v-if="possible_moves.length > 0">
        <option :value="null">ANY</option>
        <option v-for="mon in possible_moves" :key="mon.id" :value="mon.idName">
          {{ mon.idName }} #{{ mon.id }}
        </option>
      </select>
      <div v-else>
        <p>NO MOVES LEFT</p>
      </div>
      <button v-if="current_move" @click="add_move()" class="add-mon">ADD</button>
      <table v-if="filter_moves && filter_moves.length > 0" class="mon-table">
        <tr>
          <td>
            <table>
              <tr v-for="mon in filter_moves" :key="mon">
                <td>{{ mon }}</td>
                <td><button @click="remove_move(mon)">DEL</button></td>
              </tr>
            </table>
          </td>
        </tr>
      </table>
    </td>
  </tr>
</template>

<style scoped lang="scss">
.add-mon {
  margin-left: 2px;
}

table {
  border-spacing: 0 0;
}

.mon-table button {
  margin-top: 0px;
}

select {
  width: 100px;
}
</style>
