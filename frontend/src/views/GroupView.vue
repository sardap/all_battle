<script setup lang="ts">
import SeriesViewSelector from '@/components/SeriesViewSelector.vue'
import BattleCardHolder from '@/components/BattleCardHolder.vue'
import { ref } from 'vue'

const selected_series = ref<number | undefined>(undefined)
</script>

<template>
  <h1>GROUPS</h1>
  <div>
    <div>
      <SeriesViewSelector
        @updated_series="
          (updated) => {
            selected_series = updated
          }
        "
      />
      <br />
    </div>
    <div>
      <BattleCardHolder :key="selected_series" :series_id="selected_series" />
    </div>
  </div>
</template>

<style scoped></style>
