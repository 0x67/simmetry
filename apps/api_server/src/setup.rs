use crate::{
    app_config::{AppConfig, APP_CONFIG},
    app_state::AppState,
    handlers::ws::{f1::create_f1_namespace, forza::create_forza_namespace},
    services::forza::ForzaService,
    types::{SocketIo, SocketIoLayer, SocketIoRedisAdapter},
};
use deadpool_diesel::{postgres::Manager, Pool};
use rs_shared::database::models::{
    f1::{F1Telemetry, F1Type},
    forza::ForzaTelemetry,
};
use socketioxide::SocketIo as SocketIoBase;
use socketioxide_redis::{RedisAdapter, RedisAdapterCtr};
use std::{sync::Arc, time::Duration};
use tokio::{
    fs::OpenOptions,
    io::AsyncWriteExt,
    spawn,
    sync::mpsc::{unbounded_channel, UnboundedSender},
};

async fn setup_forza_receiver(forza_service: Arc<ForzaService>) -> UnboundedSender<ForzaTelemetry> {
    let (forza_telemetry_sender, mut forza_telemetry_receiver) =
        unbounded_channel::<ForzaTelemetry>();

    spawn(async move {
        let mut buffer = Vec::with_capacity(1000);
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            tokio::select! {
                Some(forza_telemetry) = forza_telemetry_receiver.recv() => {
                    buffer.push(forza_telemetry);

                    if buffer.len() >= 1000 {
                        if let Err(e) = forza_service.create_forza_telemetry_batch(buffer.clone()).await {
                            warn!("Error creating forza telemetry batch: {:?}", e);
                        }
                        buffer.clear();
                    }
                }
                _ = interval.tick() => {
                    if !buffer.is_empty() {
                        if let Err(e) = forza_service.create_forza_telemetry_batch(buffer.clone()).await {
                            warn!("Error creating forza telemetry batch: {:?}", e);
                        }
                        buffer.clear();
                    }
                }
            }
        }
    });

    forza_telemetry_sender
}

async fn setup_f1_receiver() -> UnboundedSender<F1Telemetry> {
    let (f1_telemetry_sender, mut f1_telemetry_receiver) = unbounded_channel::<F1Telemetry>();

    spawn(async move {
        let mut buffer = Vec::with_capacity(1000);
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            tokio::select! {
                Some(f1_telemetry) = f1_telemetry_receiver.recv() => {
                    buffer.push(f1_telemetry);
                }
                _ = interval.tick() => {
                    if !buffer.is_empty() {
                          // TODO: save data
                          buffer.clear();
                    }
                }
            }
        }
    });

    f1_telemetry_sender
}

async fn setup_file_writer() -> UnboundedSender<Vec<u8>> {
    let (file_writer_sender, mut file_writer_receiver) = unbounded_channel::<Vec<u8>>();

    spawn(async move {
        let mut buffer = Vec::with_capacity(1000);
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        let file_name = format!("../../temp/{}.bin", F1Type::F12024);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_name)
            .await
            .expect("Unable to open or create file");

        loop {
            tokio::select! {
                  Some(mut buf) = file_writer_receiver.recv() => {
                      // Ensure each entry is exactly 2048 bytes
                      if buf.len() < 2048 {
                          buf.resize(2048, 0); // Pad with zeros
                      }

                      buffer.push(buf);
                  }
                  _ = interval.tick() => {
                      if !buffer.is_empty() {
                          for entry in &buffer {
                              file.write_all(entry).await.expect("Failed to write to file");
                          }

                          file.flush().await.expect("Failed to flush file");

                          buffer.clear();
                      }
                  }
            }
        }
    });

    file_writer_sender
}

async fn setup_redis() -> Result<(redis::Client, SocketIoRedisAdapter), Box<dyn std::error::Error>>
{
    info!("Connecting to redis");
    let redis_url = APP_CONFIG.redis_url.clone();
    let client = redis::Client::open(redis_url)?;
    let adapter = RedisAdapterCtr::new_with_redis(&client).await?;

    Ok((client, adapter))
}

async fn setup_db() -> Result<Pool<Manager>, Box<dyn std::error::Error>> {
    info!("Connecting to database");
    let database_url = APP_CONFIG.database_url.clone();
    let manager =
        deadpool_diesel::postgres::Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    Ok(pool)
}

pub async fn setup_app() -> Result<(AppState, SocketIoRedisAdapter), Box<dyn std::error::Error>> {
    // verify environment variables
    let _ = AppConfig::from_env()?;

    let (redis_client, adapter) = setup_redis().await.unwrap();
    let db_pool = setup_db().await.unwrap();

    let forza_service = Arc::new(ForzaService::new(db_pool.clone()));
    let forza_telemetry_sender = setup_forza_receiver(forza_service.clone()).await;

    let f1_telemetry_sender = setup_f1_receiver().await;
    let file_writer_sender = setup_file_writer().await;

    let app_state = AppState {
        redis_client,
        db_pool,
        forza_service,
        forza_telemetry_sender,
        f1_telemetry_sender,
        file_writer_sender,
    };

    Ok((app_state, adapter))
}

pub async fn setup_socketio(
    app_state: AppState,
    adapter: SocketIoRedisAdapter,
) -> Result<(SocketIoLayer, SocketIo), Box<dyn std::error::Error>> {
    info!("Creating socketio instance");
    let (layer, io) = SocketIoBase::builder()
        // .with_parser(ParserConfig::msgpack())
        .with_adapter::<RedisAdapter<_>>(adapter)
        .max_buffer_size(512)
        .max_payload(2048)
        .with_state(app_state.clone())
        .build_layer();

    let _ = create_forza_namespace(io.clone()).await?;
    let _ = create_f1_namespace(io.clone()).await?;
    Ok((layer, io))
}
