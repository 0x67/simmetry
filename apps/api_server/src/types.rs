use socketioxide::{
    adapter::Emitter, layer::SocketIoLayer as SocketIoLayerBase, SocketIo as SocketIoBase,
};
use socketioxide_redis::{drivers::redis::RedisDriver, CustomRedisAdapter, RedisAdapterCtr};

pub type SocketIoLayer = SocketIoLayerBase<CustomRedisAdapter<Emitter, RedisDriver>>;
pub type SocketIo = SocketIoBase<CustomRedisAdapter<Emitter, RedisDriver>>;
pub type SocketIoRedisAdapter = RedisAdapterCtr<RedisDriver>;
