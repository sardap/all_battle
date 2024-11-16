<script setup lang="ts">
import CompletionCard from '@/components/CompletionCard.vue'
import MonStatsCard from '@/components/MonStatsCard.vue'
import { onMounted, ref } from 'vue'
import { random_trainer, random_trainer_mon, type Mon, type Trainer } from '@/gen_3'
import {
  get_single_mon_stats,
  get_single_trainer_stats,
  type SingleMonStats,
  type TrainerRank
} from '@/api'
import LoadingCard from '@/components/LoadingCard.vue'
import { random_mon_stats, random_trainer_stats } from '@/utils'
import BattleCardHolder from '@/components/BattleCardHolder.vue'
import TrainerStatsCard from '@/components/TrainerStatsCard.vue'

const random_mon = ref<{ mon: Mon; stats: SingleMonStats } | null>(null)
const selected_trainer = ref<{ trainer: Trainer; stats: TrainerRank } | null>(null)
const featured_series_id = ref<number>(random_series_id())

onMounted(async () => {
  await refresh_random_mon()
  await refresh_random_trainer()
})

function random_series_id() {
  return Math.floor(Math.random() * 364231)
}

async function refresh_random_mon() {
  random_mon.value = null
  const selected_mon = random_trainer_mon()
  const selected_mon_stats = await get_single_mon_stats(selected_mon.id)
  random_mon.value = {
    mon: selected_mon,
    stats: selected_mon_stats
  }
}

async function refresh_random_trainer() {
  selected_trainer.value = null
  const trainer = random_trainer()
  const selected_mon_stats = await get_single_trainer_stats(trainer.id)
  selected_trainer.value = {
    trainer: trainer,
    stats: selected_mon_stats.rank
  }
}
</script>

<template>
  <div class="home-view">
    <div class="header">
      <h1>WELCOME TO THE WORLD OF RANKINGS</h1>
      <CompletionCard />
    </div>
    <div>
      <h2>RANDOM MON FACTS!</h2>
      <LoadingCard height="300px" v-if="random_mon === null" />
      <div v-else class="col-container centered">
        <MonStatsCard
          :key="random_mon.mon.id"
          :mon="random_mon.mon"
          :stats="random_mon.stats"
          :to_show="random_mon_stats(random_mon.stats, 100)"
        />
        <button @click="refresh_random_mon">NEXT</button>
      </div>
    </div>
    <br />
    <div>
      <h2>RANDOM TRAINER FACTS!</h2>
      <LoadingCard height="300px" v-if="selected_trainer === null" />
      <div v-else class="col-container centered">
        <TrainerStatsCard
          :key="selected_trainer.trainer.id"
          :trainer="selected_trainer"
          :to_show="random_trainer_stats(selected_trainer.trainer, 100)"
        />
        <button @click="refresh_random_trainer">NEXT</button>
      </div>
    </div>
    <br />
    <div>
      <h2>A RANDOM SERIES</h2>
      <h3 v-if="featured_series_id !== null">{{ featured_series_id }}</h3>
      <LoadingCard v-if="featured_series_id === null" />
      <BattleCardHolder v-else :series_id="featured_series_id" :key="featured_series_id" />
      <button @click="featured_series_id = random_series_id()">NEXT</button>
    </div>
  </div>
</template>

<style scoped>
.home-view {
  text-align: center;
}

.header {
  margin-bottom: 20px;
  text-align: center;
}
</style>
