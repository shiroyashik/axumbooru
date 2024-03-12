// use dotenvy::dotenv;
// use sea_orm::{ConnectOptions, Database};
// use service::PostQuery;



// #[tokio::test]
// async fn query_with_filter() {
//     dotenv().ok();
//     // tracing_subscriber::fmt::init();

//     let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let opt = ConnectOptions::new(db_url);
//     let db = Database::connect(opt).await.expect("Database connection error!");

//     let fields_mas = [true, false, false, false, true, true, false, false, false, false, false, false, true, false, false];

//     let (results_raw, _) = PostQuery::find_posts_in_page_with_filter(&db, 0, fields_mas, 42)
//             .await
//             .unwrap();
    
//     println!("{results_raw:?}")
// }

// #[tokio::test]
// async fn query_without_filter() {
//     dotenv().ok();
//     // tracing_subscriber::fmt::init();

//     let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let opt = ConnectOptions::new(db_url);
//     let db = Database::connect(opt).await.expect("Database connection error!");

//     let (results_raw, _) = PostQuery::find_posts_in_page(&db, 0, 42)
//             .await
//             .unwrap();
    
//     println!("{results_raw:?}")
// }