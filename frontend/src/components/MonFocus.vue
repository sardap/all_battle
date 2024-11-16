<script setup lang="ts">
import { get_mon_by_id, get_move_by_id, get_type_icon_path, type Move } from '@/gen_3'
import { onMounted, ref, watch } from 'vue'
import LoadingCard from './LoadingCard.vue'
import {
  get_mon_series,
  get_series,
  get_single_mon_stats,
  type MonSeriesResponse,
  type MoveStats,
  type SeriesResponse,
  type SingleMonStats
} from '@/api'
import { kd_ratio } from '@/utils'
import AnimeMon from './AnimeMon.vue'
import SeriesSelector from './SeriesSelector.vue'
import BattleCardHolder from './BattleCardHolder.vue'

const props = defineProps<{
  mon_id: number
}>()

const mon = ref(get_mon_by_id(props.mon_id))
const moves = ref<{ stats: MoveStats; move: Move }[]>([])
const stats = ref<SingleMonStats | null>(null)
const mon_series = ref<MonSeriesResponse | null>(null)
const loading = ref(true)

const selected_series_id = ref<number | undefined>(undefined)
const series = ref<SeriesResponse | null>(null)
const series_loading = ref(false)

watch(selected_series_id, async (new_series) => {
  if (new_series) {
    series_loading.value = true
    series.value = await get_series(new_series)
    series_loading.value = false
  } else {
    series.value = null
  }
})

onMounted(async () => {
  loading.value = true
  stats.value = await get_single_mon_stats(props.mon_id)
  moves.value = stats.value.moves
    .filter((move) => {
      return move.times_used > 0
    })
    .map((move) => {
      return {
        stats: move,
        move: get_move_by_id(move.move_id)
      }
    })
  moves.value.sort((a, b) => b.stats.times_used - a.stats.times_used)
  mon_series.value = await get_mon_series(props.mon_id)
  selected_series_id.value = mon_series.value.series[0].series_id
  loading.value = false
})
</script>

<template>
  <LoadingCard height="500px" v-if="loading" />
  <div v-else class="col-container centered">
    <h3>#{{ mon.id }} {{ mon.name }}</h3>
    <p><img v-for="type in mon.monTypes" :key="type" :src="get_type_icon_path(type)" /></p>
    <div>
      <AnimeMon :mon="mon" />
    </div>
    <div v-if="stats">
      <p v-if="mon_series">
        WIN RATE:{{ ((mon_series.wins / mon_series.total) * 100).toFixed(2) }}% ({{
          mon_series.wins
        }}, {{ mon_series.total }})
      </p>
      <p>TIMES RELEASED:{{ stats.battles }}</p>
      <p>
        KDR:{{ kd_ratio(stats.murders, stats.deaths) }} ({{ stats.murders }}, {{ stats.deaths }})
      </p>
      <p>DAMAGE DEALT:{{ stats.damage_dealt }}</p>
      <p>DAMAGE TAKEN:{{ stats.damage_taken }}</p>
      <br />
      <h4>MOVES</h4>
      <div class="move-grid-container" :style="`grid-template-columns: repeat(2, 1fr);`">
        <div class="grid-item" v-for="(move, j) in moves" :key="move.move.id">
          <p>{{ move.move.name }} <img :src="get_type_icon_path(move.move.type)" /></p>
          <p>TIMES USED:{{ move.stats.times_used }}</p>
          <p v-if="move.stats.times_used > 0">DAMAGE DEALT:{{ move.stats.damage_dealt }}</p>
          <p v-if="move.stats.times_used > 0">KO'S:{{ move.stats.murders }}</p>
          <hr v-if="j < moves.length - 2" />
        </div>
      </div>
    </div>
    <br />
    <div v-if="mon_series">
      <p>VIEW A SERIES</p>
      <SeriesSelector
        :series="mon_series.series"
        @updated_series="(x) => (selected_series_id = x)"
        :selected_series="selected_series_id"
      />
    </div>
    <BattleCardHolder :key="selected_series_id" :series_id="selected_series_id" />
  </div>
</template>

<style scoped lang="scss">
div {
  text-align: center;
}

hr {
  margin-right: 2px;
  margin-left: 2px;
}

.move-grid-container {
  display: grid;
  max-width: 400px;
  margin: auto;
}

.grid-item {
  text-align: center;
  margin-bottom: 10px;
}
</style>
