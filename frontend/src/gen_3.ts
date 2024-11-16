import { trainers } from './gen_3_trainers'
import { moves } from './gen_3_moves'
import { mons } from './gen_3_mons'

export interface TrainerMon {
  iv: number
  lvl: number
  species: string
  heldItem?: string
  moves?: string[]
}

export interface TrainerParty {
  name: string
  mons: TrainerMon[]
}

export interface Trainer {
  id: number
  idName: string
  partyFlags: number
  trainerClass: string
  encounterMusic_gender: string
  trainerPic: string
  trainerName: string
  items: string[] // Adjust type if needed
  doubleBattle: boolean
  aiFlags: string[]
  partySize: number
  party: TrainerMon[]
}

export interface Move {
  id: number
  idName: string
  name: string
  effect: string
  power: number
  type: string
  accuracy: number
  pp: number
  secondaryEffectChance: number
  target: string
  priority: number
  flags: string[]
}

export interface LevelUpMove {
  level: number
  move: string
}

export interface Mon {
  id: number
  idName: string
  name: string
  levelUpMoves: LevelUpMove[]
  monTypes: string[]
  abilities: string[]
}

export interface Gen3Output {
  trainers: Trainer[]
  moves: Move[]
  mons: Mon[]
}

export function get_type_icon_path(type: string): string {
  return `types/${type.toLowerCase()}.png`
}

export function item_to_icon_path(item: string): string {
  return `items/${item.toLowerCase()}.png`
}

export function trainer_pic_to_path(trainerPic: string): string {
  return `trainers/${trainerPic.replace('TRAINER_PIC_', '').toLowerCase()}_front_pic.png`
}

export function mon_to_front_pic_path(mon: Mon): string {
  return `pokemon/${mon.idName.toLowerCase().replace('_', '')}/front.png`
}

export function mon_to_back_pic_path(mon: Mon): string {
  return `pokemon/${mon.idName.toLowerCase().replace('_', '')}/back.png`
}

export function mon_to_anime_front_pic_path(mon: Mon): string {
  return `pokemon/${mon.idName.toLowerCase().replace('_', '')}/anim_front.png`
}

export function mon_to_icon_pic_path(mon: Mon): string {
  return `pokemon/${mon.idName.toLowerCase().replace('_', '')}/icon.png`
}

export const gen3: Gen3Output = {
  trainers: trainers,
  moves: moves,
  mons: mons
}

interface Gen3SearchTable {
  trainer_id_lookup: { [key: number]: Trainer }
  trainer_id_name_lookup: { [key: string]: Trainer }
  mon_id_lookup: { [key: number]: Mon }
  mon_id_name_lookup: { [key: string]: Mon }
  move_id_lookup: { [key: number]: Move }
  move_id_name_lookup: { [key: string]: Move }
  trainer_mons: number[]
  existing_types: string[]
}

function gen3_search_table_gen(gen3: Gen3Output): Gen3SearchTable {
  const trainer_id_lookup: { [key: number]: Trainer } = {}
  const trainer_id_name_lookup: { [key: string]: Trainer } = {}
  const mon_id_lookup: { [key: number]: Mon } = {}
  const mon_id_name_lookup: { [key: string]: Mon } = {}
  const move_id_lookup: { [key: number]: Move } = {}
  const move_id_name_lookup: { [key: string]: Move } = {}
  const trainer_mons: number[] = []

  for (let i = 0; i < gen3.mons.length; i++) {
    mon_id_lookup[gen3.mons[i].id] = gen3.mons[i]
    mon_id_name_lookup[gen3.mons[i].idName] = gen3.mons[i]
  }

  for (let i = 0; i < gen3.trainers.length; i++) {
    trainer_id_lookup[gen3.trainers[i].id] = gen3.trainers[i]
    trainer_id_name_lookup[gen3.trainers[i].idName] = gen3.trainers[i]
    for (let j = 0; j < gen3.trainers[i].party.length; j++) {
      if (!trainer_mons.includes(mon_id_name_lookup[gen3.trainers[i].party[j].species].id)) {
        trainer_mons.push(mon_id_name_lookup[gen3.trainers[i].party[j].species].id)
      }
    }
  }
  trainer_mons.sort()

  for (let i = 0; i < gen3.moves.length; i++) {
    move_id_lookup[gen3.moves[i].id] = gen3.moves[i]
    move_id_name_lookup[gen3.moves[i].idName] = gen3.moves[i]
  }

  const existing_types: string[] = []
  for (let i = 0; i < gen3.mons.length; i++) {
    for (let j = 0; j < gen3.mons[i].monTypes.length; j++) {
      if (!existing_types.includes(gen3.mons[i].monTypes[j])) {
        existing_types.push(gen3.mons[i].monTypes[j])
      }
    }
  }
  existing_types.sort()

  return {
    trainer_id_lookup,
    trainer_id_name_lookup: trainer_id_name_lookup,
    mon_id_lookup,
    mon_id_name_lookup,
    move_id_lookup,
    move_id_name_lookup,
    trainer_mons,
    existing_types
  }
}

const gen3_search_table = gen3_search_table_gen(gen3)

export function default_trainer(): Trainer {
  return gen3.trainers[0]
}

export function get_trainer(id: number): Trainer {
  const reuslt = gen3_search_table.trainer_id_lookup[id]
  if (reuslt) {
    return reuslt
  }

  return gen3.trainers[0]
}

