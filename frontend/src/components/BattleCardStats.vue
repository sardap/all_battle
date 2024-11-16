<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { get_battle_stats, type SingleMonStats } from '@/api'
import LoadingCard from './LoadingCard.vue'
import { get_mon_by_id, type Mon } from '@/gen_3'
import BattleCardPartyFocus from './BattleCardPartyFocus.vue'

const props = defineProps<{
  battle_id: number
}>()

const loading = ref(true)
const player_mons = ref<{ stats: SingleMonStats; mon: Mon }[]>([])
const opponent_mons = ref<{ stats: SingleMonStats; mon: Mon }[]>([])

onMounted(async () => {
  loading.value = true
  const stats = await get_battle_stats(props.battle_id)

  stats.player_mons = stats.player_mons.filter((mon) => mon.times_released > 0)
  stats.player_mons.sort((a, b) => b.murders - a.murders)
  player_mons.value = stats.player_mons.map((mon) => ({
    stats: mon,
    mon: get_mon_by_id(mon.mon_id)
  }))

  stats.opponent_mons = stats.opponent_mons.filter((mon) => mon.times_released > 0)
  stats.opponent_mons.sort((a, b) => b.murders - a.murders)
  opponent_mons.value = stats.opponent_mons.map((mon) => ({
    stats: mon,
    mon: get_mon_by_id(mon.mon_id)
  }))

  loading.value = false
})
</script>

<template>
  <LoadingCard v-if="loading" />
  <div v-else-if="!loading" class="battle-card-stats">
    <div class="row-container">
      <div style="width: 50%">
        <BattleCardPartyFocus :party="player_mons" />
      </div>
      <div style="width: 50%">
        <BattleCardPartyFocus :party="opponent_mons" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.battle-card-stats {
  padding: 10px;
}
</style>
