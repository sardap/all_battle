<script setup lang="ts">
import { get_single_trainer_stats, type SeriesFilter, type TrainerStats } from '@/api'
import SeriesSelector from './SeriesSelector.vue'
import LoadingCard from '@/components/LoadingCard.vue'
import { get_trainer, type Trainer, trainer_pic_to_path } from '@/gen_3'
import { onMounted, ref } from 'vue'
import { kd_ratio, win_percent } from '@/utils'
import BattleCardHolder from './BattleCardHolder.vue'
import TrainerMonsFocus from './TrainerMonsFocus.vue'
import TrainerGameProgress from './TrainerGameProgress.vue'

const props = defineProps<{
  trainer_id: number
  filter: SeriesFilter
}>()

const trainer = ref<Trainer>(get_trainer(props.trainer_id))
const stats = ref<TrainerStats | null>(null)
const loading = ref(false)
const selected_series = ref<number>(0)
const murders_sum = ref(0)
const deaths_sum = ref(0)
const damage_dealt_sum = ref(0)
const damage_taken_sum = ref(0)

onMounted(async () => {
  loading.value = true
  stats.value = await get_single_trainer_stats(props.trainer_id, props.filter)

  selected_series.value = stats.value.series[0].series_id

  murders_sum.value = 0
  deaths_sum.value = 0
  damage_dealt_sum.value = 0
  damage_taken_sum.value = 0
  for (let i = 0; i < stats.value.rank.mons.length; i++) {
    murders_sum.value += stats.value.rank.mons[i].murders
    deaths_sum.value += stats.value.rank.mons[i].deaths
    damage_dealt_sum.value += stats.value.rank.mons[i].damage_dealt
    damage_taken_sum.value += stats.value.rank.mons[i].damage_taken
  }

  loading.value = false
})
</script>

<template>
  <div class="trainer-focus">
    <LoadingCard height="500px" v-if="loading" />
    <div v-else-if="stats">
      <div>
        <div>
          <p>RANK:{{ stats.rank.overall_rank + 1 }}</p>
          <p>
            #{{ trainer.id }} {{ trainer.trainerName }}
            {{ trainer.trainerClass.replace('TRAINER_NAME_', '') }}
          </p>
        </div>
        <div>
          <img class="trainer-pic" :src="trainer_pic_to_path(trainer.trainerPic)" />
        </div>
      </div>
      <div>
        <p>
          WR:{{ win_percent(stats.rank.wins, stats.rank.total) }}% ({{ stats.rank.wins }},
          {{ stats.rank.total }})
        </p>
        <div v-if="trainer.partySize > 1">
          <p>K:{{ murders_sum }} D:{{ deaths_sum }} KDR:{{ kd_ratio(murders_sum, deaths_sum) }}</p>
          <p>DAMAGE DEALT:{{ damage_dealt_sum }} DAMAGE TAKEN:{{ damage_taken_sum }}</p>
        </div>
      </div>
      <br />
      <div>
        <p>TRAINER MONS</p>
        <TrainerMonsFocus :trainer_id="trainer_id" :stats="stats.rank" />
      </div>
      <br />
      <div>
        <p>HOW FAR WOULD THIS TRAINER MAKE IT IN EMERALD?</p>
        <TrainerGameProgress :trainer_id="trainer_id" />
      </div>
      <br />
      <div>
        <p>VIEW A SERIES</p>
        <SeriesSelector
          :series="stats.series"
          @updated_series="(x) => (selected_series = x)"
          :featured_trainer="trainer_id"
          :selected_series="selected_series"
        />
      </div>
      <BattleCardHolder :key="selected_series" :series_id="selected_series" />
    </div>
  </div>
</template>

<style scoped>
.trainer-focus {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 4px;
  text-align: center;
}

.trainer-pic {
  margin: 10px;
  width: 72px;
}
</style>
