<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { type SingleMonStats, type TrainerRank } from '@/api'
import {
  trainer_pic_to_path,
  type Mon,
  mon_to_icon_pic_path,
  type Trainer,
  get_mon,
  get_party_types,
  get_type_icon_path
} from '@/gen_3'
import MonStatsCard from './MonStatsCard.vue'
import { random_mon_stats } from '@/utils'

const props = defineProps<{
  trainer: {
    trainer: Trainer
    stats: TrainerRank
  }
  to_show: RandomStats
}>()

export interface RandomStats {
  win_rate?: boolean
  total_matches?: boolean
  overall_rank?: boolean
  mons?: boolean
  types?: boolean
  feature_mon?: 'used' | 'murders' | 'damage_dealt' | 'damage_taken' | 'deaths' | 'random'
}

const featured_mon = ref<MonTuple | null>(null)

onMounted(async () => {
  console.log(`${props.trainer.trainer.trainerName} stats:`, props.to_show)

  const ordered_mons = order_mons(props.trainer.trainer, props.trainer.stats)

  console.log(`ordered mons used`)

  let chosen_mon = null
  if (props.to_show.feature_mon) {
    switch (props.to_show.feature_mon) {
      case 'used':
        chosen_mon = ordered_mons.used[0]
        break
      case 'murders':
        chosen_mon = ordered_mons.murders[0]
        break
      case 'damage_dealt':
        chosen_mon = ordered_mons.damage_dealt[0]
        break
      case 'damage_taken':
        chosen_mon = ordered_mons.damage_taken[0]
        break
      case 'deaths':
        chosen_mon = ordered_mons.deaths[0]
        break
      case 'random':
        chosen_mon = ordered_mons.used[Math.floor(Math.random() * ordered_mons.used.length)]
        break
    }
  }

  featured_mon.value = chosen_mon
})

interface MonTuple {
  mon: Mon
  stats: SingleMonStats
}

interface OrderedMons {
  used: MonTuple[]
  murders: MonTuple[]
  damage_dealt: MonTuple[]
  damage_taken: MonTuple[]
  deaths: MonTuple[]
}

function order_mons(trainer: Trainer, stats: TrainerRank): OrderedMons {
  const tuples = stats.mons.map((mon, index) => {
    return {
      mon: get_mon(trainer.party[index].species),
      stats: mon
    }
  })

  const used_order = tuples.slice().sort((a, b) => b.stats.times_released - a.stats.times_released)
  const kill_order = tuples.slice().sort((a, b) => b.stats.murders - a.stats.murders)
  const damage_dealt_order = tuples
    .slice()
    .sort((a, b) => b.stats.damage_dealt - a.stats.damage_dealt)
  const damage_taken_order = tuples
    .slice()
    .sort((a, b) => b.stats.damage_taken - a.stats.damage_taken)
  const death_order = tuples.slice().sort((a, b) => b.stats.deaths - a.stats.deaths)

  return {
    used: used_order,
    murders: kill_order,
    damage_dealt: damage_dealt_order,
    damage_taken: damage_taken_order,
    deaths: death_order
  }
}

function featured_mon_title(): string {
  if (!featured_mon.value) {
    return ''
  }

  switch (props.to_show.feature_mon) {
    case 'used':
      return 'FAVORITE MON'
    case 'murders':
      return 'DEATH DEALER MON'
    case 'damage_dealt':
      return 'DAMAGER'
    case 'damage_taken':
      return 'MON TAKES IT THE MOST'
    case 'deaths':
      return 'MOST DEAD MON'
    case 'random':
      return 'MON'
  }

  return 'MON'
}

function safe_win_rate(): string {
  if (props.trainer.stats.total === 0 || props.trainer.stats.wins === 0) {
    return '0.00'
  }

  return ((props.trainer.stats.wins / props.trainer.stats.total) * 100).toFixed(2)
}
</script>

<template>
  <div>
    <div v-if="trainer">
      <p>
        #{{ trainer.trainer.id }} {{ trainer.trainer.trainerName }}
        {{ trainer.trainer.trainerClass.replace('TRAINER_CLASS_', '').replace('_', ' ') }}
      </p>
      <img :src="trainer_pic_to_path(trainer.trainer.trainerPic)" />
      <p v-if="to_show.overall_rank">RANK:{{ trainer.stats.overall_rank }}</p>
      <p v-if="to_show.win_rate">WIN RATE: {{ safe_win_rate() }}%</p>
      <p v-if="to_show.total_matches">TOTAL MATCHES: {{ trainer.stats.total }}</p>
      <p v-if="to_show.types">
        <img
          v-for="type in get_party_types(trainer.trainer)"
          :key="type"
          :src="get_type_icon_path(type)"
        />
      </p>
      <div v-if="to_show.mons">
        <p>PARTY</p>
        <p v-for="mon in trainer.trainer.party" :key="mon.species" class="mons">
          <img :src="mon_to_icon_pic_path(get_mon(mon.species))" width="28" height="28" />
          {{ get_mon(mon.species).name }} Lv:{{ mon.lvl }}
        </p>
      </div>
      <div class="featured-mon" v-if="featured_mon && to_show.feature_mon">
        <p>{{ featured_mon_title() }}</p>
        <MonStatsCard
          :key="featured_mon.mon.id"
          :mon="featured_mon.mon"
          :stats="featured_mon.stats"
          :to_show="random_mon_stats(featured_mon.stats, 4)"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.mons {
  text-align: left;
  margin-left: 5px;
}

.featured-mon {
  margin-top: 5px;
}
</style>
