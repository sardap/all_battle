<script setup lang="ts">
import { type Trainer, type Mon, get_trainer_classes, gen3 } from '@/gen_3'
import { onMounted, ref, watch } from 'vue'
import { fix_trainer_class_string } from '@/gen_3'
import MonPartySelectorRow from './MonPartySelectorRow.vue'

export interface TrainerRankFilterOptions {
  show_class?: string
  show_trainer_name?: string
  includes_mons?: string[]
  mon_count?: number
}

interface Props extends TrainerRankFilterOptions {
  trainers: { trainer: Trainer }[]
}

const props = withDefaults(defineProps<Props>(), {
  show_class: undefined,
  show_trainer_name: '',
  includes_mons: undefined,
  mon_count: undefined
})

const emit = defineEmits<{
  (e: 'filter_updated', filter: TrainerRankFilterOptions): void
}>()

const show_class = ref<string | undefined>(props.show_class)
const show_trainer_name = ref(props.show_trainer_name)
const includes_mons = ref<string[] | undefined>(props.includes_mons)
const mon_count = ref<number | undefined>(props.mon_count)

watch([show_class, show_trainer_name, includes_mons, mon_count], () => {
  emit('filter_updated', {
    show_class: show_class.value,
    show_trainer_name: show_trainer_name.value,
    includes_mons: includes_mons.value,
    mon_count: mon_count.value
  })
})

const filtered_mons = ref<Mon[]>(gen3.mons)
const filtered_classes = ref<string[]>(get_trainer_classes())

onMounted(() => {
  // const mons = new Set<string>()
  // const classes = new Set<string>()
  // for (const trainer of props.trainers) {
  //   for (const mon of trainer.trainer.party) {
  //     mons.add(mon.species)
  //   }
  //   classes.add(trainer.trainer.trainerClass)
  // }
  // filtered_mons.value = Array.from(mons).map((mon_species) => {
  //   return get_mon(mon_species)
  // })
  // filtered_mons.value.sort((a, b) => {
  //   return a.id - b.id
  // })
  // filtered_classes.value = Array.from(classes)
})
</script>

<template>
  <table>
    <tr>
      <td>CLASS</td>
      <td>
        <select v-model="show_class">
          <option :value="undefined">ALL</option>
          <option
            v-for="trainer_class in filtered_classes"
            :value="trainer_class"
            :key="trainer_class"
          >
            {{ fix_trainer_class_string(trainer_class) }}
          </option>
        </select>
      </td>
    </tr>
    <tr>
      <td>NAME INCLUDES</td>
      <td>
        <input v-model="show_trainer_name" />
      </td>
    </tr>
    <tr>
      <td>NUMBER OF MONS</td>
      <td>
        <select v-model="mon_count">
          <option :value="undefined">ANY</option>
          <option v-for="i in 6" :value="i" :key="i">
            {{ i }}
          </option>
        </select>
      </td>
    </tr>
    <MonPartySelectorRow
      :possible_mons="filtered_mons"
      @update_selected="
        (selected) => (selected ? (includes_mons = selected) : (includes_mons = undefined))
      "
    />
  </table>
</template>

<style scoped lang="scss">
table {
  border-collapse: separate;
  border-spacing: 0 0.5em;
}

select {
  width: 160px;
}
input {
  width: 160px;
}
</style>