export function get_trainer_by_idName(idName: string): Trainer {
  const result = gen3_search_table.trainer_id_name_lookup[idName]
  if (result) {
    return result
  }

  return gen3.trainers[0]
}

export function get_trainers_of_class(trainerClass: string): Trainer[] {
  const trainers: Trainer[] = []
  for (let i = 0; i < gen3.trainers.length; i++) {
    if (gen3.trainers[i].trainerClass === trainerClass) {
      trainers.push(gen3.trainers[i])
    }
  }

  return trainers
}

export function get_mon(idName: string): Mon {
  const result = gen3_search_table.mon_id_name_lookup[idName]
  if (result) {
    return result
  }

  return gen3.mons[0]
}

export function get_mon_by_id(id: number): Mon {
  const result = gen3_search_table.mon_id_lookup[id]
  if (result) {
    return result
  }

  return gen3.mons[0]
}

export function get_move(idName: string): Move {
  const result = gen3_search_table.move_id_name_lookup[idName]
  if (result) {
    return result
  }

  return gen3.moves[0]
}

export function get_move_by_id(id: number): Move {
  const result = gen3_search_table.move_id_lookup[id]
  if (result) {
    return result
  }

  return gen3.moves[0]
}

export function get_trainer_mons() {
  const result = []
  for (let i = 0; i < gen3_search_table.trainer_mons.length; i++) {
    result.push(get_mon_by_id(gen3_search_table.trainer_mons[i]))
  }
  return result
}

const trainer_mon_moves_cache = init_trainer_mon_moves()

function init_trainer_mon_moves() {
  const trainer_mon_moves: number[] = []
  for (let i = 0; i < gen3.trainers.length; i++) {
    for (let j = 0; j < gen3.trainers[i].party.length; j++) {
      const moves = get_moves(gen3.trainers[i].party[j])
      for (let k = 0; k < moves.length; k++) {
        if (!trainer_mon_moves.includes(moves[k].id)) {
          trainer_mon_moves.push(moves[k].id)
        }
      }
    }
  }
  trainer_mon_moves.sort()
  return trainer_mon_moves
}

export function get_trainer_mon_moves() {
  const result = []
  for (let i = 0; i < trainer_mon_moves_cache.length; i++) {
    result.push(get_move_by_id(trainer_mon_moves_cache[i]))
  }
  return result
}

export function function_trainers_count_with_mon(idName: string): number {
  let count = 0
  for (let i = 0; i < gen3.trainers.length; i++) {
    for (let j = 0; j < gen3.trainers[i].party.length; j++) {
      if (gen3.trainers[i].party[j].species === idName) {
        count++
        break
      }
    }
  }

  return count
}

export function get_complete_moves(moves: string[]): Move[] {
  return moves.map((move) => {
    return get_move(move)
  })
}

export function get_moves(mon: TrainerMon): Move[] {
  if (mon.moves && mon.moves.length > 0) {
    return get_complete_moves(mon.moves)
  }
  const complete_mon = get_mon(mon.species)

  let max_index = complete_mon.levelUpMoves.length - 1
  while (complete_mon.levelUpMoves[max_index].level > mon.lvl) {
    max_index--
  }

  const moves = []
  for (let i = 0; max_index - i > 0 && moves.length < 4; i++) {
    moves.push(complete_mon.levelUpMoves[max_index - i].move)
  }
  if (moves.length < 4) {
    moves.unshift(complete_mon.levelUpMoves[0].move)
  }

  return get_complete_moves(moves)
}

export function get_party_types(trainer: Trainer): string[] {
  const result: string[] = []
  for (let i = 0; i < trainer.party.length; i++) {
    const mon = get_mon(trainer.party[i].species)
    for (let j = 0; j < mon.monTypes.length; j++) {
      if (!result.includes(mon.monTypes[j])) {
        result.push(mon.monTypes[j])
      }
    }
  }
  return result
}

export function get_trainer_classes(): string[] {
  const result: string[] = []
  for (let i = 0; i < gen3.trainers.length; i++) {
    if (!result.includes(gen3.trainers[i].trainerClass)) {
      result.push(gen3.trainers[i].trainerClass)
    }
  }
  return result
}

export function random_trainer_mon(): Mon {
  const id =
    gen3_search_table.trainer_mons[
      Math.floor(Math.random() * gen3_search_table.trainer_mons.length)
    ]
  return get_mon_by_id(id)
}

export function random_trainer(): Trainer {
  return trainers[Math.floor(Math.random() * trainers.length)]
}

export function all_types(): string[] {
  return gen3_search_table.existing_types
}

export function fix_trainer_class_string(trainer_class: String) {
  const result = trainer_class.replace('TRAINER_CLASS_', '')
  return result.replace(/_/g, ' ')
}
export function trainer_name_pretty(trainer: Trainer | number): String {
  if (typeof trainer === 'number') {
    trainer = gen3_search_table.trainer_id_lookup[trainer]
  }

  return `#${trainer.id} ${trainer.trainerName} - ${fix_trainer_class_string(trainer.trainerClass)}`
}

export function get_trainers_party_avg_level(trainer: Trainer): number {
  let total = 0
  for (let i = 0; i < trainer.party.length; i++) {
    total += trainer.party[i].lvl
  }
  return total / trainer.party.length
}
