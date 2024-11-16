<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { i18n } from '@/main'

const props = defineProps<{
  icon: string
  location: string
}>()

const icon_pos_table: Record<string, { x: number; y: number }> = {
  aqua_hideout: { x: 237, y: 43 },
  cable_car: { x: 86, y: 43 },
  cave_of_origin: { x: 248, y: 100 },
  dewford_town: { x: 30, y: 180 },
  dewford_town_gym: { x: 30, y: 180 },
  elite_four: { x: 315, y: 110 },
  ever_grande_city: { x: 315, y: 110 },
  fallarbor_town: { x: 43, y: 21 },
  fiery_path: { x: 86, y: 43 },
  fortree_city: { x: 145, y: 20 },
  fortree_city_gym: { x: 145, y: 20 },
  granite_cave: { x: 19, y: 182 },
  jagged_pass: { x: 86, y: 43 },
  lavaridge_town: { x: 65, y: 56 },
  lavaridge_town_gym: { x: 65, y: 56 },
  lilycove_city: { x: 225, y: 55 },
  mauville_city: { x: 100, y: 90 },
  mauville_city_gym: { x: 100, y: 90 },
  meteor_falls: { x: 10, y: 46 },
  mossdeep_city: { x: 279, y: 77 },
  mossdeep_city_gym: { x: 279, y: 77 },
  mossdeep_city_space_center: { x: 293, y: 77 },
  mt_chimney: { x: 75, y: 35 },
  mt_pyre: { x: 190, y: 73 },
  oldale_town: { x: 55, y: 124 },
  petalburg_city: { x: 20, y: 124 },
  petalburg_city_gym: { x: 20, y: 124 },
  petalburg_woods: { x: 9, y: 110 },
  route_102: { x: 40, y: 124 },
  route_103: { x: 55, y: 112 },
  route_104: { x: 9, y: 124 },
  route_110: { x: 100, y: 110 },
  route_111: { x: 100, y: 37 },
  route_112: { x: 85, y: 56 },
  route_113: { x: 70, y: 21 },
  route_114: { x: 21, y: 35 },
  route_115: { x: 10, y: 60 },
  route_116: { x: 30, y: 78 },
  route_117: { x: 75, y: 90 },
  route_118: { x: 125, y: 90 },
  route_119: { x: 135, y: 56 },
  route_124: { x: 255, y: 55 },
  route_128: { x: 283, y: 122 },
  russboro_city: { x: 10, y: 88 },
  rustboro_city: { x: 10, y: 88 },
  rustboro_city_gym: { x: 10, y: 88 },
  rusturf_tunnel: { x: 52, y: 76 },
  seafloor_cavern: { x: 283, y: 122 },
  sky_pillar: { x: 235, y: 123 },
  slateport_city: { x: 100, y: 135 },
  slateport_city_museum: { x: 100, y: 135 },
  sootopolis_city: { x: 248, y: 100 },
  stootopolis_city: { x: 248, y: 100 },
  stootopolis_city_gym: { x: 248, y: 100 },
  verdanturf_town: { x: 55, y: 91 },
  victory_road: { x: 316, y: 118 }
}

const map_ref = ref<HTMLCanvasElement | null>(null)
const text_ref = ref<HTMLCanvasElement | null>(null)
const icon_ref = ref<HTMLCanvasElement | null>(null)

onMounted(async () => {
  if (map_ref.value === null || text_ref.value === null || icon_ref.value === null) {
    return
  }

  {
    const map_ctx = map_ref.value.getContext('2d')
    if (map_ctx === null) {
      return
    }

    const canvas = map_ref.value

    const map = new Image()
    map.onload = () => {
      map_ctx.drawImage(map, 0, 0, canvas.width, canvas.height)
    }
    map.src = '/map.png'
  }

  {
    const text_ctx = text_ref.value.getContext('2d')
    if (text_ctx === null) {
      return
    }

    let size = 1.3
    let y = 200
    let location_text = i18n.global.t(`common.location.${props.location}`)
    if (location_text.length > 15) {
      size = 0.7
      y = 195
    } else if (location_text.length > 12) {
      size = 0.9
      y = 198
    } else if (location_text.length > 10) {
      size = 1.0
      y = 195
    }

    text_ctx.font = `${size}em pokemon`
    text_ctx.fillText(location_text, 200, y)
  }

  {
    const icon_ctx = icon_ref.value.getContext('2d')
    if (icon_ctx === null) {
      return
    }

    const icon = new Image()
    icon.onload = () => {
      const icon_pos = icon_pos_table[props.location]

      const ratio = icon.width / icon.height

      const size = 20
      const width = size * ratio
      const height = size
      const x = icon_pos.x + 7 - width / 2
      const y = icon_pos.y + 7 - height / 2

      icon_ctx.drawImage(icon, x, y, width, height)
    }
    icon.src = props.icon
  }
})
</script>

<template>
  <div class="map-holder">
    <div class="canvas-container">
      <canvas style="z-index: 0" ref="map_ref" id="map" width="340" height="226"></canvas>
      <canvas style="z-index: 1" ref="text_ref" id="map" width="340" height="226"></canvas>
      <canvas style="z-index: 2" ref="icon_ref" id="map" width="340" height="226"></canvas>
    </div>
  </div>
</template>

<style scoped>
.map-holder {
  display: flex;
  justify-content: center;
  align-items: center;
  margin-top: 20px;
  margin-bottom: 20px;
  height: 300px;
}

.canvas-container {
  position: relative;
  width: 400px;
  max-width: 90%;
  height: 264px;
  border: 3px solid red;
}

canvas {
  height: 100%;
  width: 100%;
  position: absolute;
  left: 0;
  top: 0;
}
</style>
