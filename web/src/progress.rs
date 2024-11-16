use all_battle_core::{
    gen3::GEN3,
    schema::{self},
};
use diesel::prelude::*;
use lazy_static::lazy_static;
use serde_derive::Serialize;
use std::collections::{HashMap, HashSet};

use crate::PooledConnection;

#[derive(Debug, Copy, Clone)]
struct TrainerBattle {
    trainer_id: i32,
}

#[derive(Debug, Clone)]
struct OrTrainerBattle {
    battles: Vec<TrainerBattle>,
}

#[derive(Debug, Copy, Clone)]
struct Condition {
    pub key: &'static str,
    pub value: &'static str,
}

#[derive(Debug, Clone)]
enum Node {
    ChangeLocation(&'static str),
    Single(TrainerBattle),
    Or(Vec<OrTrainerBattle>),
    Cond(Condition, TrainerBattle),
    CondOr(Vec<(Vec<Condition>, OrTrainerBattle)>),
}

static STARTER_MUDKIP: Condition = Condition {
    key: "starter",
    value: "mudkip",
};

static STARTER_TREECKO: Condition = Condition {
    key: "starter",
    value: "treecko",
};

static STARTER_TORCHIC: Condition = Condition {
    key: "starter",
    value: "torchic",
};

static STARTER_CONDITIONS: [Condition; 3] = [STARTER_MUDKIP, STARTER_TREECKO, STARTER_TORCHIC];

static RIVAL_BRENDAN: Condition = Condition {
    key: "rival",
    value: "brendan",
};

static RIVAL_MAY: Condition = Condition {
    key: "rival",
    value: "may",
};

static RIVAL_CONDITIONS: [Condition; 2] = [RIVAL_BRENDAN, RIVAL_MAY];

static PARTY_SIZE_GREATER_THAN_ONE: Condition = Condition {
    key: "party_size_greater_than_one",
    value: "yes",
};

static PARTY_SIZE_ONE: Condition = Condition {
    key: "party_size_greater_than_one",
    value: "no",
};

static PBCG_LLR: Condition = Condition {
    key: "pbcg",
    value: "llr",
};

static PBCG_LRL: Condition = Condition {
    key: "pbcg",
    value: "lrl",
};

static PBCG_LRR: Condition = Condition {
    key: "pbcg",
    value: "lrr",
};

static PBCG_RLL: Condition = Condition {
    key: "pbcg",
    value: "rll",
};

static PBCG_RLR: Condition = Condition {
    key: "pbcg",
    value: "rlr",
};

static PBCG_RRL: Condition = Condition {
    key: "pbcg",
    value: "rrl",
};

static PBCG_CONDITIONS: [Condition; 6] =
    [PBCG_LLR, PBCG_LRL, PBCG_LRR, PBCG_RLL, PBCG_RLR, PBCG_RRL];

static BRENDAN_ROUTE_103_MUDKIP: TrainerBattle = TrainerBattle { trainer_id: 520 };

static BRENDAN_ROUTE_103_TREECKO: TrainerBattle = TrainerBattle { trainer_id: 523 };

static BRENDAN_ROUTE_103_TORCHIC: TrainerBattle = TrainerBattle { trainer_id: 526 };

static MAY_ROUTE_103_MUDKIP: TrainerBattle = TrainerBattle { trainer_id: 529 };

static MAY_ROUTE_103_TREECKO: TrainerBattle = TrainerBattle { trainer_id: 532 };

static MAY_ROUTE_103_TORCHIC: TrainerBattle = TrainerBattle { trainer_id: 535 };

static CALVIN_1: TrainerBattle = TrainerBattle { trainer_id: 318 };

static GRUNT_PETEALBURG_WOODS: TrainerBattle = TrainerBattle { trainer_id: 10 };

static GINA_AND_MIA_1: TrainerBattle = TrainerBattle { trainer_id: 483 };

static ROXANNE_1: TrainerBattle = TrainerBattle { trainer_id: 265 };

static DEVAN: TrainerBattle = TrainerBattle { trainer_id: 753 };

static GRUNT_RUSTURF_TUNNEL: TrainerBattle = TrainerBattle { trainer_id: 16 };

static GRUNT_MUSEUM_1: TrainerBattle = TrainerBattle { trainer_id: 20 };

static GRUNT_MUSEUM_2: TrainerBattle = TrainerBattle { trainer_id: 21 };

static ISABEL_1: TrainerBattle = TrainerBattle { trainer_id: 302 };

static KALEB: TrainerBattle = TrainerBattle { trainer_id: 699 };

static EDWARD: TrainerBattle = TrainerBattle { trainer_id: 232 };

static ALYSSA: TrainerBattle = TrainerBattle { trainer_id: 701 };

static BRENDAN_ROUTE_110_MUDKIP: TrainerBattle = TrainerBattle { trainer_id: 521 };

static BRENDAN_ROUTE_110_TREECKO: TrainerBattle = TrainerBattle { trainer_id: 524 };

static BRENDAN_ROUTE_110_TORCHIC: TrainerBattle = TrainerBattle { trainer_id: 527 };

static MAY_ROUTE_110_MUDKIP: TrainerBattle = TrainerBattle { trainer_id: 530 };

static MAY_ROUTE_110_TREECKO: TrainerBattle = TrainerBattle { trainer_id: 533 };

static MAY_ROUTE_110_TORCHIC: TrainerBattle = TrainerBattle { trainer_id: 536 };

static WALLY_MAUVILLE: TrainerBattle = TrainerBattle { trainer_id: 656 };

static SHAWN: TrainerBattle = TrainerBattle { trainer_id: 194 };

static ANGELO: TrainerBattle = TrainerBattle { trainer_id: 802 };

static WATTSON_1: TrainerBattle = TrainerBattle { trainer_id: 267 };

static BROOKE_1: TrainerBattle = TrainerBattle { trainer_id: 94 };

static LUCAS_1: TrainerBattle = TrainerBattle { trainer_id: 629 };

static MARLENE: TrainerBattle = TrainerBattle { trainer_id: 752 };

static MIKE_2: TrainerBattle = TrainerBattle { trainer_id: 635 };

static GRUNT_MT_CHIMNEY_1: TrainerBattle = TrainerBattle { trainer_id: 146 };

static GRUNT_MT_CHIMNEY_2: TrainerBattle = TrainerBattle { trainer_id: 579 };

static TABITHA_MT_CHIMNEY: TrainerBattle = TrainerBattle { trainer_id: 597 };

static MAXIE_MT_CHIMNEY: TrainerBattle = TrainerBattle { trainer_id: 602 };

static FLANNERY_1: TrainerBattle = TrainerBattle { trainer_id: 268 };

static BRAWLY_1: TrainerBattle = TrainerBattle { trainer_id: 266 };

static RANDALL: TrainerBattle = TrainerBattle { trainer_id: 71 };

static PARKER: TrainerBattle = TrainerBattle { trainer_id: 72 };

static JODY: TrainerBattle = TrainerBattle { trainer_id: 91 };

static ALEXIA: TrainerBattle = TrainerBattle { trainer_id: 90 };

static BERKE: TrainerBattle = TrainerBattle { trainer_id: 74 };

static MARY: TrainerBattle = TrainerBattle { trainer_id: 89 };

static GEORGE: TrainerBattle = TrainerBattle { trainer_id: 73 };

static NORMAN_1: TrainerBattle = TrainerBattle { trainer_id: 269 };

static AMY_AND_LIV_1: TrainerBattle = TrainerBattle { trainer_id: 481 };

static DEANDRE: TrainerBattle = TrainerBattle { trainer_id: 715 };

static ROSE_1: TrainerBattle = TrainerBattle { trainer_id: 37 };

static GRUNT_WEATHER_INST_1: TrainerBattle = TrainerBattle { trainer_id: 17 };

static GRUNT_WEATHER_INST_4: TrainerBattle = TrainerBattle { trainer_id: 26 };

static GRUNT_WEATHER_INST_2: TrainerBattle = TrainerBattle { trainer_id: 18 };

static GRUNT_WEATHER_INST_5: TrainerBattle = TrainerBattle { trainer_id: 596 };

static SHELLY_WEATHER_INSTITUTE: TrainerBattle = TrainerBattle { trainer_id: 32 };

static BRENDAN_ROUTE_119_MUDKIP: TrainerBattle = TrainerBattle { trainer_id: 522 };

static BRENDAN_ROUTE_119_TREECKO: TrainerBattle = TrainerBattle { trainer_id: 525 };

static BRENDAN_ROUTE_119_TORCHIC: TrainerBattle = TrainerBattle { trainer_id: 528 };

static MAY_ROUTE_119_MUDKIP: TrainerBattle = TrainerBattle { trainer_id: 531 };

static MAY_ROUTE_119_TREECKO: TrainerBattle = TrainerBattle { trainer_id: 534 };

static MAY_ROUTE_119_TORCHIC: TrainerBattle = TrainerBattle { trainer_id: 537 };

static FLINT: TrainerBattle = TrainerBattle { trainer_id: 654 };

static EDWARDO: TrainerBattle = TrainerBattle { trainer_id: 404 };

static DARIUS: TrainerBattle = TrainerBattle { trainer_id: 803 };

static WINONA_1: TrainerBattle = TrainerBattle { trainer_id: 270 };

static GRUNT_MT_PYRE_2: TrainerBattle = TrainerBattle { trainer_id: 24 };

static GRUNT_MT_PYRE_1: TrainerBattle = TrainerBattle { trainer_id: 23 };

static GRUNT_MT_PYRE_3: TrainerBattle = TrainerBattle { trainer_id: 25 };

static GRUNT_MT_PYRE_4: TrainerBattle = TrainerBattle { trainer_id: 596 };

static GRUNT_MAGMA_HIDEOUT_2: TrainerBattle = TrainerBattle { trainer_id: 717 };

static GRUNT_MAGMA_HIDEOUT_3: TrainerBattle = TrainerBattle { trainer_id: 718 };

static GRUNT_MAGMA_HIDEOUT_7: TrainerBattle = TrainerBattle { trainer_id: 722 };

static GRUNT_MAGMA_HIDEOUT_11: TrainerBattle = TrainerBattle { trainer_id: 726 };

static GRUNT_MAGMA_HIDEOUT_16: TrainerBattle = TrainerBattle { trainer_id: 731 };

static GRUNT_MAGMA_HIDEOUT_15: TrainerBattle = TrainerBattle { trainer_id: 730 };

static GRUNT_MAGMA_HIDEOUT_13: TrainerBattle = TrainerBattle { trainer_id: 728 };

static TABITHA_MAGMA_HIDEOUT: TrainerBattle = TrainerBattle { trainer_id: 732 };

static MAXIE_MAGMA_HIDEOUT: TrainerBattle = TrainerBattle { trainer_id: 601 };

static GRUNT_AQUA_HIDEOUT_2: TrainerBattle = TrainerBattle { trainer_id: 3 };

static GRUNT_AQUA_HIDEOUT_7: TrainerBattle = TrainerBattle { trainer_id: 192 };

static GRUNT_AQUA_HIDEOUT_6: TrainerBattle = TrainerBattle { trainer_id: 28 };

static GRUNT_AQUA_HIDEOUT_8: TrainerBattle = TrainerBattle { trainer_id: 193 };

static MATT: TrainerBattle = TrainerBattle { trainer_id: 30 };

static DECLAN: TrainerBattle = TrainerBattle { trainer_id: 15 };

static PRESTON: TrainerBattle = TrainerBattle { trainer_id: 233 };

static MAURA: TrainerBattle = TrainerBattle { trainer_id: 246 };

static BLAKE: TrainerBattle = TrainerBattle { trainer_id: 235 };

static SAMANTHA: TrainerBattle = TrainerBattle { trainer_id: 245 };

static KATHLEEN: TrainerBattle = TrainerBattle { trainer_id: 583 };

static NICHOLAS: TrainerBattle = TrainerBattle { trainer_id: 585 };

static MACEY: TrainerBattle = TrainerBattle { trainer_id: 591 };

static HANNAH: TrainerBattle = TrainerBattle { trainer_id: 244 };

static TATE_AND_LIZA_1: TrainerBattle = TrainerBattle { trainer_id: 271 };

static GRUNT_SPACE_CENTER_4: TrainerBattle = TrainerBattle { trainer_id: 587 };

static GRUNT_SPACE_CENTER_2: TrainerBattle = TrainerBattle { trainer_id: 116 };

static GRUNT_SPACE_CENTER_5: TrainerBattle = TrainerBattle { trainer_id: 588 };

static GRUNT_SPACE_CENTER_6: TrainerBattle = TrainerBattle { trainer_id: 589 };

static GRUNT_SPACE_CENTER_7: TrainerBattle = TrainerBattle { trainer_id: 590 };

static MAXIE_MOSSDEEP: TrainerBattle = TrainerBattle { trainer_id: 734 };

static TABITHA_MOSSDEEP: TrainerBattle = TrainerBattle { trainer_id: 514 };

static ARCHIE: TrainerBattle = TrainerBattle { trainer_id: 34 };

static JUAN_1: TrainerBattle = TrainerBattle { trainer_id: 272 };

static WALLY_VR_1: TrainerBattle = TrainerBattle { trainer_id: 519 };

static HOPE: TrainerBattle = TrainerBattle { trainer_id: 96 };

static SHANNON: TrainerBattle = TrainerBattle { trainer_id: 97 };

static JULIE: TrainerBattle = TrainerBattle { trainer_id: 100 };

static EDGAR: TrainerBattle = TrainerBattle { trainer_id: 79 };

static SIDNEY: TrainerBattle = TrainerBattle { trainer_id: 261 };

static PHOEBE: TrainerBattle = TrainerBattle { trainer_id: 262 };

static GLACIA: TrainerBattle = TrainerBattle { trainer_id: 263 };

static DRAKE: TrainerBattle = TrainerBattle { trainer_id: 264 };

static WALLACE: TrainerBattle = TrainerBattle { trainer_id: 335 };

lazy_static! {
    static ref FULL_ROUTE: Vec<Node> = vec![
        Node::ChangeLocation("route_101"),
        Node::ChangeLocation("route_103"),
        Node::CondOr(vec![
            (
                vec![RIVAL_BRENDAN, STARTER_MUDKIP],
                OrTrainerBattle {
                    battles: vec![BRENDAN_ROUTE_103_MUDKIP]
                }
            ),
            (
                vec![RIVAL_BRENDAN, STARTER_TREECKO],
                OrTrainerBattle {
                    battles: vec![BRENDAN_ROUTE_103_TREECKO]
                }
            ),
            (
                vec![RIVAL_BRENDAN, STARTER_TORCHIC],
                OrTrainerBattle {
                    battles: vec![BRENDAN_ROUTE_103_TORCHIC]
                }
            ),
            (
                vec![RIVAL_MAY, STARTER_MUDKIP],
                OrTrainerBattle {
                    battles: vec![MAY_ROUTE_103_MUDKIP]
                }
            ),
            (
                vec![RIVAL_MAY, STARTER_TREECKO],
                OrTrainerBattle {
                    battles: vec![MAY_ROUTE_103_TREECKO]
                }
            ),
            (
                vec![RIVAL_MAY, STARTER_TORCHIC],
                OrTrainerBattle {
                    battles: vec![MAY_ROUTE_103_TORCHIC]
                }
            )
        ]),
        Node::ChangeLocation("route_101"),
        Node::ChangeLocation("route_102"),
        Node::Single(CALVIN_1),
        Node::ChangeLocation("route_104"),
        Node::ChangeLocation("petalburg_woods"),
        Node::Single(GRUNT_PETEALBURG_WOODS),
        Node::ChangeLocation("route_104"),
        Node::Cond(PARTY_SIZE_GREATER_THAN_ONE, GINA_AND_MIA_1),
        Node::ChangeLocation("rustboro_city"),
        Node::ChangeLocation("rustboro_city_gym"),
        Node::Single(ROXANNE_1),
        Node::ChangeLocation("rustboro_city"),
        Node::ChangeLocation("route_116"),
        Node::Single(DEVAN),
        Node::ChangeLocation("rusturf_tunnel"),
        Node::Single(GRUNT_RUSTURF_TUNNEL),
        Node::ChangeLocation("route_116"),
        Node::ChangeLocation("rustboro_city"),
        Node::ChangeLocation("petalburg_woods"),
        Node::ChangeLocation("route_104"),
        Node::ChangeLocation("dewford_town"),
        Node::ChangeLocation("granite_cave"),
        Node::ChangeLocation("dewford_town"),
        Node::ChangeLocation("slateport_city"),
        Node::ChangeLocation("slateport_city_museum"),
        Node::Single(GRUNT_MUSEUM_1),
        Node::Single(GRUNT_MUSEUM_2),
        Node::ChangeLocation("route_110"),
        Node::Single(ISABEL_1),
        Node::Single(KALEB),
        Node::CondOr(vec![
            (
                vec![RIVAL_BRENDAN, STARTER_MUDKIP],
                OrTrainerBattle {
                    battles: vec![BRENDAN_ROUTE_110_MUDKIP]
                }
            ),
            (
                vec![RIVAL_BRENDAN, STARTER_TREECKO],
                OrTrainerBattle {
                    battles: vec![BRENDAN_ROUTE_110_TREECKO]
                }
            ),
            (
                vec![RIVAL_BRENDAN, STARTER_TORCHIC],
                OrTrainerBattle {
                    battles: vec![BRENDAN_ROUTE_110_TORCHIC]
                }
            ),
            (
                vec![RIVAL_MAY, STARTER_MUDKIP],
                OrTrainerBattle {
                    battles: vec![MAY_ROUTE_110_MUDKIP]
                }
            ),
            (
                vec![RIVAL_MAY, STARTER_TREECKO],
                OrTrainerBattle {
                    battles: vec![MAY_ROUTE_110_TREECKO]
                }
            ),
            (
                vec![RIVAL_MAY, STARTER_TORCHIC],
                OrTrainerBattle {
                    battles: vec![MAY_ROUTE_110_TORCHIC]
                }
            )
        ]),
        Node::Single(EDWARD),
        Node::Single(ALYSSA),
        Node::ChangeLocation("mauville_city"),
        Node::Single(WALLY_MAUVILLE),
        Node::ChangeLocation("mauville_city_gym"),
        Node::Single(SHAWN),
        Node::Single(ANGELO),
        Node::Single(WATTSON_1),
        Node::ChangeLocation("mauville_city"),
        Node::ChangeLocation("route_111"),
        Node::ChangeLocation("fiery_path"),
        Node::ChangeLocation("route_111"),
        Node::Single(BROOKE_1),
        Node::ChangeLocation("route_113"),
        Node::ChangeLocation("fallarbor_town"),
        Node::ChangeLocation("route_114"),
        Node::Single(LUCAS_1),
        Node::ChangeLocation("meteor_falls"),
        Node::ChangeLocation("route_115"),
        Node::Single(MARLENE),
        Node::ChangeLocation("russboro_city"),
        Node::ChangeLocation("route_116"),
        Node::ChangeLocation("rusturf_tunnel"),
        Node::Single(MIKE_2),
        Node::ChangeLocation("verdanturf_town"),
        Node::ChangeLocation("route_117"),
        Node::ChangeLocation("mauville_city"),
        Node::ChangeLocation("route_111"),
        Node::ChangeLocation("cable_car"),
        Node::ChangeLocation("mt_chimney"),
        Node::Single(GRUNT_MT_CHIMNEY_1),
        Node::Single(GRUNT_MT_CHIMNEY_2),
        Node::Single(TABITHA_MT_CHIMNEY),
        Node::Single(MAXIE_MT_CHIMNEY),
        Node::ChangeLocation("jagged_pass"),
        Node::ChangeLocation("route_112"),
        Node::ChangeLocation("lavaridge_town"),
        Node::ChangeLocation("lavaridge_town_gym"),
        Node::Single(FLANNERY_1),
        Node::ChangeLocation("lavaridge_town"),
        Node::ChangeLocation("route_112"),
        Node::ChangeLocation("route_111"),
        Node::ChangeLocation("mauville_city"),
        Node::ChangeLocation("route_110"),
        Node::ChangeLocation("slateport_city"),
        Node::ChangeLocation("dewford_town"),
        Node::ChangeLocation("dewford_town_gym"),
        Node::Single(BRAWLY_1),
        Node::ChangeLocation("route_104"),
        Node::ChangeLocation("petalburg_city"),
        Node::ChangeLocation("petalburg_city_gym"),
        Node::CondOr(vec![
            (
                vec![PBCG_LLR],
                OrTrainerBattle {
                    battles: vec![RANDALL, PARKER, JODY]
                }
            ),
            (
                vec![PBCG_LRL],
                OrTrainerBattle {
                    battles: vec![RANDALL, ALEXIA, JODY]
                }
            ),
            (
                vec![PBCG_LRR],
                OrTrainerBattle {
                    battles: vec![RANDALL, ALEXIA, BERKE]
                }
            ),
            (
                vec![PBCG_RLL],
                OrTrainerBattle {
                    battles: vec![MARY, ALEXIA, JODY]
                }
            ),
            (
                vec![PBCG_RLR],
                OrTrainerBattle {
                    battles: vec![MARY, ALEXIA, BERKE]
                }
            ),
            (
                vec![PBCG_RRL],
                OrTrainerBattle {
                    battles: vec![MARY, GEORGE, BERKE]
                }
            )
        ]),
        Node::Single(NORMAN_1),
        Node::ChangeLocation("petalburg_city"),
        Node::ChangeLocation("route_102"),
        Node::ChangeLocation("oldale_town"),
        Node::ChangeLocation("route_103"),
        Node::Cond(PARTY_SIZE_GREATER_THAN_ONE, AMY_AND_LIV_1),
        Node::ChangeLocation("route_110"),
        Node::ChangeLocation("mauville_city"),
        Node::ChangeLocation("route_118"),
        Node::Single(DEANDRE),
        Node::Single(ROSE_1),
        Node::ChangeLocation("route_119"),
        Node::Or(vec![
            OrTrainerBattle {
                battles: vec![GRUNT_WEATHER_INST_1]
            },
            OrTrainerBattle {
                battles: vec![GRUNT_WEATHER_INST_4]
            }
        ]),
        Node::Single(GRUNT_WEATHER_INST_2),
        Node::Single(GRUNT_WEATHER_INST_5),
        Node::Single(SHELLY_WEATHER_INSTITUTE),
        Node::ChangeLocation("route_119"),
        Node::CondOr(vec![
            (
                vec![RIVAL_BRENDAN, STARTER_MUDKIP],
                OrTrainerBattle {
                    battles: vec![BRENDAN_ROUTE_119_MUDKIP]
                }
            ),
            (
                vec![RIVAL_BRENDAN, STARTER_TREECKO],
                OrTrainerBattle {
                    battles: vec![BRENDAN_ROUTE_119_TREECKO]
                }
            ),
            (
                vec![RIVAL_BRENDAN, STARTER_TORCHIC],
                OrTrainerBattle {
                    battles: vec![BRENDAN_ROUTE_119_TORCHIC]
                }
            ),
            (
                vec![RIVAL_MAY, STARTER_MUDKIP],
                OrTrainerBattle {
                    battles: vec![MAY_ROUTE_119_MUDKIP]
                }
            ),
            (
                vec![RIVAL_MAY, STARTER_TREECKO],
                OrTrainerBattle {
                    battles: vec![MAY_ROUTE_119_TREECKO]
                }
            ),
            (
                vec![RIVAL_MAY, STARTER_TORCHIC],
                OrTrainerBattle {
                    battles: vec![MAY_ROUTE_119_TORCHIC]
                }
            )
        ]),
        Node::ChangeLocation("fortree_city"),
        Node::ChangeLocation("route_120"),
        Node::ChangeLocation("fortree_city"),
        Node::ChangeLocation("fortree_city_gym"),
        Node::Single(FLINT),
        Node::Single(EDWARDO),
        Node::Single(DARIUS),
        Node::Single(WINONA_1),
        Node::ChangeLocation("fortree_city"),
        Node::ChangeLocation("route_120"),
        Node::ChangeLocation("route_121"),
        Node::ChangeLocation("route_122"),
        Node::ChangeLocation("mt_pyre"),
        Node::Single(GRUNT_MT_PYRE_2),
        Node::Single(GRUNT_MT_PYRE_1),
        Node::Single(GRUNT_MT_PYRE_3),
        Node::Single(GRUNT_MT_PYRE_4),
        Node::ChangeLocation("route_122"),
        Node::ChangeLocation("route_121"),
        Node::ChangeLocation("lilycove_city"),
        Node::ChangeLocation("lavaridge_town"),
        Node::ChangeLocation("route_112"),
        Node::ChangeLocation("route_111"),
        Node::ChangeLocation("cable_car"),
        Node::ChangeLocation("mt_chimney"),
        Node::Single(GRUNT_MAGMA_HIDEOUT_2),
        Node::Single(GRUNT_MAGMA_HIDEOUT_3),
        Node::Single(GRUNT_MAGMA_HIDEOUT_7),
        Node::Single(GRUNT_MAGMA_HIDEOUT_11),
        Node::Single(GRUNT_MAGMA_HIDEOUT_16),
        Node::Single(GRUNT_MAGMA_HIDEOUT_15),
        Node::Single(GRUNT_MAGMA_HIDEOUT_13),
        Node::Single(TABITHA_MAGMA_HIDEOUT),
        Node::Single(MAXIE_MAGMA_HIDEOUT),
        Node::ChangeLocation("jagged_pass"),
        Node::ChangeLocation("slateport_city"),
        Node::ChangeLocation("lilycove_city"),
        Node::ChangeLocation("aqua_hideout"),
        Node::Single(GRUNT_AQUA_HIDEOUT_2),
        Node::Single(GRUNT_AQUA_HIDEOUT_7),
        Node::Single(GRUNT_AQUA_HIDEOUT_6),
        Node::Single(GRUNT_AQUA_HIDEOUT_8),
        Node::Single(MATT),
        Node::ChangeLocation("route_124"),
        Node::Single(DECLAN),
        Node::ChangeLocation("mossdeep_city"),
        Node::ChangeLocation("mossdeep_city_gym"),
        Node::Single(PRESTON),
        Node::Single(MAURA),
        Node::Single(BLAKE),
        Node::Single(SAMANTHA),
        Node::Single(KATHLEEN),
        Node::Single(NICHOLAS),
        Node::Single(MACEY),
        Node::Single(HANNAH),
        Node::Cond(PARTY_SIZE_GREATER_THAN_ONE, TATE_AND_LIZA_1),
        Node::ChangeLocation("mossdeep_city"),
        Node::ChangeLocation("mossdeep_city_space_center"),
        Node::Single(GRUNT_SPACE_CENTER_4),
        Node::Single(GRUNT_SPACE_CENTER_2),
        Node::Single(GRUNT_SPACE_CENTER_5),
        Node::Single(GRUNT_SPACE_CENTER_6),
        Node::Single(GRUNT_SPACE_CENTER_7),
        Node::Single(MAXIE_MOSSDEEP),
        Node::Single(TABITHA_MOSSDEEP),
        Node::ChangeLocation("mossdeep_city"),
        Node::ChangeLocation("route_127"),
        Node::ChangeLocation("route_128"),
        Node::ChangeLocation("seafloor_cavern"),
        Node::Single(ARCHIE),
        Node::ChangeLocation("route_128"),
        Node::ChangeLocation("route_127"),
        Node::ChangeLocation("route_126"),
        Node::ChangeLocation("sootopolis_city"),
        Node::ChangeLocation("cave_of_origin"),
        Node::ChangeLocation("sootopolis_city"),
        Node::ChangeLocation("route_126"),
        Node::ChangeLocation("route_127"),
        Node::ChangeLocation("route_128"),
        Node::ChangeLocation("route_129"),
        Node::ChangeLocation("route_130"),
        Node::ChangeLocation("route_131"),
        Node::ChangeLocation("sky_pillar"),
        Node::ChangeLocation("stootopolis_city"),
        Node::ChangeLocation("stootopolis_city_gym"),
        Node::Single(JUAN_1),
        Node::ChangeLocation("stootopolis_city"),
        Node::ChangeLocation("route_126"),
        Node::ChangeLocation("route_127"),
        Node::ChangeLocation("route_128"),
        Node::ChangeLocation("ever_grande_city"),
        Node::ChangeLocation("victory_road"),
        Node::Single(WALLY_VR_1),
        Node::Single(HOPE),
        Node::Single(SHANNON),
        Node::Single(JULIE),
        Node::Single(EDGAR),
        Node::ChangeLocation("ever_grande_city"),
        Node::ChangeLocation("elite_four"),
        Node::Single(SIDNEY),
        Node::Single(PHOEBE),
        Node::Single(GLACIA),
        Node::Single(DRAKE),
        Node::Single(WALLACE),
    ];
    static ref POSSIBILITIES_FACT_SETS: Vec<Facts> = {
        let mut result = vec![];

        for starter in &STARTER_CONDITIONS {
            for rival in &RIVAL_CONDITIONS {
                for pbcg in &PBCG_CONDITIONS {
                    let mut facts = Facts::new();
                    facts.add_fact(starter);
                    facts.add_fact(rival);
                    facts.add_fact(pbcg);
                    result.push(facts);
                }
            }
        }

        result
    };
    static ref TRAINER_LOCATION_TABLE: HashMap<i32, &'static str> = {
        let mut result = HashMap::new();
        let mut last_location = "unknown";
        for node in FULL_ROUTE.iter() {
            match node {
                Node::ChangeLocation(location) => {
                    last_location = location;
                }
                Node::Single(trainer_battle) => {
                    result.insert(trainer_battle.trainer_id, last_location);
                }
                Node::Cond(_, trainer_battle) => {
                    result.insert(trainer_battle.trainer_id, last_location);
                }
                Node::CondOr(items) => {
                    for (_, or_battle) in items {
                        for trainer_battle in &or_battle.battles {
                            result.insert(trainer_battle.trainer_id, last_location);
                        }
                    }
                }
                Node::Or(possibilities) => {
                    for possibility in possibilities {
                        for trainer_battle in &possibility.battles {
                            result.insert(trainer_battle.trainer_id, last_location);
                        }
                    }
                }
            }
        }

        result
    };
}

