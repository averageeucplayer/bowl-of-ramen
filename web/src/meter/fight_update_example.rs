use futures::{channel::mpsc, stream::{iter, StreamExt}, SinkExt, Stream};
use app_core::models::FightUpdate;
use log::info;
use serde_json::Value;
use tauri_sys::{event::Event, Error};


pub async fn fake_listen() -> Result<impl Stream<Item = Event<FightUpdate>>, Error> {

    let json_value: FightUpdate = serde_json::from_str(JSON_DATA).unwrap();
    let js_value = serde_wasm_bindgen::to_value(&json_value).unwrap();
    let payload: FightUpdate = serde_wasm_bindgen::from_value(js_value).unwrap();
    let (mut tx, rx) = mpsc::unbounded();
    
    let event = Event {
        id: 0,
        event: "test".into(),
        payload
    };
    
    tx.unbounded_send(event).unwrap();
    // let stream = iter(vec![event]);

    drop(tx);

    Ok(rx)
}

pub const JSON_DATA: &str = r#"
{
    "players": [
      {
        "id": 64083,
        "name": "Mcxeszdjdg",
        "class_name": "Souleater",
        "class_id": 405,
        "created_on": "2025-01-21T13:24:59.368382700Z",
        "stats": {
          "crit_rate": 0.6666667,
          "crit_damage": 1421668302,
          "top_damage": 720276060,
          "total_damage": 1747842916,
          "damage_percentage": 0.08543577,
          "dps": {
            "raw": 436960729,
            "abbreviated": "437.0m",
            "value": 436.960729,
            "unit": "m"
          },
          "back_attacks_total_damage": 1046450674,
          "front_attacks_total_damage": 701392242,
          "non_positional_attacks_total_damage": 0,
          "back_attacks_damage_percentage": 0.5987098,
          "front_attacks_damage_percentage": 0.4012902,
          "non_positional_attacks_damage_percentage": 0.0,
          "hyper_awakening_damage": 0,
          "updated_on": "2025-01-21T13:25:04.087000600Z",
          "skills": {
            "counter_count": 0,
            "hit_count": 3,
            "crit_count": 2,
            "skill": {
              "46440": {
                "id": 46440,
                "name": "Soul Drain",
                "icon": "se_skill_01_14.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 720276060,
                "total_damage": 720276060
              },
              "46220": {
                "id": 46220,
                "name": "Rusted Nail",
                "icon": "se_skill_01_6.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 701392242,
                "total_damage": 701392242
              },
              "46430": {
                "id": 46430,
                "name": "Astaros",
                "icon": "se_skill_01_13.png",
                "hit_count": 1,
                "crit_count": 0,
                "crit_damage": 0,
                "total_damage": 326174614
              }
            }
          }
        }
      },
      {
        "id": 50805,
        "name": "Bqvcyhpqwk",
        "class_name": "Soulfist",
        "class_id": 304,
        "created_on": "2025-01-21T13:24:59.368382700Z",
        "stats": {
          "crit_rate": 0.8,
          "crit_damage": 4392080652,
          "top_damage": 952878738,
          "total_damage": 4392080652,
          "damage_percentage": 0.2919426,
          "dps": {
            "raw": 1098020163,
            "abbreviated": "1.1b",
            "value": 1.098020163,
            "unit": "b"
          },
          "back_attacks_total_damage": 946491880,
          "front_attacks_total_damage": 0,
          "non_positional_attacks_total_damage": 3445588772,
          "back_attacks_damage_percentage": 0.21549965,
          "front_attacks_damage_percentage": 0.0,
          "non_positional_attacks_damage_percentage": 0.7845003,
          "hyper_awakening_damage": 0,
          "updated_on": "2025-01-21T13:25:01.976925600Z",
          "skills": {
            "counter_count": 0,
            "hit_count": 5,
            "crit_count": 5,
            "skill": {
              "24020": {
                "id": 24020,
                "name": "Hype",
                "icon": "so_skill_01_21.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 946491880,
                "total_damage": 946491880
              },
              "24210": {
                "id": 24210,
                "name": "Merciless Pummel",
                "icon": "so_skill_01_6.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 865743646,
                "total_damage": 865743646
              },
              "24110": {
                "id": 24110,
                "name": "Palm Burst",
                "icon": "so_skill_01_7.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 694289752,
                "total_damage": 694289752
              },
              "24040": {
                "id": 24040,
                "name": "Energy Blast",
                "icon": "so_skill_01_2.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 952878738,
                "total_damage": 952878738
              },
              "24290": {
                "id": 24290,
                "name": "Illusion Strike",
                "icon": "so_skill_01_26.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 932676636,
                "total_damage": 932676636
              }
            }
          }
        }
      },
      {
        "id": 85567,
        "name": "Zdnhbjtqcw",
        "class_name": "Breaker",
        "class_id": 313,
        "created_on": "2025-01-21T13:24:59.368382700Z",
        "stats": {
          "crit_rate": 0.8333333,
          "crit_damage": 4480420382,
          "top_damage": 948122508,
          "total_damage": 4480420382,
          "damage_percentage": 0.23650175,
          "dps": {
            "raw": 1120105095,
            "abbreviated": "1.1b",
            "value": 1.120105095,
            "unit": "b"
          },
          "back_attacks_total_damage": 550825648,
          "front_attacks_total_damage": 2155614478,
          "non_positional_attacks_total_damage": 1773980256,
          "back_attacks_damage_percentage": 0.12294062,
          "front_attacks_damage_percentage": 0.4811188,
          "non_positional_attacks_damage_percentage": 0.3959406,
          "hyper_awakening_damage": 0,
          "updated_on": "2025-01-21T13:25:04.087000600Z",
          "skills": {
            "counter_count": 0,
            "hit_count": 6,
            "crit_count": 6,
            "skill": {
              "47260": {
                "id": 47260,
                "name": "Celestial Force Barrage",
                "icon": "ifm_skill_01_20.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 825857748,
                "total_damage": 825857748
              },
              "47110": {
                "id": 47110,
                "name": "Hurricane Chain",
                "icon": "ifm_skill_01_5.png",
                "hit_count": 2,
                "crit_count": 2,
                "crit_damage": 1768304624,
                "total_damage": 1768304624
              },
              "47030": {
                "id": 47030,
                "name": "Featherweight",
                "icon": "ifm_skill_01_1.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 550825648,
                "total_damage": 550825648
              },
              "47180": {
                "id": 47180,
                "name": "Punishing Wave",
                "icon": "ifm_skill_01_12.png",
                "hit_count": 2,
                "crit_count": 2,
                "crit_damage": 1335432362,
                "total_damage": 1335432362
              }
            }
          }
        }
      },
      {
        "id": 40357,
        "name": "Lngizsbcas",
        "class_name": "Paladin",
        "class_id": 105,
        "created_on": "2025-01-21T13:24:59.368382700Z",
        "stats": {
          "crit_rate": 0.0,
          "crit_damage": 0,
          "top_damage": 63485,
          "total_damage": 87965,
          "damage_percentage": 0.0000049621367,
          "dps": {
            "raw": 21991,
            "abbreviated": "22.0k",
            "value": 21.991,
            "unit": "k"
          },
          "back_attacks_total_damage": 0,
          "front_attacks_total_damage": 87965,
          "non_positional_attacks_total_damage": 0,
          "back_attacks_damage_percentage": 0.0,
          "front_attacks_damage_percentage": 1.0,
          "non_positional_attacks_damage_percentage": 0.0,
          "hyper_awakening_damage": 0,
          "updated_on": "2025-01-21T13:25:03.555788200Z",
          "skills": {
            "counter_count": 0,
            "hit_count": 2,
            "crit_count": 0,
            "skill": {
              "36180": {
                "id": 36180,
                "name": "Execution of Justice",
                "icon": "hk_skill_01_15.png",
                "hit_count": 1,
                "crit_count": 0,
                "crit_damage": 0,
                "total_damage": 24480
              },
              "36090": {
                "id": 36090,
                "name": "Flash Slash",
                "icon": "hk_skill_01_6.png",
                "hit_count": 1,
                "crit_count": 0,
                "crit_damage": 0,
                "total_damage": 63485
              }
            }
          }
        }
      },
      {
        "id": 21928,
        "name": "Nqnnohaist",
        "class_name": "Artillerist",
        "class_id": 504,
        "created_on": "2025-01-21T13:24:59.368382700Z",
        "stats": {
          "crit_rate": 0.16666667,
          "crit_damage": 859771940,
          "top_damage": 859771940,
          "total_damage": 2739509264,
          "damage_percentage": 0.1360787,
          "dps": {
            "raw": 684877316,
            "abbreviated": "684.9m",
            "value": 684.877316,
            "unit": "m"
          },
          "back_attacks_total_damage": 1210291749,
          "front_attacks_total_damage": 830544717,
          "non_positional_attacks_total_damage": 698672798,
          "back_attacks_damage_percentage": 0.44179145,
          "front_attacks_damage_percentage": 0.3031728,
          "non_positional_attacks_damage_percentage": 0.25503573,
          "hyper_awakening_damage": 0,
          "updated_on": "2025-01-21T13:25:04.087000600Z",
          "skills": {
            "counter_count": 0,
            "hit_count": 6,
            "crit_count": 1,
            "skill": {
              "30260": {
                "id": 30260,
                "name": "Barrage: Focus Fire",
                "icon": "bs_skill_01_18.png",
                "hit_count": 2,
                "crit_count": 0,
                "crit_damage": 0,
                "total_damage": 698672798
              },
              "30190": {
                "id": 30190,
                "name": "Forward Barrage",
                "icon": "bs_skill_01_19.png",
                "hit_count": 2,
                "crit_count": 1,
                "crit_damage": 859771940,
                "total_damage": 1320004700
              },
              "30021": {
                "id": 30021,
                "name": "Exit Barrage Mode",
                "icon": "dh_skill_01_0.png",
                "hit_count": 1,
                "crit_count": 0,
                "crit_damage": 0,
                "total_damage": 370311957
              },
              "30180": {
                "id": 30180,
                "name": "Napalm Shot",
                "icon": "bs_skill_01_13.png",
                "hit_count": 1,
                "crit_count": 0,
                "crit_damage": 0,
                "total_damage": 350519809
              }
            }
          }
        }
      },
      {
        "id": 88491,
        "name": "Nqkpvtdinh",
        "class_name": "Deathblade",
        "class_id": 402,
        "created_on": "2025-01-21T13:24:59.368382700Z",
        "stats": {
          "crit_rate": 0.75,
          "crit_damage": 3340898044,
          "top_damage": 984444992,
          "total_damage": 3340898044,
          "damage_percentage": 0.23281454,
          "dps": {
            "raw": 835224511,
            "abbreviated": "835.2m",
            "value": 835.224511,
            "unit": "m"
          },
          "back_attacks_total_damage": 811782894,
          "front_attacks_total_damage": 2529115150,
          "non_positional_attacks_total_damage": 0,
          "back_attacks_damage_percentage": 0.24298345,
          "front_attacks_damage_percentage": 0.75701654,
          "non_positional_attacks_damage_percentage": 0.0,
          "hyper_awakening_damage": 0,
          "updated_on": "2025-01-21T13:25:01.976925600Z",
          "skills": {
            "counter_count": 0,
            "hit_count": 4,
            "crit_count": 4,
            "skill": {
              "25210": {
                "id": 25210,
                "name": "Head Hunt",
                "icon": "bl_skill_01_17.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 811782894,
                "total_damage": 811782894
              },
              "25190": {
                "id": 25190,
                "name": "Fatal Wave",
                "icon": "bl_skill_01_15.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 844972894,
                "total_damage": 844972894
              },
              "25031": {
                "id": 25031,
                "name": "Death Trance",
                "icon": "bl_skill_01_20.png",
                "hit_count": 2,
                "crit_count": 2,
                "crit_damage": 1684142256,
                "total_damage": 1684142256
              }
            }
          }
        }
      },
      {
        "id": 41143,
        "name": "Kugputhpfd",
        "class_name": "Sharpshooter",
        "class_id": 502,
        "created_on": "2025-01-21T13:24:59.368382700Z",
        "stats": {
          "crit_rate": 0.5,
          "crit_damage": 2893519106,
          "top_damage": 882113822,
          "total_damage": 3756666989,
          "damage_percentage": 0.18991046,
          "dps": {
            "raw": 939166747,
            "abbreviated": "939.2m",
            "value": 939.166747,
            "unit": "m"
          },
          "back_attacks_total_damage": 1463751570,
          "front_attacks_total_damage": 863147883,
          "non_positional_attacks_total_damage": 1429767536,
          "back_attacks_damage_percentage": 0.38964102,
          "front_attacks_damage_percentage": 0.2297643,
          "non_positional_attacks_damage_percentage": 0.3805947,
          "hyper_awakening_damage": 0,
          "updated_on": "2025-01-21T13:25:04.087000600Z",
          "skills": {
            "counter_count": 0,
            "hit_count": 6,
            "crit_count": 4,
            "skill": {
              "28210": {
                "id": 28210,
                "name": "Stalker",
                "icon": "he_skill_01_17.png",
                "hit_count": 1,
                "crit_count": 0,
                "crit_damage": 0,
                "total_damage": 470992234
              },
              "28140": {
                "id": 28140,
                "name": "Moving Slash",
                "icon": "he_skill_01_13.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 627054222,
                "total_damage": 627054222
              },
              "28250": {
                "id": 28250,
                "name": "Hawk Shot",
                "icon": "he_skill_01_25.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 836697348,
                "total_damage": 836697348
              },
              "28120": {
                "id": 28120,
                "name": "DM-42",
                "icon": "he_skill_01_11.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 882113822,
                "total_damage": 882113822
              },
              "28200": {
                "id": 28200,
                "name": "Shadow Arrow",
                "icon": "he_skill_01_16.png",
                "hit_count": 1,
                "crit_count": 0,
                "crit_damage": 0,
                "total_damage": 392155649
              },
              "28320": {
                "id": 28320,
                "name": "Wings of Storm",
                "icon": "ark_passive_he_5.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 547653714,
                "total_damage": 547653714
              }
            }
          }
        }
      },
      {
        "id": 32185,
        "name": "Fgnruczrmo",
        "class_name": "Artist",
        "class_id": 602,
        "created_on": "2025-01-21T13:24:59.368382700Z",
        "stats": {
          "crit_rate": 0.16666667,
          "crit_damage": 196292,
          "top_damage": 196292,
          "total_damage": 468885,
          "damage_percentage": 0.000023703471,
          "dps": {
            "raw": 117221,
            "abbreviated": "117.2k",
            "value": 117.221,
            "unit": "k"
          },
          "back_attacks_total_damage": 0,
          "front_attacks_total_damage": 256129,
          "non_positional_attacks_total_damage": 212756,
          "back_attacks_damage_percentage": 0.0,
          "front_attacks_damage_percentage": 0.54625124,
          "non_positional_attacks_damage_percentage": 0.4537488,
          "hyper_awakening_damage": 0,
          "updated_on": "2025-01-21T13:25:04.087000600Z",
          "skills": {
            "counter_count": 0,
            "hit_count": 6,
            "crit_count": 1,
            "skill": {
              "31051": {
                "id": 31051,
                "name": "Moonfall",
                "icon": "yy_skill_01_3.png",
                "hit_count": 1,
                "crit_count": 0,
                "crit_damage": 0,
                "total_damage": 87192
              },
              "31040": {
                "id": 31040,
                "name": "Stroke: Here and There",
                "icon": "yy_skill_01_2.png",
                "hit_count": 1,
                "crit_count": 1,
                "crit_damage": 196292,
                "total_damage": 196292
              },
              "31020": {
                "id": 31020,
                "name": "Stroke: Spill",
                "icon": "yy_skill_01_1.png",
                "hit_count": 2,
                "crit_count": 0,
                "crit_damage": 0,
                "total_damage": 97682
              },
              "31470": {
                "id": 31470,
                "name": "Stroke: One Stroke",
                "icon": "yy_skill_01_16.png",
                "hit_count": 2,
                "crit_count": 0,
                "crit_damage": 0,
                "total_damage": 87719
              }
            }
          }
        }
      }
    ],
    "boss": {
      "id": 86247,
      "name": "Test Boss",
      "stats": {
        "max_hp": 100000000000,
        "hp": 79542024903,
        "max_hp_bars": 300,
        "hp_bars": 238,
        "hp_per_bar": 333333340.0,
        "hp_percentage": 79.54203,
        "damage_taken": 326174614,
        "updated_on": "2025-01-21T13:25:04.087000600Z"
      },
      "created_on": "2025-01-21T13:24:59.368693300Z"
    }
}
"#;