<script setup lang="ts">
import NavBar from '@/components/NavBar.vue'
import FooterBar from './components/FooterBar.vue'
</script>

<template>
  <div id="page-container">
    <div id="content-wrap">
      <NavBar />
      <main id="root">
        <RouterView />
      </main>
    </div>
    <FooterBar />
  </div>
</template>

<style scoped>
#root {
  max-width: 1100px;
  margin-left: auto;
  margin-right: auto;
  padding: 0.3rem;
  padding-left: 0.5rem;
  margin-bottom: 50px;
}

#page-container {
  position: relative;
  min-height: 100vh;
}

#content-wrap {
  padding-bottom: 2.5rem; /* Footer height */
}
</style>
