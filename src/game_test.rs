use quickcheck::TestResult;
use rand::{thread_rng, Rng};

use super::*;
use bat::bat_tests::create_mock_provided_bat;
use bat::SuperBat;
use map;
use map::map_tests::gen_rand_valid_path_of_len;
use pit::BottomlessPit;
use player::player_tests::create_mock_directed_player;
use player::Action;
use wumpus::Wumpus;

pub fn new_game_from(p: Player, s: State) -> Game {
    let wumpus = Rc::new(Wumpus::new(s.wumpus));
    let hazzards: Vec<Rc<dyn Hazzard>> = vec![
        wumpus.clone(),
        Rc::new(BottomlessPit { room: s.pit1 }),
        Rc::new(BottomlessPit { room: s.pit2 }),
        Rc::new(SuperBat::new(s.bat1)),
        Rc::new(SuperBat::new(s.bat2)),
    ];

    Game {
        player: box p,
        wumpus,
        pit1_room: s.pit1,
        pit2_room: s.pit2,
        bat1_room: s.bat1,
        bat2_room: s.bat2,
        hazzards,
        is_cheating: false
    }
}

/// player moves into bat room, gets snatched back into the bat room, then
/// snatched to pit room.
#[test]
fn player_can_get_multi_snatched_into_pit() {
    let player_room = 1;
    let bat1_room = 2;
    let bat2_room = 20;
    let pit1_room = 3;
    let pit2_room = 19;
    let wumpus_room = 18;

    let player = box create_mock_directed_player(player_room, vec![Action::Move(bat1_room)]);

    let wumpus = Rc::new(Wumpus::new(wumpus_room));

    let hazzards: Vec<Rc<dyn Hazzard>> = vec![
        Rc::new(BottomlessPit { room: pit1_room }),
        Rc::new(BottomlessPit { room: pit2_room }),
        Rc::new(create_mock_provided_bat(
            bat1_room,
            vec![bat1_room, pit1_room]
        )),
        Rc::new(SuperBat::new(bat2_room)),
    ];

    let mut game = Game {
        player,
        wumpus,
        pit1_room,
        pit2_room,
        bat1_room,
        bat2_room,
        hazzards,
        is_cheating: false
    };
    assert_eq!(RunResult::KilledByPit, game.run());
}

/// loop up to the max where we can shoot through up to 5 rooms in line and
/// still miss the Wumpus by one.
#[test]
fn can_miss_by_one() {
    let max = map::NUM_OF_ROOMS - MAX_TRAVERSABLE;

    for room_num in 1..max {
        perform_trial(4, &|| {
            // room count includes the player's room plus the number of rooms for the
            // crooked arrow to traverse. [2, 6]
            let room_count = thread_rng().gen_range(2, MAX_TRAVERSABLE + 2);
            let rooms: Vec<_> = (room_num..(room_num + room_count)).collect();
            let wumpus = room_num + room_count;

            let shoot_result = traverse(&rooms, &get_inital_state(rooms[0], wumpus));

            assert_eq!(
                ShootResult::Miss,
                shoot_result,
                "rooms: {:?} wumpus: {}",
                rooms,
                wumpus
            );
        });
    }
}

/// loop up to the max where we can shoot through up to 5 rooms in line and hit
/// the Wumpus.
#[test]
fn can_hit() {
    let max = map::NUM_OF_ROOMS - MAX_TRAVERSABLE;

    for room_num in 1..max {
        perform_trial(4, &|| {
            let room_count = thread_rng().gen_range(2, MAX_TRAVERSABLE + 2);
            let rooms: Vec<_> = (room_num..(room_num + room_count)).collect();
            let wumpus = rooms[rooms.len() - 1];
            let shoot_result = traverse(&rooms, &get_inital_state(rooms[0], wumpus));

            assert_eq!(ShootResult::Hit, shoot_result);
        });
    }
}

#[quickcheck]
fn invalid_first_room_causes_random_traversal(room_to_shoot: RoomNum) -> TestResult {
    let player = 1;

    if !is_adj(player, room_to_shoot) {
        let wumpus = 20;
        let rooms = [player, room_to_shoot];
        // cannot shoot from a room not adjacent to the player.
        let shoot_result = traverse(&rooms, &get_inital_state(player, wumpus));

        TestResult::from_bool(ShootResult::Remaining(2, player) == shoot_result)
    } else {
        TestResult::discard()
    }
}

#[test]
fn disjoint_room_causes_random_traversal() {
    perform_trial(10, &|| {
        // get rand number from [1, 5] and leave room for the last room to be disjoint.
        let max = MAX_TRAVERSABLE + 1; // player room + 5 traversable rooms we can shoot.
        let room_count = thread_rng().gen_range(1, max);
        let mut paths = gen_rand_valid_path_of_len(room_count);

        assert_eq!(room_count, paths.len(), "must gen paths of the given len.");

        let last = paths[paths.len() - 1];
        let disjoint_room = get_rand_room_disjoint_from(last);
        paths.push(disjoint_room);

        let wumpus = 21; // off the map so we don't hit the Wumpus.
        let shoot_result = traverse(&paths, &get_inital_state(paths[0], wumpus));

        let last_valid = paths[paths.len() - 2];

        println!("{:?}", paths);

        assert_eq!(
            ShootResult::Remaining(2, last_valid),
            shoot_result,
            "paths: {:?}",
            &paths
        );
    });
}

#[test]
fn player_can_suicide() {
    let rooms = [1, 2, 3, 4, 5, 1];
    let player = 1;
    let wumpus = 20;
    let shoot_result = traverse(&rooms, &get_inital_state(player, wumpus));
    assert_eq!(ShootResult::Suicide, shoot_result);
}

fn get_rand_room_disjoint_from(room: RoomNum) -> RoomNum {
    loop {
        let r = map::rand_room();
        if !map::is_adj(r, room) {
            return r;
        }
    }
}

fn get_inital_state(player: RoomNum, wumpus: RoomNum) -> State {
    State {
        player,
        wumpus,
        arrow_count: 5,
        ..Default::default()
    }
}

fn perform_trial(trial_count: u32, trial: &dyn Fn()) {
    (0..trial_count).for_each(|_| trial());
}
