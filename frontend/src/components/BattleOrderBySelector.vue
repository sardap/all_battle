<script setup lang="ts">
import { ref, watch } from 'vue'

export type MonOrderByField = 'id' | 'duration'

export interface BattleOrderBy {
  field: MonOrderByField
  descending: boolean
}

const props = withDefaults(defineProps<BattleOrderBy>(), {
  field: 'id',
  descending: false
})

const emit = defineEmits<{
  (e: 'updated', val: BattleOrderBy): void
}>()

const field = ref<MonOrderByField>(props.field)
const descending = ref<boolean>(props.descending)
watch([field, descending], () => {
  emit('updated', { field: field.value, descending: descending.value })
})
</script>

<template>
  <div>
    <select v-model="field">
      <option value="id">ID</option>
      <option value="duration">DURATION</option>
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
  width: 120px;
}

hr {
  width: 100%;
  margin: 0;
  margin-top: 5px;
  margin-bottom: 5px;
}
</style>
