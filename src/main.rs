fn main() {}

// use std::borrow::Cow;

// use sqlx::{Pool, Postgres};

// mod database;
// mod settings;

// use database::{get_configuration, get_pool, Settings};
// use dawg::async_impl::Dawg;

// #[tokio::main]
// async fn main() {
//     let config = get_configuration().unwrap();
//     let pool = get_pool(&config.db);

//     let ddawg = build(config, pool).await;

//     // println!("the dawg o >>>>>>--------- {ddawg:#?}");
// }

// async fn build(_config: Settings, pool: Pool<Postgres>) -> Dawg {
//     // migrations
//     let migration = sqlx::migrate!("./migrations").run(&pool).await;

//     if let Err(e) = migration {
//         eprintln!("THERE WAS AN ERROR MIGRATING THE DAWGIE {e:#?}");
//     }

//     // let mut dawgie = Dawg::new(pool.clone()).await;

//     let mut dawgie = Dawg::new(pool.clone()).await;

//     let mut words = vec![
//         "BAM", "BAT", "BATH", "CATH", "BATHE", "CAR", "CARS", "CAREERS", "CATH", "CRASE", "HUMAN",
//         "a", "aliancia", "alpa", "aloa", "alobal",
//     ]
//     .iter()
//     .map(|x| x.to_string())
//     .collect::<Vec<_>>()
//     .iter()
//     .map(|x| x.to_uppercase())
//     .collect::<Vec<_>>();

//     words.sort();

//     // for word in words {
//     //     dawgie.insert(word).await;
//     // }

//     // println!(
//     //     "dawgie please find CARS >>>>>>>>>> {:#?} \n\n",
//     //     dawgie
//     //         .is_word(
//     //             vec![
//     //                 String::from("C"),
//     //                 String::from("A"),
//     //                 String::from("R"),
//     //                 String::from("S")
//     //             ],
//     //             false
//     //         )
//     //         .await
//     //         .is_some()
//     // );

//     // println!(
//     //     "dawgie please find BATHS >>>>>>>>>> {:#?} \n\n",
//     //     dawgie
//     //         .is_word(
//     //             vec![
//     //                 String::from("B"),
//     //                 String::from("A"),
//     //                 String::from("T"),
//     //                 String::from("H"),
//     //                 String::from("S"),
//     //             ],
//     //             false
//     //         )
//     //         .await
//     //         .is_none()
//     // );

//     // println!(
//     //     "dawgie please find BATH >>>>>>>>>> {:#?} \n\n",
//     //     dawgie
//     //         .is_word(
//     //             vec![
//     //                 String::from("B"),
//     //                 String::from("A"),
//     //                 String::from("T"),
//     //                 String::from("H"),
//     //             ],
//     //             false
//     //         )
//     //         .await
//     //         .is_some()
//     // );

//     // println!(
//     //     "dawgie please find alobal >>>>>>>>>> {:#?} \n\n",
//     //     dawgie
//     //         .is_word(
//     //             vec![
//     //                 String::from("a"),
//     //                 String::from("l"),
//     //                 String::from("o"),
//     //                 String::from("b"),
//     //                 String::from("a"),
//     //                 String::from("l"),
//     //             ],
//     //             false
//     //         )
//     //         .await
//     //         .is_some()
//     // );

//     // println!(
//     //     "dawgie please find alianca >>>>>>>>>> {:#?} \n\n",
//     //     dawgie
//     //         .is_word(
//     //             vec![
//     //                 String::from("a"),
//     //                 String::from("l"),
//     //                 String::from("i"),
//     //                 String::from("a"),
//     //                 String::from("n"),
//     //                 String::from("c"),
//     //                 String::from("a"),
//     //             ],
//     //             false
//     //         )
//     //         .await
//     //         .is_none()
//     // );

//     // println!(
//     //     "dawgie please find aliancia >>>>>>>>>> {:#?} \n\n",
//     //     dawgie
//     //         .is_word(
//     //             vec![
//     //                 String::from("a"),
//     //                 String::from("l"),
//     //                 String::from("i"),
//     //                 String::from("a"),
//     //                 String::from("n"),
//     //                 String::from("c"),
//     //                 String::from("i"),
//     //                 String::from("a"),
//     //             ],
//     //             false
//     //         )
//     //         .await
//     //         .is_some()
//     // );

//     // println!(
//     //     "dawgie please find CRASE >>>>>>>>>> {:#?} \n\n",
//     //     dawgie
//     //         .is_word(
//     //             vec![
//     //                 String::from("C"),
//     //                 String::from("R"),
//     //                 String::from("A"),
//     //                 String::from("S"),
//     //                 String::from("E"),
//     //             ],
//     //             false
//     //         )
//     //         .await
//     //         .is_some()
//     // );

//     // println!(
//     //     "dawgie please find CRAZE >>>>>>>>>> {:#?} \n\n",
//     //     dawgie
//     //         .is_word(
//     //             vec![
//     //                 String::from("C"),
//     //                 String::from("R"),
//     //                 String::from("A"),
//     //                 String::from("Z"),
//     //                 String::from("E"),
//     //             ],
//     //             false
//     //         )
//     //         .await
//     //         .is_none()
//     // );

//     dawgie.finish().await;

//     // let dawgie = SqlDawg::init(pool).await;

//     dawgie
// }
