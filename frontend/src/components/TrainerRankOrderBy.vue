<script setup lang="ts">
import { ref, watch } from 'vue'

export type TrainerOrderByField = 'win_rate' | 'battle_count'

export interface TrainerOrderBy {
  field: TrainerOrderByField
  descending: boolean
}

interface Props extends TrainerOrderBy {}

const props = withDefaults(defineProps<Props>(), {
  field: 'win_rate',
  descending: true
})

const emit = defineEmits<{
  (e: 'updated', val: TrainerOrderBy): void
}>()

const field = ref<'win_rate' | 'battle_count'>(props.field)
const descending = ref<boolean>(props.descending)
watch([field, descending], () => {
  emit('updated', { field: field.value, descending: descending.value })
})
</script>

<template>
  <div>
    <select v-model="field">
      <option value="win_rate">WIN RATE</option>
      <option value="battle_count">BATTLE COUNT</option>
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
