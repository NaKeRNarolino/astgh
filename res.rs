/// This component can be specified as a Boolean. If this component is omitted, the default value for this component is true, which will give your block the default values for its parameters (a collision box the size/shape of a regular block). 
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CollisionBox {
   pub origin: Vec<i32>,
   pub size: Vec<i32>,
}

 /// Describes the destructible by explosion properties for this block. If set to true, the block will have the default explosion resistance. If set to false, this block is indestructible by explosion. If the component is omitted, the block will have the default explosion resistance. 
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DestructibleByExplosion {
   pub explosion_resistance: i32,
}

 /// Describes the destructible by mining properties for this block. If set to true, the block will take the default number of seconds to destroy. If set to false, this block is indestructible by mining. If the component is omitted, the block will take the default number of seconds to destroy. 
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DestructibleByMining {
   pub seconds_to_destroy: i32,
}

 /// Describes the flammable properties for this block. If set to true, default values are used. If set to false, or if this component is omitted, the block will not be able to catch on fire naturally from neighbors, but it can still be directly ignited. 
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Flammable {
   pub catch_chance_modifier: i32,
   pub destroy_chance_modifier: i32,
}

 