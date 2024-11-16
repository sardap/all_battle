<script setup lang="ts">
import { ref, watch } from 'vue'
import { all_types } from '@/gen_3'

export interface MonRankLocalFilter {
  mon_type?: string
  name?: string
  min_exists?: number
  max_exists?: number
  min_kdr?: number
  max_kdr?: number
  min_avg_lvl?: number
  max_avg_lvl?: number
}

const props = withDefaults(defineProps<MonRankLocalFilter>(), {
  mon_type: undefined,
  name: undefined,
  min_exists: 1,
  max_exists: 40,
  min_kdr: 0.0,
  max_kdr: 10000,
  min_avg_lvl: 1,
  max_avg_lvl: 100
})

const emit = defineEmits<{
  (e: 'updated', update: MonRankLocalFilter): void
}>()

const mon_type = ref<string | undefined>(props.mon_type)
const name = ref<string | undefined>(props.name)
const min_exists = ref<number>(props.min_exists)
const max_exists = ref<number>(props.max_exists)
const min_kdr = ref<number>(props.min_kdr)
const max_kdr = ref<number>(props.max_kdr)
const min_avg_lvl = ref<number>(props.min_avg_lvl)
const max_avg_lvl = ref<number>(props.max_avg_lvl)

console.log('STARTING MON TYPE ' + mon_type.value)

watch([mon_type, name, min_exists, min_kdr, max_kdr, min_avg_lvl, max_avg_lvl], () => {
  emit('updated', {
    mon_type: mon_type.value,
    name: name.value,
    min_exists: min_exists.value,
    max_exists: max_exists.value,
    min_kdr: min_kdr.value,
    max_kdr: max_kdr.value,
    min_avg_lvl: min_avg_lvl.value,
    max_avg_lvl: max_avg_lvl.value
  })
})
</script>

<template>
  <div>
    <table>
      <tr>
        <td>TYPE</td>
        <td>
          <select v-model="mon_type">
            <option :value="undefined">ANY</option>
            <option v-for="type in all_types()" :key="type" :value="type">{{ type }}</option>
          </select>
        </td>
      </tr>
      <tr>
        <td>NAME</td>
        <td><input v-model="name" type="text" /></td>
      </tr>
      <tr>
        <td>MIN EXISTS</td>
        <td>
          <input v-model="min_exists" type="number" /> -
          <input v-model="max_exists" type="number" />
        </td>
      </tr>
      <tr>
        <td>KDR</td>
        <td>
          <input v-model="min_kdr" type="number" /> - <input v-model="max_kdr" type="number" />
        </td>
      </tr>
      <tr>
        <td>LVL</td>
        <td>
          <input v-model="min_avg_lvl" type="number" /> -
          <input v-model="max_avg_lvl" type="number" />
        </td>
      </tr>
    </table>
  </div>
</template>

<style scoped lang="scss">
input[type='text'] {
  width: 100px;
}

input[type='number'] {
  width: 40px;
}

hr {
  width: 100%;
  margin: 0;
  margin-top: 5px;
  margin-bottom: 5px;
}
</style>
