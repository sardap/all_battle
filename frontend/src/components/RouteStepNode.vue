<script setup lang="ts">
import { type RouteStep } from '@/api'
import GameMap from './GameMap.vue'
import { get_trainer, type Trainer } from '@/gen_3'
import BattleCardHolder from './BattleCardHolder.vue'
import { ref } from 'vue'
import { get_trainer_icon } from '@/gen_3_trainer_icon_map'
import RouteStepOpponent from './RouteStepOpponent.vue'

const props = defineProps<{
  trainer: Trainer
  steps: RouteStep[]
  step: RouteStep
}>()

const emit = defineEmits<{
  (e: 'change', next: RouteStep): void
}>()

const other_trainer = get_trainer(props.step.other_trainer_id)
const show_battle = ref(false)

const next_step = ref(get_next_step(props.step, 1, 'common.next'))
const prev_step = ref(get_next_step(props.step, -1, 'common.previous'))

function get_next_step(
  step: RouteStep,
  delta: number,
  default_text: string
): { step: RouteStep | null; text: string } {
  const current_index = props.steps.findIndex((s) => s === step)
  const next_index = current_index + delta
  if (next_index < 0 || next_index > props.steps.length - 1) {
    return { step: null, text: delta > 0 ? 'com.route_step.end' : 'com.route_step.start' }
  }

  if (step.battle_id === null) {
    return { step: null, text: 'com.route_step.battle_not_happened' }
  }

  if (step.won === false && delta > 0) {
    return { step: null, text: 'com.route_step.lost_current' }
  }

  const next_step = props.steps[next_index]

  if (next_step.battle_id === null) {
    return { step: null, text: 'com.route_step.battle_not_happened' }
  }

  return { step: next_step, text: default_text }
}
</script>

<template>
  <div>
    <div>
      <GameMap :icon="get_trainer_icon(trainer)" :location="step.location" />
      <div>
        <div class="row-container centered">
          <button
            @click="emit('change', prev_step.step as RouteStep)"
            :disabled="prev_step.step === null"
          >
            {{ $t(prev_step.text) }}
          </button>
          <button
            @click="emit('change', next_step.step as RouteStep)"
            :disabled="next_step.step === null"
          >
            {{ $t(next_step.text) }}
          </button>
        </div>
      </div>
      <div>
        <p>
          {{
            $t(`com.route_step.steps`, {
              current: steps.findIndex((i) => i == step) + 1,
              total: steps.length
            })
          }}
        </p>
        <RouteStepOpponent :trainer="other_trainer" />
        <br />
        <div v-if="step.series_id !== null && step.battle_id !== null">
          <div v-if="show_battle">
            <BattleCardHolder
              :series_id="step.series_id"
              :battle_id="step.battle_id"
              :hide_game_selector="true"
            />
          </div>
        </div>
        <div v-else>
          <p>{{ $t('com.route_step.battle_not_happened') }}</p>
        </div>
        <button :disabled="show_battle || step.series_id === null" @click="show_battle = true">
          {{ $t('com.route_step.show_battle') }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
button {
  margin-top: 0px;
  margin-left: 10px;
  margin-right: 10px;
}
</style>
