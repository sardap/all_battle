<script setup lang="ts">
import { get_trainers_rank, type SeriesFilter, type TrainerRankBrief } from '@/api'
import TrainerFilterSelector from '@/components/TrainerFilterSelector.vue'
import { onMounted, ref, watch } from 'vue'
import LoadingCard from '@/components/LoadingCard.vue'
import TrainerFocus from '@/components/TrainerFocus.vue'
import { win_percent } from '@/utils'
import UnknownWriting from '@/components/UnknownWriting.vue'
import {
  get_mon,
  get_trainer,
  type Trainer,
  trainer_pic_to_path,
  mon_to_icon_pic_path
} from '@/gen_3'
import TrainerRankFilterSelector, {
  type TrainerRankFilterOptions
} from '@/components/TrainerRankFilterSelector.vue'
import TrainerRankOrderBy, { type TrainerOrderBy } from '@/components/TrainerRankOrderBy.vue'
import { useRoute } from 'vue-router'
import router from '@/router'

const filter = ref<SeriesFilter>(get_filter_from_query())
const rank = ref<{ rank: TrainerRankBrief; trainer: Trainer }[]>([])
const loading = ref(false)
const selected_trainer_id = ref<number>(get_selected_trainer_from_query())
const rank_filter = ref<TrainerRankFilterOptions>(get_rank_filter_from_query())
const order_by = ref<TrainerOrderBy>(get_order_by_from_query())
const filtered_rank = ref<{ rank: TrainerRankBrief; trainer: Trainer }[]>([])
const find_dirty = ref(false)

watch([rank_filter, filter, order_by, selected_trainer_id], () => {
  router.push({
    query: {
      selected_trainer_id: selected_trainer_id.value,
      filter: encodeURIComponent(JSON.stringify(filter.value)),
      rank_filter: encodeURIComponent(JSON.stringify(rank_filter.value)),
      order_by: encodeURIComponent(JSON.stringify(order_by.value))
    }
  })
})

watch(filter, () => {
  find_dirty.value = true
})

watch(order_by, () => {
  update_rank_order()
})

watch(rank_filter, () => {
  update_fileted_rank()
})

function get_selected_trainer_from_query() {
  const route = useRoute()
  if (route.query.selected_trainer_id) {
    return parseInt(route.query.selected_trainer_id.toString())
  }
  return 0
}

function get_filter_from_query() {
  const route = useRoute()
  if (route.query.filter) {
    return JSON.parse(decodeURIComponent(route.query.filter.toString()))
  }
  return {}
}

function get_rank_filter_from_query() {
  const route = useRoute()
  if (route.query.rank_filter) {
    return JSON.parse(decodeURIComponent(route.query.rank_filter.toString()))
  }
  return {}
}

function get_order_by_from_query() {
  const route = useRoute()
  if (route.query.order_by) {
    return JSON.parse(decodeURIComponent(route.query.order_by.toString()))
  }
  return { field: 'win_rate', descending: true }
}

onMounted(async () => {
  find(true)
})

async function find(first_run: boolean = false) {
  loading.value = true
  const response = await get_trainers_rank(filter.value)
  const updated = []
  for (const trainer of response.trainers) {
    const trainer_data = get_trainer(trainer.id)
    updated.push({ rank: trainer, trainer: trainer_data })
  }
  rank.value = updated
  if (!first_run) {
    rank_filter.value = {}
    selected_trainer_id.value = 0
  }
  update_fileted_rank()
  loading.value = false
  find_dirty.value = false
}

function update_fileted_rank() {
  const updated = []
  for (const trainer of rank.value) {
    if (
      rank_filter.value.show_class &&
      trainer.trainer.trainerClass !== rank_filter.value.show_class
    ) {
      continue
    }

    if (
      rank_filter.value.show_trainer_name &&
      !trainer.trainer.trainerName.includes(rank_filter.value.show_trainer_name.toUpperCase())
    ) {
      continue
    }

    if (rank_filter.value.includes_mons && rank_filter.value.includes_mons.length > 0) {
      let include = true
      for (const must_include of rank_filter.value.includes_mons) {
        let found = false
        for (const mon of trainer.trainer.party) {
          if (mon.species === must_include) {
            found = true
            break
          }
        }
        if (!found) {
          include = false
          break
        }
      }
      if (!include) {
        continue
      }
    }

    if (
      rank_filter.value.mon_count &&
      trainer.trainer.party.length !== rank_filter.value.mon_count
    ) {
      continue
    }

    updated.push(trainer)
  }
  filtered_rank.value = updated
  update_rank_order()
}

function expand(trainer_id: number) {
  selected_trainer_id.value = trainer_id
}

function update_rank_order() {
  const updated = [...filtered_rank.value]
  updated.sort((a, b) => {
    let left = 0
    let right = 0
    switch (order_by.value.field) {
      case 'win_rate':
        left = b.rank.overall_rank
        right = a.rank.overall_rank
        break
      case 'battle_count':
        left = a.rank.total
        right = b.rank.total
        break
    }

    let result: number = 0

    if (order_by.value.descending) {
      result = right - left
    } else {
      result = left - right
    }

    if (result === 0) {
      return a.rank.id - b.rank.id
    }

    return result
  })
  filtered_rank.value = updated
}
</script>

