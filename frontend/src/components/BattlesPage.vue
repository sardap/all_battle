<script setup lang="ts">
import type { BattleSearchQueryResponse } from '@/api'
import { get_mon, get_trainer, mon_to_icon_pic_path, trainer_name_pretty } from '@/gen_3'

defineProps<{
  page: BattleSearchQueryResponse
  current_selected: { battle_id: number; series_id: number } | null
}>()

const emit = defineEmits<{
  (e: 'select-battle', selected: { battle_id: number; series_id: number }): void
}>()
</script>

<template>
  <div>
    <div>
      <hr />
      <div v-for="entry in page.data" :key="entry.battle_id">
        <p>BATTLE {{ entry.battle_id }}</p>
        <p>{{ entry.completed_at }}</p>
        <p>{{ entry.duration_seconds }}s</p>
        <div class="row-container centered">
          <div
            class="trainer-section"
            v-for="trainer in [entry.player_perspective, entry.opponent_perspective]"
            :key="trainer"
          >
            <p>{{ trainer_name_pretty(trainer) }}</p>
            <div>
              <img
                v-for="pokemon in get_trainer(trainer).party"
                :src="mon_to_icon_pic_path(get_mon(pokemon.species))"
                :key="pokemon.species"
              />
            </div>
          </div>
        </div>
        <button
          @click="emit('select-battle', { battle_id: entry.battle_id, series_id: entry.series_id })"
          :disabled="current_selected?.battle_id === entry.battle_id"
        >
          {{ current_selected?.battle_id === entry.battle_id ? 'VIEWING' : 'VIEW' }}
        </button>
        <hr />
      </div>
    </div>
  </div>
</template>

<style scoped>
hr {
  width: 430px;
  max-width: 70%;
  margin: auto;
}

button {
  margin-bottom: 5px;
}

p {
  text-align: center;
}

.trainer-section {
  margin: 10px;
}

img {
  margin-right: 3px;
}
</style>
