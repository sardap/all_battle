<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from 'vue'
import TrainerCard from './TrainerCard.vue'
import {
  get_active_mon_index_in_battle,
  get_fainted_mons_in_battle,
  get_trainer_history,
  type SeriesResponse,
  type TrainerHistory
} from '@/api'
import LoadingCard from './LoadingCard.vue'

const props = defineProps<{
  series: SeriesResponse
  battle_index: number
  show_controls: Boolean
}>()

const video = ref<HTMLVideoElement | null>(null)

interface PlayerState {
  active_mon_index: number
  fainted_mons: number[]
}

const player_state = ref<PlayerState>({
  active_mon_index: 0,
  fainted_mons: []
})
const opponent_state = ref<PlayerState>({
  active_mon_index: 0,
  fainted_mons: []
})

const loading = ref(true)
const player_history = ref<TrainerHistory | null>(null)
const opponent_history = ref<TrainerHistory | null>(null)
const refresh_video = ref(0)
const battle_index = ref(props.battle_index)
const battle = ref(props.series.battles[battle_index.value])

const video_text_interval = ref<number | null>(null)

onMounted(async () => {
  // Loop checking the video duration
  video_text_interval.value = setInterval(() => {
    if (video.value) {
      const player_active_mon = get_active_mon_index_in_battle(
        battle.value,
        true,
        video.value.currentTime
      )
      if (player_state.value.active_mon_index !== player_active_mon) {
        player_state.value.active_mon_index = player_active_mon
      }
      const player_fainted_mons = get_fainted_mons_in_battle(
        battle.value,
        true,
        video.value.currentTime
      )
      if (player_state.value.fainted_mons.length !== player_fainted_mons.length) {
        player_state.value.fainted_mons = player_fainted_mons
      }

      const opponent_active_mon = get_active_mon_index_in_battle(
        battle.value,
        false,
        video.value.currentTime
      )
      if (opponent_state.value.active_mon_index !== opponent_active_mon) {
        opponent_state.value.active_mon_index = opponent_active_mon
      }

      const opponent_fainted_mons = get_fainted_mons_in_battle(
        battle.value,
        false,
        video.value.currentTime
      )
      if (opponent_state.value.fainted_mons.length !== opponent_fainted_mons.length) {
        opponent_state.value.fainted_mons = opponent_fainted_mons
      }
    }
  }, 500)

  player_history.value = await get_trainer_history(battle.value.player, battle.value.created_at)
  opponent_history.value = await get_trainer_history(battle.value.opponent, battle.value.created_at)
  loading.value = false
})

onUnmounted(() => {
  if (video_text_interval.value) {
    clearInterval(video_text_interval.value)
  }
})

watch(battle_index, async (new_index) => {
  battle.value = props.series.battles[new_index]
  player_history.value = await get_trainer_history(battle.value.player, battle.value.created_at)
  opponent_history.value = await get_trainer_history(battle.value.opponent, battle.value.created_at)
})

function completed_at() {
  const date = new Date(battle.value.created_at)

  const month = String(date.getUTCMonth() + 1).padStart(2, '0')
  const day = String(date.getUTCDate()).padStart(2, '0')
  const hours = String(date.getUTCHours()).padStart(2, '0')
  const minutes = String(date.getUTCMinutes()).padStart(2, '0')

  return `${date.getUTCFullYear()}/${month}/${day} ${hours}${minutes} UTC`
}
</script>

<template>
  <LoadingCard v-if="loading" />
  <div v-else class="battle-card" :key="battle_index">
    <div>
      <div>
        <p>HAPPENED:{{ completed_at() }}</p>
        <p>SEED:{{ battle.seed }}</p>
      </div>
      <br />
      <div class="container">
        <div v-if="player_history">
          <TrainerCard
            :trainer_id="battle.player"
            :opponent_id="battle.opponent"
            :series_id="series.id"
            :active_mon_index="player_state.active_mon_index"
            :fainted_mons="player_state.fainted_mons"
            :trainer_history="player_history"
          />
        </div>
        <div :key="refresh_video">
          <video ref="video" width="400" autoplay :controls="!!show_controls">
            <source :src="`/api/v1/battle/video/${battle.id}`" type="video/mp4" />
            Your browser does not support the video tag.
          </video>
        </div>
        <div v-if="opponent_history">
          <TrainerCard
            :trainer_id="battle.opponent"
            :opponent_id="battle.player"
            :series_id="series.id"
            :active_mon_index="opponent_state.active_mon_index"
            :fainted_mons="opponent_state.fainted_mons"
            :trainer_history="opponent_history"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  gap: 4px;
}

@media (max-width: 800px) {
  .container {
    flex-direction: column;
  }
}

.battle-card {
  text-align: center;
  margin: 10px;
}

.wins {
  display: inline-flex;
  width: 100%;
}

.trainer-card-left {
  padding-left: 1rem;
  padding-top: 1rem;
  padding-right: 0.2rem;
}

.trainer-card-right {
  padding-right: 1rem;
  padding-top: 1rem;
  padding-left: 0.2rem;
}

.trainer-card {
  width: 25%;
}

.video-card {
  width: 50%;
  text-align: center;
}

video {
  width: 460px;
  max-width: 100%;
  margin-top: 25px;
  margin-bottom: 25px;
  border: 5px solid #323431;
}

.parent {
  /* padding-left: 0.5rem;
  padding-right: 0.5rem; */
  width: 100%;
}

.inline-flex-parent {
  display: inline-flex;
}
</style>
