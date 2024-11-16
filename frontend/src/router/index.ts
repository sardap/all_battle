import { createWebHistory, createRouter } from 'vue-router'

import GroupView from '@/views/GroupView.vue'
import HomeView from '@/views/HomeView.vue'
import TrainerRankView from '@/views/TrainersRankView.vue'
import MonRankView from '@/views/MonRankView.vue'
import BattleView from '@/views/BattleView.vue'

const routes = [
  { path: '/', component: HomeView },
  { path: '/trainer-rank', component: TrainerRankView },
  { path: '/groups', component: GroupView },
  { path: '/mon-rank', component: MonRankView },
  { path: '/battles', component: BattleView }
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes
})

export default router
