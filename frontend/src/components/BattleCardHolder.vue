<script setup lang="ts">
import UnknownWriting from './UnknownWriting.vue'
import LoadingCard from './LoadingCard.vue'
import BattleCard from './BattleCard.vue'
import { onMounted, ref } from 'vue'
import { get_series, type SeriesResponse } from '@/api'
import BattleCardStats from './BattleCardStats.vue'
import { random_copy_pasta_sentence } from '@/utils'

const props = defineProps<{
  series_id?: number
  battle_id?: number
  hide_game_selector?: boolean
}>()

const loading = ref(true)
const series = ref<SeriesResponse | null>(null)
const battle_index = ref(0)
const hide_game_selector = props.hide_game_selector ? props.hide_game_selector : false

onMounted(async () => {
  loading.value = true
  if (!props.series_id) {
    loading.value = false
    return
  }
  series.value = await get_series(props.series_id)
  if (props.battle_id) {
    battle_index.value = series.value.battles.findIndex((battle) => battle.id === props.battle_id)
  }
  loading.value = false
})
</script>

<template>
  <div class="battle-card-holder" :key="series_id">
    <LoadingCard v-if="loading" />
    <div v-else-if="series">
      <div class="row-container centered game-header" v-if="!hide_game_selector">
        <h1>GAME</h1>
        <select v-model="battle_index">
          <option v-for="(_, i) in series.battles" :value="i" :key="i">{{ i + 1 }}</option>
        </select>
      </div>
      <hr v-if="!hide_game_selector" />
      <BattleCard
        :key="battle_index"
        :series="series"
        :battle_index="battle_index"
        :show_controls="true"
      />
      <hr />
      <h2>STATS</h2>
      <BattleCardStats :key="battle_index" :battle_id="series.battles[battle_index].id" />
    </div>
    <div v-else>
      <UnknownWriting :message="random_copy_pasta_sentence()" />
    </div>
  </div>
</template>

<style scoped>
.game-header {
  margin-top: 20px;
  gap: 5px;
}

hr {
  margin-left: 10px;
  margin-right: 10px;
  margin-bottom: 20px;
  margin-top: 20px;
}

.battle-card-holder {
  text-align: center;
  margin: 10px;
  background-color: #ff00000e;
  border: 5px solid #ff0000;
  min-height: 500px;
  min-width: 900px;
}

@media (max-width: 800px) {
  .battle-card-holder {
    min-width: 80%;
    flex-direction: column;
  }
}
</style>
