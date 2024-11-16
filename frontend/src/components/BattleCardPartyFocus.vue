<script setup lang="ts">
import { type SingleMonStats } from '@/api'
import type { Mon } from '@/gen_3'
import BattleCardPartyFocusMon from './BattleCardPartyFocusMon.vue'

defineProps<{
  party: { mon: Mon; stats: SingleMonStats }[]
}>()
</script>

<template>
  <div>
    <div v-for="(mon, i) in party" :key="mon.mon.id">
      <div>
        <BattleCardPartyFocusMon :mon="mon.mon" :stats="mon.stats" />
        <hr v-if="i !== party.length - 1" />
      </div>
    </div>
  </div>
</template>

<style scoped>
hr {
  width: 200px;
  margin-left: auto;
  margin-right: auto;
  margin-top: 10px;
  margin-bottom: 10px;
  border-color: #00000000;
}
</style>
