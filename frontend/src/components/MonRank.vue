<script setup lang="ts">
import { ref } from 'vue'
import { type MonRanking, type MonRankResponse } from '@/api'
import { get_mon_by_id, mon_to_front_pic_path, get_type_icon_path, type Mon } from '@/gen_3'
import { kd_ratio } from '@/utils'
import { type MonRankLocalFilter } from '@/components/MonRankLocalFilterSelector.vue'
import UnknownWriting from './UnknownWriting.vue'
import type { MonOrderBy, MonOrderByField } from './MonRankOrderBy.vue'

const emit = defineEmits<{
  (e: 'mon_selected', mon_id: number): void
}>()

const props = defineProps<{
  rank: MonRankResponse
  order_by: MonOrderBy
  filter: MonRankLocalFilter
  selected_mon?: number
}>()

const mons = ref(get_mons())

function compare(
  order_by: MonOrderByField,
  descending: boolean,
  a: MonRanking,
  b: MonRanking
): number {
  let left = 0
  let right = 0

  if (order_by === 'exists') {
    left = a.number_exist
    right = b.number_exist
  } else if (order_by === 'released') {
    left = a.times_released
    right = b.times_released
  } else if (order_by === 'kdr') {
    left = a.deaths === 0 ? a.murders : a.murders / a.deaths
    right = b.deaths === 0 ? b.murders : b.murders / b.deaths
  } else if (order_by === 'avg_lvl') {
    left = a.average_level
    right = b.average_level
  }

  if (descending) {
    return right - left
  }
  return left - right
}

function matches_filter(rank: MonRanking, mon: Mon): boolean {
  const filter = props.filter
  if (filter.mon_type && !mon.monTypes.includes(filter.mon_type)) {
    return false
  }

  if (filter.name && !mon.name.toLowerCase().includes(filter.name.toLowerCase())) {
    return false
  }

  if (filter.min_exists && rank.number_exist < filter.min_exists) {
    return false
  }

  if (filter.max_exists && rank.number_exist > filter.max_exists) {
    return false
  }

  {
    const kdr = rank.deaths === 0 ? rank.murders : rank.murders / rank.deaths
    if (filter.min_kdr && kdr < filter.min_kdr) {
      return false
    }
    if (filter.max_kdr && kdr > filter.max_kdr) {
      return false
    }
  }

  if (filter.min_avg_lvl && rank.average_level < filter.min_avg_lvl) {
    return false
  }

  if (filter.max_avg_lvl && rank.average_level > filter.max_avg_lvl) {
    return false
  }

  return true
}

function get_mons() {
  const result = []
  for (let i = 0; i < props.rank.mons.length; i++) {
    const rank = props.rank.mons[i]
    const mon = get_mon_by_id(rank.mon_id)
    if (rank.times_released > 0 && matches_filter(rank, mon)) {
      result.push({
        rank: rank,
        mon: mon
      })
    }
  }

  result.sort((a, b) => {
    const result = compare(props.order_by.field, props.order_by.descending, a.rank, b.rank)
    if (result !== 0) {
      return result
    }

    return b.mon.id - a.mon.id
  })

  return result
}
</script>

<template>
  <div v-if="mons.length > 0">
    <div v-for="(mon, i) in mons" :key="mon.mon.id" class="col-container centered">
      <p>RANK:{{ i + 1 }}</p>
      <p>#{{ mon.mon.id }} {{ mon.mon.name }}</p>
      <p><img v-for="type in mon.mon.monTypes" :key="type" :src="get_type_icon_path(type)" /></p>
      <img :src="mon_to_front_pic_path(mon.mon)" />
      <p>EXISTS:{{ mon.rank.number_exist }}</p>
      <p>RELEASED:{{ mon.rank.times_released }}</p>
      <p>KDR:{{ kd_ratio(mon.rank.murders, mon.rank.deaths) }}</p>
      <p>AVG LVL:{{ mon.rank.average_level.toFixed(0) }}</p>
      <button
        :disabled="mon.mon.id === selected_mon"
        @click="() => emit(`mon_selected`, mon.mon.id)"
      >
        {{ mon.mon.id === selected_mon ? `SELECTED` : `SEE MORE` }}
      </button>
      <hr v-if="i !== mons.length - 1" />
    </div>
  </div>
  <div v-else>
    <UnknownWriting message="no matches" />
  </div>
</template>

<style scoped lang="scss">
hr {
  width: 100%;
  margin: 0;
  margin-top: 5px;
  margin-bottom: 5px;
}
</style>
