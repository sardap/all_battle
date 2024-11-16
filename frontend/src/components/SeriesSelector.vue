<script setup lang="ts">
import { ref, watch } from 'vue'
import { type SeriesSearchEntry } from '@/api'
import { get_trainer, type Trainer } from '@/gen_3'

const props = defineProps<{
  series: SeriesSearchEntry[]
  featured_trainer?: number
  extra_trainer_attributes?: Record<number, string>
  selected_series?: number
}>()

const trainer_win_count = ref<number | null>(null)
const battle_count = ref<number | null>(null)
const min_duration = ref<number>(0)
const max_duration = ref<number>(10000)

const filited_series = ref<SeriesSearchEntry[]>(props.series)

watch([trainer_win_count, min_duration, max_duration, battle_count], () => {
  const updated: SeriesSearchEntry[] = []

  for (let i = 0; i < props.series.length; i++) {
    const series = props.series[i]
    let matches = true

    if (matches) {
      let battle_matches = false
      series.battle.forEach((battle) => {
        if (battle.duration >= min_duration.value && battle.duration <= max_duration.value) {
          battle_matches = true
        }
      })
      matches = battle_matches
    }

    if (
      matches &&
      !!props.featured_trainer &&
      trainer_win_count.value != null &&
      trainer_win_count.value >= 0
    ) {
      let featued_trainer_wins = 0
      series.battle.forEach((battle) => {
        if (battle.winning_trainer_id == props.featured_trainer) {
          featued_trainer_wins++
        }
      })
      if (featued_trainer_wins != trainer_win_count.value) {
        matches = false
      }
    }

    if (matches && battle_count.value != null) {
      matches = series.battle.length == battle_count.value
    }

    if (matches) {
      updated.push(series)
    }
  }

  if (
    series_id.value != null &&
    updated.find((series) => {
      series.series_id === series_id.value
    }) === undefined
  ) {
    if (updated.length > 0) {
      series_id.value = updated[0].series_id
    } else {
      series_id.value = null
    }
  }

  filited_series.value = updated
})

const emit = defineEmits<{
  (e: 'updated_series', series_id: number): void
}>()

const series_id = ref<number | null>(props.selected_series ? props.selected_series : null)

watch(series_id, (new_series_id) => {
  if (new_series_id) {
    emit('updated_series', new_series_id)
  }
})

function trainer_title(series: SeriesSearchEntry, trainer: Trainer) {
  let wins = series.battle.filter((i) => i.winning_trainer_id === trainer.id).length

  let extra = ''
  if (props.extra_trainer_attributes && props.extra_trainer_attributes[trainer.id]) {
    extra = ` ${props.extra_trainer_attributes[trainer.id]}`
  }

  return `${trainer.trainerName} #${trainer.id} ${wins}W` + extra
}

function series_title(series: SeriesSearchEntry) {
  let trainer_a = get_trainer(series.trainers[0])
  let trainer_b = get_trainer(series.trainers[1])

  let left_trainer = trainer_b.id === props.featured_trainer ? trainer_b : trainer_a
  let right_trainer = trainer_b.id === props.featured_trainer ? trainer_a : trainer_b

  return `${trainer_title(series, left_trainer)} vs ${trainer_title(series, right_trainer)} (${series.series_id})`
}
</script>

<template>
  <div class="series-selector">
    <br />
    <div class="col-container centered">
      <p>SERIES FILTER</p>
      <table>
        <tr v-if="featured_trainer">
          <td>WIN COUNT</td>
          <td>
            <select v-model="trainer_win_count">
              <option :value="null">DON'T</option>
              <option :value="0">0</option>
              <option :value="1">1</option>
              <option :value="2">2</option>
            </select>
          </td>
        </tr>
        <tr>
          <td>BATTLE COUNT</td>
          <td>
            <select v-model="battle_count">
              <option :value="null">DON'T</option>
              <option :value="2">2</option>
              <option :value="3">3</option>
            </select>
          </td>
        </tr>
        <tr>
          <td>BATTLE LENGTH</td>
          <td>
            <input v-model="min_duration" type="number" /> -
            <input v-model="max_duration" type="number" />
          </td>
        </tr>
      </table>
      <p>FOUND {{ filited_series.length }} SERIES</p>
      <p v-if="filited_series.length > 5000">WARNING OVER 5000 SERIES SELECT MAY BE SLOW</p>
    </div>
    <div v-if="filited_series.length > 0">
      <select v-model="series_id">
        <option :value="null" disabled selected>SELECT A SERIES</option>
        <option v-for="i in filited_series" :key="i.series_id" :value="i.series_id">
          {{ series_title(i) }}
        </option>
      </select>
    </div>
    <div v-else>
      <p>NO SERIES MATCH THE GIVEN FILTER</p>
    </div>
  </div>
</template>

<style scoped lang="scss">
table {
  border-collapse: separate;
  border-spacing: 0 0.5em;
  text-align: left;
}

input {
  width: 50px;
}

.series-search {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
}

.series-selector {
  max-width: 100%;
  overflow: hidden;
}

.find {
  margin-top: 20px;
  margin-bottom: 20px;
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
}

.trainer-filter-container {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
}

.trainer-filter {
  width: 50%;
}

.group-title {
  text-align: center;
}
</style>
