<script setup lang="ts">
import { RouterLink } from 'vue-router'
</script>

<template>
  <nav class="row-container">
    <div class="row-container centered">
      <RouterLink active-class="selected-nav" to="/">HOME</RouterLink>
      <RouterLink active-class="selected-nav" to="/trainer-rank">TRAINERS</RouterLink>
      <RouterLink active-class="selected-nav" to="/mon-rank">GROUPED MONS</RouterLink>
      <RouterLink active-class="selected-nav" to="/groups">GROUPS</RouterLink>
      <RouterLink active-class="selected-nav" to="/battles">BATTLES</RouterLink>
    </div>
  </nav>
</template>

<style scoped lang="scss">
nav {
  margin: 0;
  padding-top: 15px;
  padding-bottom: 15px;
  padding-left: 5px;
  width: 100%;
  background-color: #1f1f1f;
  box-shadow: 0px 4px #0d0d0d;
}

@media (max-width: 800px) {
  nav a {
    font-size: 12px;
  }
}
</style>