fn get_trainer_location(trainer_id: i32) -> &'static str {
    TRAINER_LOCATION_TABLE
        .get(&trainer_id)
        .unwrap_or(&"unknown")
}

#[derive(Debug, Clone)]
struct Facts {
    conditions: HashMap<&'static str, &'static str>,
}

impl Facts {
    fn new() -> Self {
        Facts {
            conditions: HashMap::new(),
        }
    }

    fn add_fact(&mut self, condition: &Condition) {
        self.conditions.insert(condition.key, condition.value);
    }

    fn is_true(&self, condition: &Condition) -> bool {
        self.conditions.get(condition.key) == Some(&condition.value)
    }
}

fn generate_trainer_battle_lists_recsusvie(
    facts: &Facts,
    remaining_nodes: Vec<Node>,
    mut trainers_to_battle: Vec<i32>,
    result_sets: &mut Vec<Vec<i32>>,
) {
    for i in 0..remaining_nodes.len() {
        let node = &remaining_nodes[i];
        match node {
            Node::ChangeLocation(_) => {}
            Node::Single(trainer_battle) => {
                trainers_to_battle.push(trainer_battle.trainer_id);
            }
            Node::Cond(cond, battle) => {
                if facts.is_true(cond) {
                    trainers_to_battle.push(battle.trainer_id);
                }
            }
            Node::CondOr(items) => {
                for (conditions, or_battle) in items {
                    if conditions.iter().all(|cond| facts.is_true(cond)) {
                        for trainer_battle in &or_battle.battles {
                            trainers_to_battle.push(trainer_battle.trainer_id);
                        }
                    }
                }
            }
            Node::Or(possibilities) => {
                for possibility in possibilities {
                    let mut trainers_to_battle = trainers_to_battle.clone();
                    for trainer_battle in &possibility.battles {
                        trainers_to_battle.push(trainer_battle.trainer_id);
                    }
                    generate_trainer_battle_lists_recsusvie(
                        facts,
                        remaining_nodes[i + 1..].to_vec(),
                        trainers_to_battle,
                        result_sets,
                    );
                }
                return;
            }
        }
    }

    result_sets.push(trainers_to_battle);
}

