<script setup lang="ts">
export interface Props {
  height: string
}

withDefaults(defineProps<Props>(), {
  height: '100px'
})

function random_load_sprite() {
  const sprites = ['may-bike-sprite', 'breadon-spin-sprite']
  return sprites[Math.floor(Math.random() * sprites.length)]
}
</script>

<template>
  <div class="loading-root col-container centered" :style="`height: ${height};`">
    <div :class="random_load_sprite()"></div>
    <p>LOADING</p>
  </div>
</template>

<style scoped>
div {
  margin-left: auto;
  margin-right: auto;
}

.loading-root {
  text-align: center;
}

.may-bike-sprite {
  height: 66px;
  width: 69px;
  background-size: cover;
  background-repeat: no-repeat;
  background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAAAZCAMAAAC2NtjTAAAAAXNSR0IArs4c6QAAADBQTFRFc8Wk/97N3qSUzYNze1pSYmJzKTlBpGpSSjE5zc3eatVBQawg/2JaxUFB////AAAAIgfWXgAAABB0Uk5TAP///////////////////8BQi0MAAAEZSURBVDiNnZNZcsMgEEQlM0Cjhb7/bdOgxKmSWFwe//k9mhlAy3IrqO7/fWzAe+z7PggYGvCbSsJgg5GB7Sqgu8PQQMxiKefEHb65fmwgRjFRxPb6mREjYKViZHP9zDCs2qJWJ2Bi2LoW/tKvFzA2hPHCoIGZQXPCms86AUwTQwmunFLvFTGlmVEeOaga8b7BnIGTZ9i2HvcS0DFCUIveGb2+lUZA5XSmBppGKH0pW17YGZ7ryaERaMkO8fNEa8JfbiHIaAl0XA7dw3VEjfkvriGI9gE5z0RPqFUEH+4d/nHSyAYvht6pK1tYnfWZULkC2lxGvd1iHOWDeyZct9/l/6LZmevM3/EybaIeRDdgwt/Tfs3f037KfwDSSxTonrRUAQAAAABJRU5ErkJggg==');
  animation-name: may-bike-play;
  animation-timing-function: steps(2);
  animation-duration: 0.3s;
  animation-iteration-count: infinite;
}

@keyframes may-bike-play {
  0% {
    background-position: 0 0;
  }
  100% {
    background-position: -192px 0;
  }
}

.breadon-spin-sprite {
  height: 63px;
  width: 48px;
  background-size: cover;
  background-repeat: no-repeat;
  background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEEAAAAVCAMAAAAuNnOWAAAAAXNSR0IArs4c6QAAADBQTFRFc8Wk/9W0/8WU3pRze0FBOUp7KTliGClSECA53ubuc81zSpRS/2JaxUFB////AAAALyDOEgAAABB0Uk5TAP///////////////////8BQi0MAAAGXSURBVDiNdVOLsuwgCOsKzqBW+f+/vQHFunt6M07XQIj42OvayK/T/wTfFHnkl+lr/kWRgTFyDmbznP/kT4dvBXhxeCjP+VORI/8sEYoIYO6aLz7CIuc6luBYwkN1WWgtFShVr0Ngw/MCZS1ld5ljXuEsbtBagWaU1txCy4bzNsy/QlV09gBFax4czSty8zZry9NBwkCcc5sQmY62hsgKsgmSfhY0IcAEgdcXJRcQUuIIh8kQnoLEEPjg6cDIwkCEp8BypL8OSl7iXRIlSRhEwdURHA6JFEPj+pFLGOFgJZRQPwumZUrp5KYgfhyYkgc5HF3DsYZqZ6DvNdnzsg1sH14RBgmbAkTSvAsKTo8FsC57PQAPrez9gbh3lH1uO8kbzbQxGlq71zY7LKVxjSYrN4v0SHcoAPx0ca5sr4XVuUlE7xsW0QRamJF9DGoWHUtLcMfitmuYobFjF2qRfTKrRJ+TVHtuet6e4zjJ74Dy5HFbvzwKruvXYrOOPWPvfTtYQ/bZAvwP+NsBZyn9XGJ93vn1p4Uj9A+B+BzmB80N7AAAAABJRU5ErkJggg==');
  animation-name: breadon-spin-play;
  animation-timing-function: steps(1);
  animation-duration: 0.75s;
  animation-iteration-count: infinite;
}

@keyframes breadon-spin-play {
  0% {
    background-position: 0 0;
  }
  25% {
    background-position: -96px 0;
  }
  50% {
    background-position: -48px 0;
  }
  75% {
    background-position: -144px 0;
  }
}
</style>
