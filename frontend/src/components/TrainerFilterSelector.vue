<script setup lang="ts">
import type { SeriesFilter } from '@/api'
import { gen3, get_trainer_classes, type Trainer } from '@/gen_3'
import { onMounted, ref, watch } from 'vue'
import { fix_trainer_class_string } from '@/gen_3'
import MonPartySelectorRow from './MonPartySelectorRow.vue'

interface Props extends SeriesFilter {}

const props = withDefaults(defineProps<Props>(), {
  trainer_class: null,
  trainer_id: null,
  number_of_mons: null,
  mons: null
})

const emit = defineEmits<{
  (e: 'filter_updated', filter: SeriesFilter): void
}>()

const filter_trainer_class = ref<null | string>(props.trainer_class)
const filter_trainer_id = ref<null | number>(props.trainer_id)
const filter_number_of_mons = ref<null | number>(props.number_of_mons)
const filter_mons = ref<null | string[]>(props.mons)

const possible_trainers = ref(gen3.trainers)
const possible_trainers_changed = ref(0)
const possible_mons = ref(gen3.mons)
const possible_classes = ref(get_trainer_classes())

const mon_counts = [1, 2, 3, 4, 5, 6]

const possible_mon_counts = ref(mon_counts)

interface SkipFilter {
  trainer_class?: boolean
  number_of_mons?: boolean
  mons?: boolean
}

function get_possible_trainers(skip: SkipFilter): Trainer[] {
  let filtered_trainers = gen3.trainers
  if (filter_trainer_class.value && !skip.trainer_class) {
    filtered_trainers = gen3.trainers.filter((trainer) => {
      return trainer.trainerClass === filter_trainer_class.value
    })
  }

  if (filter_number_of_mons.value && !skip.number_of_mons) {
    filtered_trainers = filtered_trainers.filter((trainer) => {
      return trainer.partySize === filter_number_of_mons.value
    })
  }

  if (filter_mons.value && !skip.mons) {
    filtered_trainers = filtered_trainers.filter((trainer) => {
      return trainer.party.some((party_mon) => {
        if (!filter_mons.value) {
          return true
        }
        return filter_mons.value.includes(party_mon.species)
      })
    })
  }

  return filtered_trainers
}

function update_possible() {
  let filtered_trainers = get_possible_trainers({ number_of_mons: true })

  let filtered_mon_counts = mon_counts.filter((count) => {
    return filtered_trainers.some((trainer) => {
      return trainer.partySize === count
    })
  })

  filtered_trainers = get_possible_trainers({ mons: true })
  const mons_existing_in_parties = filtered_trainers.flatMap((trainer) => {
    return trainer.party.map((party_mon) => party_mon.species)
  })
  let filtered_mons = gen3.mons.filter((mon) => {
    return mons_existing_in_parties.includes(mon.idName)
  })
  if (filter_mons.value) {
    filtered_mons = filtered_mons.filter((mon) => {
      if (!filter_mons.value) {
        return true
      }
      return !filter_mons.value.includes(mon.idName)
    })
  }

  filtered_trainers = get_possible_trainers({ trainer_class: true })
  let filtered_trainer_classes = get_trainer_classes().filter((trainer_class) => {
    return filtered_trainers.some((trainer) => {
      return trainer_class === trainer.trainerClass
    })
  })

  filtered_trainers = get_possible_trainers({})

  possible_classes.value = filtered_trainer_classes
  possible_mon_counts.value = filtered_mon_counts
  possible_trainers.value = filtered_trainers
  possible_mons.value = filtered_mons
  possible_trainers_changed.value += 1

  emit('filter_updated', {
    trainer_class: filter_trainer_class.value,
    trainer_id: filter_trainer_id.value,
    number_of_mons: filter_number_of_mons.value,
    mons: filter_mons.value
  })
}

watch([filter_trainer_id, filter_number_of_mons, filter_trainer_class, filter_mons], () => {
  update_possible()
})

onMounted(() => {
  update_possible()
})
</script>

<template>
  <div class="col-container centered">
    <table>
      <tr>
        <td>TRAINER CLASS</td>
        <td>
          <select v-model="filter_trainer_class">
            <option :value="null">ANY</option>
            <option
              v-for="trainer_class in possible_classes"
              :key="trainer_class"
              :value="trainer_class"
            >
              {{ fix_trainer_class_string(trainer_class) }}
            </option>
          </select>
        </td>
      </tr>
      <tr :key="possible_trainers_changed">
        <td>NUMBER OF MONS</td>
        <td>
          <select v-model="filter_number_of_mons">
            <option :value="null">ANY</option>
            <option v-for="num in possible_mon_counts" :key="num" :value="num">
              {{ num }}
            </option>
          </select>
        </td>
      </tr>
      <MonPartySelectorRow
        :possible_mons="possible_mons"
        @update_selected="(selected) => (filter_mons = selected)"
      />
      <tr :key="possible_trainers_changed">
        <td>TRAINERS</td>
        <td>
          <select class="trainer-option" v-model="filter_trainer_id">
            <option :value="null">ANY</option>
            <option v-for="trainer in possible_trainers" :key="trainer.id" :value="trainer.id">
              {{ trainer.idName }} {{ trainer.trainerClass }} #{{ trainer.id }}
            </option>
          </select>
        </td>
      </tr>
    </table>
    <div>
      <h3>SELECTED TRAINERS {{ filter_trainer_id ? 1 : possible_trainers.length }}</h3>
    </div>
  </div>
</template>

<style scoped lang="scss">
.add-mon {
  margin-left: 2px;
}

table {
  border-collapse: separate;
  border-spacing: 0 0.5em;
}

.mon-table table {
  border-spacing: 0 0;
}

.mon-table button {
  margin-top: 0px;
}

select {
  width: 160px;
}
</style>
