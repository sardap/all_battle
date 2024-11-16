<script setup lang="ts">
import { get_type_icon_path } from '@/gen_3'

defineProps<{
  types: string[]
}>()
</script>

<template>
  <img v-for="type in types" :key="type" :src="get_type_icon_path(type)" :alt="type" />
</template>

<style scoped>
img {
  margin-right: 3px;
}
</style>
