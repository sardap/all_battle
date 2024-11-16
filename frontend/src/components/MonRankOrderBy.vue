<script setup lang="ts">
import { ref, watch } from 'vue'

export type MonOrderByField = 'exists' | 'released' | 'kdr' | 'avg_lvl'

export interface MonOrderBy {
  field: MonOrderByField
  descending: boolean
}

const props = withDefaults(defineProps<MonOrderBy>(), {
  field: 'kdr',
  descending: true
})

const emit = defineEmits<{
  (e: 'updated', val: MonOrderBy): void
}>()

const field = ref<'exists' | 'released' | 'kdr' | 'avg_lvl'>(props.field)
const descending = ref<boolean>(props.descending)
watch([field, descending], () => {
  emit('updated', { field: field.value, descending: descending.value })
})
</script>

<template>
  <div>
    <select v-model="field">
      <option value="kdr">KDR</option>
      <option value="exists">NUMBER THAT EXIST</option>
      <option value="released">TIMES RELEASED</option>
      <option value="avg_lvl">AVERAGE LVL</option>
    </select>
    <select v-model="descending">
      <option :value="true">DESCENDING</option>
      <option :value="false">ASCENDING</option>
    </select>
  </div>
</template>

<style scoped lang="scss">
select {
  margin-right: 5px;
}

hr {
  width: 100%;
  margin: 0;
  margin-top: 5px;
  margin-bottom: 5px;
}
</style>
