<script setup lang="ts">
import { get_completed_series } from '@/api'
import { onMounted, onUnmounted, ref } from 'vue'
import LoadingCard from './LoadingCard.vue'

const emit = defineEmits<{
  (e: 'latest', series_id: number): void
}>()

const total_matches = ref<number>(0)
const series_completed = ref<number>(0)
const series_total = ref<number>(0)
const total_battle_duration = ref<number>(0)
const estimated_complete_date = ref<Date>(new Date())
const countdown = ref('')
let countdown_interval: number | null = null

function update_countdown() {
  const now = new Date()
  const diff = estimated_complete_date.value.getTime() - now.getTime()
  const seconds = Math.floor(diff / 1000) % 60
  const minutes = Math.floor(diff / 1000 / 60) % 60
  const hours = Math.floor(diff / 1000 / 60 / 60) % 24
  const days = Math.floor(diff / 1000 / 60 / 60 / 24)
  countdown.value = `${days}D ${hours}H ${minutes}M ${seconds}S`
}

onMounted(async () => {
  countdown_interval = setInterval(() => {
    update_countdown()
  }, 1000)

  const completed = await get_completed_series()
  series_completed.value = completed.completed_count
  total_matches.value = completed.total_battles
  total_battle_duration.value = completed.total_battle_duration
  series_total.value = completed.total_ids
  const duration_ms =
    ((completed.last_timestamp - completed.start_timestamp) / completed.completed_count) *
    completed.total_ids
  estimated_complete_date.value = new Date(completed.start_timestamp + duration_ms)
  update_countdown()
  emit('latest', completed.latest_series_id)
})

onUnmounted(() => {
  if (countdown_interval) {
    clearInterval(countdown_interval)
  }
})
</script>

<template>
  <LoadingCard v-if="series_total === 0" />
  <div v-else>
    <p>
      SERIES COMPLETED:{{ ((series_completed / series_total) * 100).toFixed(2) }}% ({{
        series_completed
      }}, {{ series_total }})
    </p>
    <p>
      BATTLES:{{ total_matches }} BATTLES TOTAL DURATION:{{
        (total_battle_duration / 60 / 60 / 24 / 1000).toFixed(0)
      }}
      Days
    </p>
  </div>
</template>

<style scoped lang="scss"></style>
