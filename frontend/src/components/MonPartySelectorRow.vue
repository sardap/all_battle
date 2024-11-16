<script setup lang="ts">
import type { Mon } from '@/gen_3'
import { ref } from 'vue'

const props = defineProps<{
  possible_mons: Mon[]
  selected_mons?: string[]
  row_title?: string
}>()

const emit = defineEmits<{
  (e: 'update_selected', selected: null | string[]): void
}>()

const filter_mons = ref<null | string[]>(props.selected_mons ? props.selected_mons : null)
const current_mon = ref<null | string>(null)

function add_mon() {
  if (!current_mon.value) {
    filter_mons.value = null
  } else {
    let mon = current_mon.value
    filter_mons.value = filter_mons.value ? [...filter_mons.value, mon] : [mon]
  }
  current_mon.value = null
  emit('update_selected', filter_mons.value)
}

function remove_mon(mon: string) {
  if (!filter_mons.value) {
    return
  }
  filter_mons.value = filter_mons.value.filter((m) => m !== mon)
  if (filter_mons.value.length === 0) {
    filter_mons.value = null
  }
  emit('update_selected', filter_mons.value)
}
</script>

<template>
  <tr class="mon-table">
    <td style="vertical-align: top">{{ row_title ? row_title : `PARTY INCLUDES` }}</td>
    <td style="width: 160px">
      <select v-model="current_mon" v-if="possible_mons.length > 0">
        <option :value="null">ANY</option>
        <option v-for="mon in possible_mons" :key="mon.id" :value="mon.idName">
          {{ mon.idName }} #{{ mon.id }}
        </option>
      </select>
      <div v-else>
        <p>NO MONS LEFT</p>
      </div>
      <button v-if="current_mon" @click="add_mon()" class="add-mon">ADD</button>
      <table v-if="filter_mons && filter_mons.length > 0">
        <tr>
          <td>
            <table>
              <tr v-for="mon in filter_mons" :key="mon">
                <td>{{ mon }}</td>
                <td><button class="remove-mon" @click="remove_mon(mon)">DEL</button></td>
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
