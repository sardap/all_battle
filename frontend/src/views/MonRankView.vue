<script setup lang="ts">
import { get_mon_rank, type MonRankResponse } from '@/api'
import { onMounted, ref, watch } from 'vue'
import MonRank from '@/components/MonRank.vue'
import LoadingCard from '@/components/LoadingCard.vue'
import MonFocus from '@/components/MonFocus.vue'
import MonRankOrderBy, { type MonOrderBy } from '@/components/MonRankOrderBy.vue'
import MonRankLocalFilterSelector from '@/components/MonRankLocalFilterSelector.vue'
import { type MonRankLocalFilter } from '@/components/MonRankLocalFilterSelector.vue'
import UnknownWriting from '@/components/UnknownWriting.vue'
import { random_copy_pasta_sentence } from '@/utils'
import { useRoute } from 'vue-router'
import router from '@/router'

const loading_rank = ref(true)
const rank = ref<MonRankResponse | null>(null)
const selected_mon_id = ref<number>(0)
const order_by = ref<MonOrderBy>(parse_order_by_from_query())
const filter = ref<MonRankLocalFilter>(parse_filter_from_query())
const refresh_ranking = ref(0)

watch([filter, order_by], () => {
  refresh_ranking.value += 1
})

watch([selected_mon_id, filter, order_by], () => {
  router.push({
    query: {
      selected_mon_id: selected_mon_id.value.toString(),
      filter: encodeURIComponent(JSON.stringify(filter.value)),
      order_by: encodeURIComponent(JSON.stringify(order_by.value))
    }
  })
})

function parse_order_by_from_query(): MonOrderBy {
  const route = useRoute()
  if (route.query.order_by) {
    console.log(route.query.order_by)
    return JSON.parse(decodeURIComponent(route.query.order_by.toString()))
  }
  return { field: 'kdr', descending: true }
}

function parse_filter_from_query() {
  const route = useRoute()
  if (route.query.filter) {
    return JSON.parse(decodeURIComponent(route.query.filter.toString()))
  }
  return {}
}

onMounted(async () => {
  const route = useRoute()
  if (route.query.selected_mon_id) {
    selected_mon_id.value = parseInt(route.query.selected_mon_id.toString())
  }

  loading_rank.value = true
  const response = await get_mon_rank({})
  rank.value = response
  loading_rank.value = false
})
</script>

<template>
  <div class="header">
    <h1>GROUPED MONS</h1>
  </div>
  <div class="col-container centered">
    <h2>RANKING</h2>
    <div>
      <h3>FILTER</h3>
      <MonRankLocalFilterSelector
        :mon_type="filter.mon_type"
        :name="filter.name"
        :min_exists="filter.min_exists"
        :max_exists="filter.max_exists"
        :min_kdr="filter.min_kdr"
        :max_kdr="filter.max_kdr"
        :min_avg_lvl="filter.min_avg_lvl"
        :max_avg_lvl="filter.max_avg_lvl"
        @updated="(updated_filter) => (filter = updated_filter)"
      />
    </div>
    <br />
    <div>
      <h3>ORDER BY</h3>
      <MonRankOrderBy
        :field="order_by.field"
        :descending="order_by.descending"
        @updated="(updated_order_by) => (order_by = updated_order_by)"
      />
    </div>
    <hr />
    <div class="mon-rank">
      <LoadingCard v-if="loading_rank" />
      <div v-else-if="rank">
        <MonRank
          :key="refresh_ranking"
          :rank="rank"
          :order_by="order_by"
          :filter="filter"
          @mon_selected="(updated) => (selected_mon_id = updated)"
          :selected_mon="selected_mon_id"
        />
      </div>
    </div>
    <hr />
  </div>
  <div class="col-container centered" :key="selected_mon_id">
    <h2>MON FOCUS</h2>
    <div v-if="selected_mon_id === 0">
      <p>SELECT A MON TO SEE MORE</p>
      <UnknownWriting :message="random_copy_pasta_sentence()" />
    </div>
    <MonFocus v-else :mon_id="selected_mon_id" />
  </div>
</template>

<style scoped>
p {
  text-align: center;
}

h3 {
  text-align: center;
}

hr {
  width: 70%;
  margin: 0;
  margin-top: 5px;
  margin-bottom: 5px;
}

.mon-rank {
  height: 400px;
  width: 400px;
  max-width: 100%;
  overflow: scroll;
}
</style>
