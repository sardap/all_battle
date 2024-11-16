const tournament_id = 1

export interface BattleResponse {
  id: number
  player: number
  opponent: number
  player_won: boolean
  seed: number
  video_path: string
  events: string[]
  created_at: number
}

export function get_active_mon_index_in_battle(
  battle: BattleResponse,
  player: boolean,
  elapsed: number
): number {
  const target_event = player ? 'PP' : 'OP'
  let result = 0
  for (const event of battle.events) {
    const splits = event.split(',')
    const event_elapsed = parseFloat(splits[0])
    if (event_elapsed > elapsed) {
      break
    }
    const event_type = splits[1]
    if (event_type === target_event) {
      const mon_index = parseInt(splits[2])
      result = mon_index
    }
  }
  return result
}

export function get_fainted_mons_in_battle(
  battle: BattleResponse,
  player: boolean,
  elapsed: number
): number[] {
  const target_event = player ? 'PF' : 'OF'
  const result: number[] = []
  for (const event of battle.events) {
    const splits = event.split(',')
    const event_elapsed = parseFloat(splits[0])
    if (event_elapsed > elapsed) {
      break
    }
    const event_type = splits[1]
    if (event_type === target_event) {
      const mon_index = parseInt(splits[2])
      result.push(mon_index)
    }
  }
  return result
}

export interface SeriesResponse {
  id: number
  first_to: number
  completed_at: number | null
  trainers: number[]
  battles: BattleResponse[]
}

export async function get_series(id: number): Promise<SeriesResponse> {
  const response = await fetch(`/api/v1/series/${id}`)
  const data: SeriesResponse = await response.json()
  return data
}

export interface CompletedSeriesResponse {
  completed_count: number
  total_ids: number
  total_battles: number
  total_battle_duration: number
  latest_series_id: number
  start_timestamp: number
  last_timestamp: number
}

export async function get_completed_series(): Promise<CompletedSeriesResponse> {
  const response = await fetch(`/api/v1/series/completed`)
  return await response.json()
}

export interface TrainerHistoryBattle {
  opponent: number
  won: boolean
}

export interface TrainerHistory {
  id: number
  wins: number
  total: number
  history: TrainerHistoryBattle[]
}

export async function get_trainer_history(
  trainer_id: number,
  cutoff: number
): Promise<TrainerHistory> {
  const response = await fetch(`/api/v1/trainer/history/${trainer_id}?exclude_after=${cutoff}`)
  return await response.json()
}

export interface TrainerRank {
  id: number
  wins: number
  total: number
  overall_rank: number
  mons: SingleMonStats[]
}

export interface TrainerRankBrief {
  id: number
  wins: number
  total: number
  overall_rank: number
}

export interface TrainersRanked {
  trainers: TrainerRankBrief[]
}

export async function get_trainers_rank(filter?: SeriesFilter): Promise<TrainersRanked> {
  let query = ''
  if (filter) {
    query = series_filter_to_query('', filter)
  }

  const response = await fetch(`/api/v1/trainer/rank?foo=bar` + query)
  return await response.json()
}

export interface MoveStats {
  move_id: number
  times_used: number
  damage_dealt: number
  murders: number
}

export interface SingleMonStats {
  mon_id: number
  battles: number
  damage_dealt: number
  damage_taken: number
  times_released: number
  murders: number
  deaths: number
  moves: MoveStats[]
}

export async function get_single_mon_stats(mon_id: number): Promise<SingleMonStats> {
  const response = await fetch(`/api/v1/mon/stats/${mon_id}`)
  return await response.json()
}

export interface TrainerStats {
  rank: TrainerRank
  series: SeriesSearchEntry[]
}

export async function get_single_trainer_stats(
  trainer_id: number,
  filter?: SeriesFilter
): Promise<TrainerStats> {
  let query = ''
  if (filter) {
    query = series_filter_to_query('', filter)
  }

  const response = await fetch(`/api/v1/trainer/stats/${trainer_id}?foo=bar` + query)
  return await response.json()
}

export interface SeriesFilter {
  trainer_class: string | null
  trainer_id: number | null
  number_of_mons: number | null
  mons: string[] | null
}

export function series_filter_match(a: SeriesFilter, b: SeriesFilter): boolean {
  return (
    a.trainer_class === b.trainer_class &&
    a.trainer_id === b.trainer_id &&
    a.number_of_mons === b.number_of_mons &&
    a.mons === b.mons
  )
}

export function default_series_filter(): SeriesFilter {
  return {
    trainer_class: null,
    trainer_id: null,
    number_of_mons: null,
    mons: null
  }
}

