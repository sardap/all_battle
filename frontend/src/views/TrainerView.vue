<script setup lang="ts">
import MonFocus from '@/components/MonFocus.vue'
import { ref } from 'vue'

const selected_mon_id = ref<number>(1)
</script>

<template>
  <div class="col-container centered">
    <h1>MON FOCUS</h1>
    <MonFocus :mon_id="selected_mon_id" />
  </div>
</template>

<style scoped>
p {
  text-align: center;
}

h3 {
  text-align: center;
}

.mon-rank {
  height: 400px;
  width: 400px;
  overflow: scroll;
}
</style>