fn generate_trainer_battle_lists(facts: &Facts) -> Vec<Vec<i32>> {
    let mut result_sets = vec![];
    generate_trainer_battle_lists_recsusvie(facts, FULL_ROUTE.clone(), vec![], &mut result_sets);
    result_sets
}

#[derive(Debug, Clone, Serialize)]
pub struct RouteStep {
    pub location: &'static str,
    pub other_trainer_id: i32,
    pub battle_id: Option<i32>,
    pub series_id: Option<i32>,
    pub won: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum RouteResult {
    Completed,
    Lost,
    IncompleteData(i32),
}

#[derive(Debug, Clone, Serialize)]
pub struct RouteProgressSet {
    pub progress: Vec<RouteStep>,
    pub result: RouteResult,
}

#[derive(Queryable, Clone)]
struct RouteBattle {
    battle_id: i32,
    series_id: i32,
    opponent_perspective: i32,
    player_perspective_won: bool,
}

fn construct_route_matchup_cache(
    conn: &mut PooledConnection,
    trainer_id: i32,
    routes: &Vec<Vec<i32>>,
) -> HashMap<i32, Option<RouteBattle>> {
    use schema::battles;

    let mut all_routes_trainers = HashSet::new();

    for route in routes {
        all_routes_trainers.extend(route.iter().cloned());
    }

    let mut matchup_cache = HashMap::new();

    let battles: Vec<RouteBattle> = battles::table
        .select((
            battles::id,
            battles::series_id,
            battles::opponent_perspective,
            battles::player_perspective_won,
        ))
        .filter(
            battles::player_perspective
                .eq(trainer_id)
                .and(battles::opponent_perspective.eq_any(&all_routes_trainers)),
        )
        .load::<RouteBattle>(conn)
        .unwrap();

    for battle in battles {
        let route_trainer = battle.opponent_perspective;

        if battle.player_perspective_won {
            matchup_cache.insert(route_trainer, Some(battle.clone()));
        } else {
            if !matchup_cache.contains_key(&route_trainer) {
                matchup_cache.insert(route_trainer, Some(battle.clone()));
            }
        }
    }

    for trainer_id in all_routes_trainers {
        if !matchup_cache.contains_key(&trainer_id) {
            matchup_cache.insert(trainer_id, None);
        }
    }

    matchup_cache
}

fn get_route_progress(
    matchup_cache: &mut HashMap<i32, Option<RouteBattle>>,
    trainer_id: i32,
    route: Vec<i32>,
) -> RouteProgressSet {
    let mut progress = vec![];
    let mut result = None;

    for route in route {
        if route == trainer_id {
            progress.push(RouteStep {
                location: get_trainer_location(route),
                other_trainer_id: route,
                battle_id: None,
                series_id: None,
                won: true,
            });
            continue;
        }

        let battle = matchup_cache.get(&route).cloned().unwrap_or_default();

        let battle = match battle {
            Some(battles) => battles,
            None => {
                progress.push(RouteStep {
                    location: get_trainer_location(route),
                    other_trainer_id: route,
                    battle_id: None,
                    series_id: None,
                    won: false,
                });
                continue;
            }
        };

        progress.push(RouteStep {
            location: get_trainer_location(route),
            other_trainer_id: route,
            battle_id: Some(battle.battle_id),
            series_id: Some(battle.series_id),
            won: battle.player_perspective_won,
        });
    }

    if result.is_none() {
        if let Some(last) = progress.last() {
            if last.won {
                if last.other_trainer_id == WALLACE.trainer_id {
                    result = Some(RouteResult::Completed);
                }
            } else {
                result = Some(RouteResult::Lost);
            }
        }
    }

    RouteProgressSet {
        progress,
        result: result.unwrap(),
    }
}

pub fn get_progress_of_trainer_id(
    conn: &mut PooledConnection,
    trainer_id: i32,
) -> RouteProgressSet {
    let trainer = GEN3.get_trainer_by_id(trainer_id);

    let fact_sets = POSSIBILITIES_FACT_SETS.clone();

    let party_fact = if trainer.party_size > 1 {
        &PARTY_SIZE_GREATER_THAN_ONE
    } else {
        &PARTY_SIZE_ONE
    };

    let mut possible_routes = vec![];

    for mut facts in fact_sets.into_iter() {
        facts.add_fact(party_fact);
        possible_routes.extend(generate_trainer_battle_lists(&facts));
    }

    let mut best_completed = std::i32::MIN;
    let mut best = None;

    let mut matchup_cache = construct_route_matchup_cache(conn, trainer_id, &possible_routes);

    for route in possible_routes {
        let progress = get_route_progress(&mut matchup_cache, trainer_id, route);

        let mut wins = 0;
        for step in &progress.progress {
            if step.won {
                wins += 1;
            } else {
                break;
            }
        }

        if wins > best_completed {
            best_completed = wins;
            best = Some(progress);
        }
    }

    match best {
        Some(best) => best,
        None => RouteProgressSet {
            progress: vec![],
            result: RouteResult::IncompleteData(0),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_trainer_battle_lists() {
        let mut total = 0;
        for facts in POSSIBILITIES_FACT_SETS.iter() {
            let result_sets = generate_trainer_battle_lists(facts);
            total += result_sets.len();
            assert_eq!(result_sets.len(), 2);
        }

        assert_eq!(total, 72);
    }
}