function series_filter_to_query(prefix: string, filter: SeriesFilter): string {
  let query = ''
  if (filter.trainer_class) {
    query += `&` + prefix + `trainer_class=${filter.trainer_class}`
  }
  if (filter.trainer_id) {
    query += `&` + prefix + `trainer_id=${filter.trainer_id}`
  }
  if (filter.number_of_mons) {
    query += `&` + prefix + `number_of_mons=${filter.number_of_mons}`
  }
  if (filter.mons) {
    let query_mons = `&` + prefix + `mons=`
    for (const mon of filter.mons) {
      query_mons += `,${mon}`
    }
    query += query_mons
  }
  return query
}

export interface SeriesSearchBattleEntry {
  id: number
  winning_trainer_id: number
  duration: number
}

export interface SeriesSearchEntry {
  series_id: number
  trainers: number[]
  battle: SeriesSearchBattleEntry[]
}

export interface SeriesSearchResponse {
  group_a_wins: number
  group_a: number[]
  group_b_wins: number
  group_b: number[]
  series: SeriesSearchEntry[]
}

export async function search_series(
  group_a: SeriesFilter,
  group_b: SeriesFilter
): Promise<SeriesSearchResponse> {
  const response = await fetch(
    `/api/v1/series/search?tournament_id=${tournament_id}` +
      series_filter_to_query('a_', group_a) +
      series_filter_to_query('b_', group_b)
  )
  if (response.status !== 200) {
    throw new Error(`search_series ${response.status}`)
  }
  return await response.json()
}

export interface MonRanking {
  mon_id: number
  murders: number
  deaths: number
  times_released: number
  damage_dealt: number
  damage_taken: number
  average_level: number
  number_exist: number
}

export interface MonRankResponse {
  mons: MonRanking[]
}

export interface MonRankFilter {
  types?: string
  abilities?: string
}

function mon_rank_query_to_query(query: MonRankFilter): string {
  let result = ''
  if (query.types) {
    result += `&types=${query.types}`
  }
  if (query.abilities) {
    result += `&abilities=${query.abilities}`
  }
  return result
}

export async function get_mon_rank(filter: MonRankFilter): Promise<MonRankResponse> {
  let query = '?foo=bar'
  query += mon_rank_query_to_query(filter)
  const response = await fetch(`/api/v1/mon/rank` + query)
  return await response.json()
}

export interface MonSeriesResponse {
  owning_trainers: number[]
  wins: number
  total: number
  series: SeriesSearchEntry[]
}

export async function get_mon_series(mon_id: number): Promise<MonSeriesResponse> {
  const response = await fetch(`/api/v1/mon/series/${mon_id}`)
  return await response.json()
}

export interface BattleStatsResponse {
  battle_id: number
  player_mons: SingleMonStats[]
  opponent_mons: SingleMonStats[]
}

export async function get_battle_stats(battle_id: number): Promise<BattleStatsResponse> {
  const response = await fetch(`/api/v1/battle/stats/${battle_id}`)
  return await response.json()
}

export interface PagedQuery {
  limit?: number
  offset?: number
}

export interface PagedResponse<T> {
  data: T
  limit: number
  offset: number
  total: number
}

export interface BattleSearchEntry {
  battle_id: number
  series_id: number
  duration_seconds: number
  completed_at: string
  seed: number
  player_perspective: number
  opponent_perspective: number
  player_perspective_won: boolean
}

export interface BattleSearchQueryResponse extends PagedResponse<BattleSearchEntry[]> {}

export interface BattleSearchQuery extends PagedQuery {
  min_length_seconds?: number
  max_length_seconds?: number
  seed?: number
  player_perspective_won?: boolean
  min_kos?: number
  max_kos?: number
  min_damage_dealt?: number
  max_damage_dealt?: number
  mons_included?: string
  mons_excluded?: string
  moves_used?: string
  moves_not_used?: string

  order_by_field?: string
  order_by_descending?: boolean
}

export async function get_battle_search(
  query: BattleSearchQuery
): Promise<BattleSearchQueryResponse> {
  const response = await fetch(`/api/v1/battle/search?` + new URLSearchParams(query as any))
  return await response.json()
}

export interface RouteStep {
  location: string
  other_trainer_id: number
  battle_id: number | null
  series_id: number | null
  won: boolean
}

export enum RouteResult {
  Completed,
  Lost,
  IncompleteData
}

export interface RouteProgressSet {
  progress: RouteStep[]
  result: RouteResult
}

export interface TrainerProgressResponse {
  id: number
  progress: RouteProgressSet
}

export async function get_trainer_progress(trainer_id: number): Promise<TrainerProgressResponse> {
  const response = await fetch(`/api/v1/trainer/progress/${trainer_id}`)
  return await response.json()
}
