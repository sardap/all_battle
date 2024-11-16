import type { SingleMonStats } from './api'
import type { Trainer } from './gen_3'

export function random_mon_stats(stats: SingleMonStats, max_size: number = 5) {
  const points = [
    { key: 'kd', size: 1 },
    { key: 'damage_dealt', size: 1 },
    { key: 'damage_taken', size: 1 },
    { key: 'times_released', size: 1 },
    { key: 'owned_by', size: 1 },
    { key: 'deaths', size: 1 },
    { key: 'murders', size: 1 },
    { key: 'move_to_show', size: 4 }
  ]

  const result: Record<string, boolean | number> = {}

  let i = 0
  let size = 0
  while (size < max_size) {
    i++
    if (i > 100) {
      break
    }

    let key = points[Math.floor(Math.random() * points.length)]
    if (size === 0 && Math.floor(Math.random() * 3) === 1) {
      key = { key: 'move_to_show', size: 4 }
    }
    if (key.key in result || key.size + size > max_size) {
      continue
    } else if (key.key == 'move_to_show') {
      if (
        stats.moves.length === 0 ||
        stats.moves.filter((move) => move.times_used > 0).length === 0
      ) {
        continue
      }

      const usedMoves = stats.moves.filter((move) => move.times_used > 0)
      const move = usedMoves[Math.floor(Math.random() * usedMoves.length)]

      result[key.key] = move.move_id
    } else {
      result[key.key] = true
    }

    size += key.size
  }

  return result
}

export function random_trainer_stats(trainer: Trainer, max_size: number = 7) {
  const points = [
    { key: 'win_rate', size: 1 },
    { key: 'total_matches', size: 1 },
    { key: 'overall_rank', size: 1 },
    { key: 'mons', size: trainer.party.length + 1 },
    { key: 'types', size: 2 },
    { key: 'feature_mon', size: 5 }
  ]

  const result: Record<string, boolean | string> = {}

  let size = 0
  let i = 0
  while (size < max_size) {
    const key = points[Math.floor(Math.random() * points.length)]
    if (
      key.key in result ||
      key.size + size > max_size ||
      (key.key === 'mons' && trainer.party.length === 1 && 'feature_mon' in result) ||
      (key.key === 'feature_mon' && trainer.party.length === 1 && 'mon' in result) ||
      (key.key === 'mon' && trainer.party.length === 1 && 'level_sum' in result)
    ) {
      i++
      if (i > 100) {
        break
      } else {
        continue
      }
    } else if (key.key === 'feature_mon') {
      result[key.key] = ['used', 'murders', 'damage_dealt', 'damage_taken', 'deaths', 'random'][
        Math.floor(Math.random() * 6)
      ]
    } else {
      result[key.key] = true
    }

    size += key.size
  }

  return result
}

export function win_percent(wins: number, total: number) {
  if (total === 0) {
    return 0
  }

  return ((wins / total) * 100).toFixed(2)
}

export function kd_ratio(kills: number, deaths: number) {
  if (deaths === 0) {
    return kills
  }

  return (kills / deaths).toFixed(2)
}

const copy_pasta = [
  'I have been programming not coding how awfully you kids these days say powerful DOS programs in assembly since 1983 and I dont know any of your hipster node.',
  'js bullshit that makes you look cool Seriously get that shit outta here.',
  'Youre just making yourself look like clowns typing some incomprehensible sugarified as fuck bullshit half of which in fact is written by your brand new and shiny Electron-based text editor you enjoy every second of and the other half of which is copied from fucking StackOverflow.',
  'Look we didnt even fucking have that and we did shit and you suckers cant even spend a single second without it.',
  'You spend hours looking for a gorgeous and flawless color scheme for it so your so-called coding is always a pleasure for you.',
  'We didnt even fucking have IntelliSense or linters you fucking spoiled little brats.',
  'Stop making programming look like a joke and go back to your parents basements where you are truly who you are.',
  'Burn HTML Burn CSS Burn JavaScript Burn PHP Burn the fucking web.',
  'I bet you wont even be able to fucking stand writing some C for an embedded system without crying to your mommy about how there are so fucking many compilers errors and everything went not as you expected.',
  'You call C fucking unsafe? Look at JavaScript Look at PHP.',
  'They let you pull out insane bullshit while you dont even notice.',
  'And then you spend hours trying to find out what you did wrong.',
  'C is actually useful.',
  'Everything you fucking see on your screen has atleast 60% of C or C++ in it.',
  'Oh what is JavaScript useful for? Rolling a stupid fucking dropdown menu or a sexy as fuck banner animation? I know what youre thinking.',
  'And by the way CSS is fucking stupid.',
  'It makes no sense half and a quarter of the time.',
  'Web development is gross and you know it.',
  'You try to compensate it by making dumb frameworks that require ages to load because they weigh over 16MiB for no fucking reason but it is no use.',
  'Oh I wonder why none of you said anything yet.',
  'Probably because I just roasted you all hard and you cant deny it.'
]

export function random_copy_pasta_sentence() {
  return copy_pasta[Math.floor(Math.random() * copy_pasta.length)]
}
