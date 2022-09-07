#![feature(proc_macro_hygiene, decl_macro)]

mod db;
mod models;
mod schema;

mod server {
    use crate::db::*;
    use crate::models::*;
    use actix_web::{delete, error, get, post, web, Error, HttpResponse};
    use diesel::pg::PgConnection;
    use diesel::r2d2::ConnectionManager;

    pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

    #[post("/house")]
    pub async fn house_create(
        pool: web::Data<DbPool>,
        new_house: web::Json<NewHouse>,
    ) -> Result<HttpResponse, Error> {
        let new_house = web::block(move || {
            let conn = &mut pool.get()?;
            let name = new_house.name.clone();
            create_house(conn, name)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(new_house))
    }

    #[get("/house/{id}")]
    pub async fn house_view(
        pool: web::Data<DbPool>,
        id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let house = web::block(move || {
            let conn = &mut pool.get()?;
            get_house(conn, *id)
        })
        .await?
        .map_err(error::ErrorNotFound)?;

        Ok(HttpResponse::Ok().json(house))
    }

    #[delete("/house/{id}")]
    pub async fn house_delete(
        pool: web::Data<DbPool>,
        id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let res = web::block(move || {
            let conn = &mut pool.get()?;
            delete_house(conn, *id)
        })
        .await?
        .map_err(error::ErrorNotFound)?;

        if res {
            Ok(HttpResponse::Ok().json(()))
        } else {
            Ok(HttpResponse::NotFound().json(()))
        }
    }

    #[post("/room")]
    pub async fn room_create(
        pool: web::Data<DbPool>,
        new_room: web::Json<NewRoom>,
    ) -> Result<HttpResponse, Error> {
        let new_room = web::block(move || {
            let conn = &mut pool.get()?;
            let name = new_room.name.clone();
            let house_id = new_room.house_id;
            create_room(conn, name, house_id)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(new_room))
    }

    #[get("/room/{id}")]
    pub async fn room_view(
        pool: web::Data<DbPool>,
        id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let room = web::block(move || {
            let conn = &mut pool.get()?;
            get_room(conn, *id)
        })
        .await?
        .map_err(error::ErrorNotFound)?;

        Ok(HttpResponse::Ok().json(room))
    }

    #[delete("/room/{id}")]
    pub async fn room_delete(
        pool: web::Data<DbPool>,
        id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let res = web::block(move || {
            let conn = &mut pool.get()?;
            delete_room(conn, *id)
        })
        .await?
        .map_err(error::ErrorNotFound)?;

        if res {
            Ok(HttpResponse::Ok().json(()))
        } else {
            Ok(HttpResponse::NotFound().json(()))
        }
    }

    #[post("/device")]
    pub async fn device_create(
        pool: web::Data<DbPool>,
        new_device: web::Json<NewDevice>,
    ) -> Result<HttpResponse, Error> {
        let name = new_device.name.clone();
        let device_type = new_device.type_.clone();
        let state = new_device.state.clone();
        let room_id = new_device.room_id;

        let device = web::block(move || {
            let conn = &mut pool.get()?;
            let device_info = get_device_info(device_type, state)?;

            create_device(conn, room_id, name, device_info)
        })
        .await?
        .map_err(error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(device))
    }

    #[get("/device/{id}")]
    pub async fn device_view(
        pool: web::Data<DbPool>,
        id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let device = web::block(move || {
            let conn = &mut pool.get()?;
            view_device(conn, *id)
        })
        .await?
        .map_err(error::ErrorNotFound)?;

        Ok(HttpResponse::Ok().json(device))
    }

    #[delete("/device/{id}")]
    pub async fn device_delete(
        pool: web::Data<DbPool>,
        id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let res = web::block(move || {
            let conn = &mut pool.get()?;
            delete_device(conn, *id)
        })
        .await?
        .map_err(error::ErrorNotFound)?;

        if res {
            Ok(HttpResponse::Ok().json(()))
        } else {
            Ok(HttpResponse::NotFound().json(()))
        }
    }

    #[get("/report/{id}")]
    pub async fn report(
        pool: web::Data<DbPool>,
        house_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
        let res = web::block(move || {
            let conn = &mut pool.get()?;
            house_report(conn, *house_id)
        })
        .await?
        .map_err(error::ErrorNotFound)?;

        Ok(HttpResponse::Ok().json(res))
    }

    fn get_device_info(
        device_type: String,
        state: serde_json::Value,
    ) -> Result<DeviceInfo, String> {
        let device_info = match device_type.as_str() {
            "thermo" => {
                let thermo: ThermoInfo =
                    serde_json::from_value(state).map_err(|_| "can't parse ".to_string())?;
                DeviceInfo::Thermo(thermo)
            }
            "socket" => {
                let socket: SocketInfo =
                    serde_json::from_value(state).map_err(|_| "can't parse ".to_string())?;
                DeviceInfo::Socket(socket)
            }
            _ => {
                return Err("unknown device type".to_string());
            }
        };

        Ok(device_info)
    }
}

use actix_web::{middleware, web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::Pool;
use std::env;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migration(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");

    let conn = &mut pool.get().expect("can't get db connection");
    run_migration(conn);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(server::house_create)
            .service(server::house_view)
            .service(server::house_delete)
            .service(server::room_create)
            .service(server::room_view)
            .service(server::room_delete)
            .service(server::device_create)
            .service(server::device_view)
            .service(server::device_delete)
            .service(server::report)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
