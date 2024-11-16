<script setup lang="ts">
import { get_battle_search, type BattleSearchQueryResponse } from '@/api'
import { onMounted, ref, watch } from 'vue'
import BattlesPage from '@/components/BattlesPage.vue'
import LoadingCard from '@/components/LoadingCard.vue'
import BattleCardHolder from '@/components/BattleCardHolder.vue'
import BattleFilter, { type BattleSearchFilter } from '@/components/BattleFilter.vue'
import router from '@/router'
import BattleOrderBySelector, { type BattleOrderBy } from '@/components/BattleOrderBySelector.vue'

const loading = ref(true)
const filter = ref<BattleSearchFilter>(parse_filter_from_query())
const order_by = ref<BattleOrderBy>(prase_order_by_from_query())
const filter_dirty = ref(false)
const current_page = ref<BattleSearchQueryResponse | null>(null)
const selected_battle_id = ref<{ battle_id: number; series_id: number } | null>(null)

watch([filter, order_by], () => {
  filter_dirty.value = true
  router.push({
    query: {
      filter: JSON.stringify(filter.value),
      order_by: JSON.stringify(order_by.value)
    }
  })
})

onMounted(async () => {
  await get_next()
})

function parse_filter_from_query(): BattleSearchFilter {
  const route = router.currentRoute.value
  if (route.query.filter) {
    return JSON.parse(route.query.filter.toString())
  }
  return {}
}

function prase_order_by_from_query(): BattleOrderBy {
  const route = router.currentRoute.value
  if (route.query.order_by) {
    return JSON.parse(route.query.order_by.toString())
  }
  return { field: 'id', descending: false }
}

const page_limit = 4

async function refresh_page() {
  current_page.value = null
  filter_dirty.value = false
  await get_next()
}

async function get_next() {
  loading.value = true

  const offset = current_page.value ? current_page.value.offset + current_page.value.data.length : 0
  const limit = page_limit

  const response = await get_battle_search({
    offset,
    limit,
    order_by_field: order_by.value.field,
    order_by_descending: order_by.value.descending,
    ...filter.value
  })
  current_page.value = response
  loading.value = false
}

async function get_previous() {
  loading.value = true

  const limit = page_limit
  let offset = current_page.value ? current_page.value.offset - limit : 0

  if (offset < 0) {
    offset = 0
  }

  const response = await get_battle_search({
    offset,
    limit,
    ...filter.value
  })
  current_page.value = response
  loading.value = false
}
</script>

<template>
  <div class="battle-view">
    <h1>BATTLES</h1>
    <div>
      <div class="col-container centered">
        <BattleFilter
          :min_length_seconds="filter.min_length_seconds"
          :max_length_seconds="filter.max_length_seconds"
          :seed="filter.seed"
          :player_perspective_won="filter.player_perspective_won"
          :min_kos="filter.min_kos"
          :max_kos="filter.max_kos"
          :min_damage_dealt="filter.min_damage_dealt"
          :max_damage_dealt="filter.max_damage_dealt"
          :mons_included="filter.mons_included"
          :mons_excluded="filter.mons_excluded"
          :moves_used="filter.moves_used"
          :moves_not_used="filter.moves_not_used"
          @updated="(x) => (filter = x)"
        />
        <p>ORDER BY</p>
        <BattleOrderBySelector
          :field="order_by.field"
          :descending="order_by.descending"
          @updated="(x) => (order_by = x)"
        />
        <button @click="refresh_page" :disabled="!filter_dirty">REFRESH</button>
      </div>
      <div>
        <h2>BATTLES VIEW</h2>
        <div v-if="current_page && !loading">
          <p>TOTAL:{{ current_page.total }}</p>
          <p>
            PAGE {{ current_page.offset / current_page.limit + 1 }} OF
            {{ Math.ceil(current_page.total / current_page.limit) }}
          </p>
        </div>
        <br />
        <div class="battle-page">
          <LoadingCard v-if="loading" height="100%" />
          <div v-else-if="current_page !== null && current_page.data.length > 0">
            <BattlesPage
              :page="current_page"
              :next_possible="current_page.offset + current_page.data.length < current_page.total"
              :current_selected="selected_battle_id"
              @select-battle="selected_battle_id = $event"
            />
          </div>
        </div>
        <br />
        <div v-if="current_page && !loading">
          <button class="page-button" @click="get_previous" :disabled="current_page.offset === 0">
            PREVIOUS
          </button>
          <button
            class="page-button"
            @click="get_next"
            :disabled="current_page.offset + current_page.limit > current_page.total"
          >
            NEXT
          </button>
        </div>
      </div>
    </div>
    <div>
      <BattleCardHolder
        :key="selected_battle_id ? selected_battle_id.series_id + selected_battle_id.battle_id : 0"
        :battle_id="selected_battle_id?.battle_id"
        :series_id="selected_battle_id?.series_id"
      />
    </div>
  </div>
</template>

<style scoped>
.battle-view {
  text-align: center;
}

.page-button {
  margin: 10px;
  width: 100px;
}

.battle-page {
  height: 790px;
  overflow-y: scroll;
}
</style>
