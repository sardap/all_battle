<script setup lang="ts">
import { get_trainer_progress, type RouteStep, type TrainerProgressResponse } from '@/api'
import LoadingCard from '@/components/LoadingCard.vue'
import { get_trainer, type Trainer } from '@/gen_3'
import { onMounted, ref } from 'vue'
import RouteStepNode from './RouteStepNode.vue'
import TrainerGameProgressOverview from './TrainerGameProgressOverview.vue'

const props = defineProps<{
  trainer_id: number
}>()

const loading = ref(false)
const trainer = ref<Trainer>(get_trainer(props.trainer_id))
const response = ref<TrainerProgressResponse | null>(null)
const selected_step = ref<RouteStep>({
  location: '',
  other_trainer_id: 0,
  battle_id: 0,
  series_id: 0,
  won: false
})

onMounted(async () => {
  loading.value = true
  response.value = await get_trainer_progress(props.trainer_id)
  selected_step.value = response.value.progress.progress[0]
  loading.value = false
})
</script>

<template>
  <div>
    <LoadingCard height="500px" v-if="loading" />
    <div v-else-if="response">
      <TrainerGameProgressOverview :trainer="trainer" :steps="response.progress.progress" />
      <RouteStepNode
        :key="selected_step.other_trainer_id"
        :trainer="trainer"
        :steps="response.progress.progress"
        :step="selected_step"
        @change="
          (change) => {
            selected_step = change
          }
        "
      />
    </div>
  </div>
</template>

<style scoped></style>
