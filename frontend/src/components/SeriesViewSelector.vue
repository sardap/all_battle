<script setup lang="ts">
import { ref, watch } from 'vue'
import TrainerFilterSelector from './TrainerFilterSelector.vue'
import SeriesSelector from './SeriesSelector.vue'
import {
  default_series_filter,
  search_series,
  type SeriesFilter,
  type SeriesSearchResponse,
  series_filter_match
} from '@/api'
import LoadingCard from './LoadingCard.vue'
import { useRoute } from 'vue-router'
import router from '@/router'

const emit = defineEmits<{
  (e: 'updated_series', series_id: number): void
}>()

const group_a = ref<SeriesFilter>(get_group_a_filter_from_query())
const group_b = ref<SeriesFilter>(get_group_b_filter_from_query())

watch([group_a, group_b], () => {
  button_disabled.value = false
  if (series_filter_match(group_a.value, group_b.value)) {
    button_disabled.value = true
    error.value = 'GROUPS PERFECTLY MATCH NO DELTA CAN BE FOUND'
  } else {
    error.value = null
  }

  router.push({
    query: {
      group_a: encodeURIComponent(JSON.stringify(group_a.value)),
      group_b: encodeURIComponent(JSON.stringify(group_b.value))
    }
  })
})

function get_group_a_filter_from_query() {
  const route = useRoute()
  if (route.query.group_a) {
    return JSON.parse(decodeURIComponent(route.query.group_a.toString()))
  }
  return default_series_filter()
}

function get_group_b_filter_from_query() {
  const route = useRoute()
  if (route.query.group_b) {
    return JSON.parse(decodeURIComponent(route.query.group_b.toString()))
  }
  return default_series_filter()
}

const button_disabled = ref<boolean>(false)
const series_search = ref<SeriesSearchResponse | null>(null)
const group_a_win_per = ref<number>(0)
const group_b_win_per = ref<number>(0)
const series_id = ref<number>(0)
const error = ref<string | null>(null)
const loading = ref(false)
const extra_trainer_attributes = ref<Record<number, string>>({})

async function find() {
  loading.value = true
  error.value = null
  try {
    const response = await search_series(group_a.value, group_b.value)
    button_disabled.value = true
    if (response.series.length === 0) {
      series_search.value = null
      error.value = 'NO MATCHES BETWEEN GROUPS YET...'
    } else {
      button_disabled.value = true
      series_id.value = response.series[0].series_id
      const total = response.group_a_wins + response.group_b_wins
      group_a_win_per.value = response.group_a_wins / total
      group_b_win_per.value = response.group_b_wins / total

      const updated_attributes: Record<number, string> = {}
      for (const trainer_id of response.group_a) {
        updated_attributes[trainer_id] = 'A'
      }
      for (const trainer_id of response.group_b) {
        updated_attributes[trainer_id] = 'B'
      }
      extra_trainer_attributes.value = updated_attributes

      // Sort series trainers by group
      for (const series of response.series) {
        series.trainers.sort((a, b) => {
          if (response.group_a.includes(a)) {
            return -1
          } else {
            return 1
          }
        })
      }

      series_search.value = response
    }
  } catch (err) {
    error.value = 'FILTERS PERFECTLY MATCH NO DELTA CAN BE FOUND'
  }
  loading.value = false
}

function update_trainer_a(filter: SeriesFilter) {
  group_a.value = filter
}

function update_trainer_b(filter: SeriesFilter) {
  group_b.value = filter
}

watch(series_id, (new_series_id) => {
  emit('updated_series', new_series_id)
})
</script>

<template>
  <div class="series-selector">
    <div class="trainer-filter-container">
      <div class="trainer-filter">
        <h2 class="group-title">GROUP A</h2>
        <TrainerFilterSelector
          :trainer_class="group_a.trainer_class"
          :trainer_id="group_a.trainer_id"
          :number_of_mons="group_a.number_of_mons"
          :mons="group_a.mons"
          @filter_updated="
            (filter) => {
              update_trainer_a(filter)
            }
          "
        />
      </div>
      <div class="trainer-filter">
        <h2 class="group-title">GROUP B</h2>
        <TrainerFilterSelector
          :trainer_class="group_b.trainer_class"
          :trainer_id="group_b.trainer_id"
          :number_of_mons="group_b.number_of_mons"
          :mons="group_b.mons"
          @filter_updated="
            (filter) => {
              update_trainer_b(filter)
            }
          "
        />
      </div>
    </div>
    <div class="finder">
      <LoadingCard v-if="loading" />
      <div v-else>
        <div class="find">
          <button @click="find()" :disabled="button_disabled">
            {{ button_disabled ? 'FOUND' : 'FIND' }}
          </button>
        </div>
        <p v-if="error">{{ error }}</p>
        <div class="series-search">
          <div v-if="series_search">
            <p>SERIES COUNT:{{ series_search.series.length }}</p>
            <p>
              GROUP A W% {{ (group_a_win_per * 100).toFixed(2) }} GROUP B W%
              {{ (group_b_win_per * 100).toFixed(2) }}
            </p>
            <SeriesSelector
              :series="series_search.series"
              :extra_trainer_attributes="extra_trainer_attributes"
              @updated_series="(updated) => (series_id = updated)"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.finder {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
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

@media (max-width: 800px) {
  .trainer-filter-container {
    flex-direction: column;
  }
}

.trainer-filter {
  width: 50%;
}

.group-title {
  text-align: center;
}
</style>