<template>
  <div class="trainers-view">
    <h1>TRAINERS</h1>
    <h2>TRAINERS RANK FILTER</h2>
    <div class="trainer-filter-container">
      <TrainerFilterSelector
        class="trainer-filter"
        :trainer_class="filter.trainer_class"
        :trainer_id="filter.trainer_id"
        :number_of_mons="filter.number_of_mons"
        :mons="filter.mons"
        @filter_updated="
          (updated) => {
            filter = updated
          }
        "
      />
    </div>
    <button
      @click="
        () => {
          find()
        }
      "
      :disabled="!find_dirty"
    >
      {{ find_dirty ? `UPDATE` : `UPDATED` }}
    </button>
    <hr />
    <div class="col-container centered">
      <h2>TRAINERS FILTER</h2>
      <LoadingCard v-if="loading" />
      <div v-else-if="find_dirty">
        <p style="text-align: center">PLEASE PRESS UPDATE WHEN READY</p>
        <UnknownWriting message="Python suxs" />
      </div>
      <div v-else class="">
        <TrainerRankFilterSelector
          :trainers="filtered_rank"
          :show_class="rank_filter.show_class"
          :show_trainer_name="rank_filter.show_trainer_name"
          :includes_mons="rank_filter.includes_mons"
          :mon_count="rank_filter.mon_count"
          @filter_updated="(x) => (rank_filter = x)"
        />
        <br />
        <div class="col-container centered">
          <h3>ORDER BY</h3>
          <TrainerRankOrderBy
            :field="order_by.field"
            :descending="order_by.descending"
            @updated="(x) => (order_by = x)"
          />
        </div>
        <br />
        <div class="trainer-rank">
          <div v-if="filtered_rank.length === 0">
            <p style="text-align: center">NO TRAINERS FOUND TRY UPDATING TRAINER FILTER</p>
          </div>
          <div v-else v-for="(trainer, i) in filtered_rank" :key="trainer.rank.id">
            <p :class="`rank-title`">RANK:{{ trainer.rank.overall_rank + 1 }}</p>
            <div class="trainer-rank-title">
              <img :src="trainer_pic_to_path(trainer.trainer.trainerPic)" />
              <div class="trainer-name-section">
                <p>
                  #{{ trainer.rank.id }} {{ trainer.trainer.trainerName }}
                  {{ trainer.trainer.trainerClass.replace('TRAINER_CLASS_', '') }}
                </p>
                <p>
                  {{ win_percent(trainer.rank.wins, trainer.rank.total) }}% ({{
                    trainer.rank.wins
                  }}, {{ trainer.rank.total }})
                </p>
              </div>
            </div>
            <div class="trainer-mon-section">
              <div>
                <div
                  class="row-container trainer-mon-entries"
                  v-for="(trainer_mon, j) in trainer.trainer.party"
                  :key="j"
                >
                  <img class="mon-icon" :src="mon_to_icon_pic_path(get_mon(trainer_mon.species))" />
                  <p>
                    {{ get_mon(trainer_mon.species).name }} lvl:{{ trainer_mon.lvl }} iv:{{
                      trainer_mon.iv
                    }}
                  </p>
                </div>
              </div>
            </div>
            <div class="trainer-expand">
              <button
                @click="() => expand(trainer.rank.id)"
                :disabled="selected_trainer_id === trainer.trainer.id"
              >
                {{ selected_trainer_id === trainer.trainer.id ? `SELECTED` : `SEE MORE` }}
              </button>
            </div>
            <hr class="trainer-rank-line" v-if="i !== filtered_rank.length - 1" />
          </div>
        </div>
      </div>
    </div>
    <hr />
    <div>
      <h2>ALL ABOUT TRAINER</h2>
      <div v-if="selected_trainer_id > 0" :key="selected_trainer_id">
        <TrainerFocus :trainer_id="selected_trainer_id" :filter="filter" />
      </div>
      <div v-else>
        <p style="text-align: center">PRESS SEE MORE ON A TRAINER</p>
        <UnknownWriting message="what the fuck did you say to me you little shit" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.trainer-filter-container {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
}

hr {
  margin-top: 1em;
  width: 70%;
}

.trainer-rank-line {
  width: 100%;
}

.trainer-mon-section {
  margin-left: 41px;
}

.mon-icon {
  width: 32px;
}

.trainer-name-section {
  width: 250px;
}

.trainer-mon-entries {
  justify-content: left;
  align-items: center;
}

.trainer-rank-title {
  display: flex;
  flex-direction: row;
}

.rank-title {
  font-size: 20px;
  text-align: center;
}

.trainer-rank {
  height: 400px;
  overflow: scroll;
}

.trainer-rank-controller select {
  margin-right: 2px;
}

.trainer-rank-controller div {
  margin-top: 5px;
}

.trainer-rank-controller .section-header {
  margin-top: 10px;
  text-align: center;
}

.trainer-rank-controller {
  overflow-x: clip;
  width: 100%;
}

.trainer-rank-controller input {
  width: 50px;
}

.trainer-expand {
  max-width: fit-content;
  margin-left: auto;
  margin-right: auto;
}

.trainers-view {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}
</style>
