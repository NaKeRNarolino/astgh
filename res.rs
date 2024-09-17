/// This component can be specified as a Boolean. If this component is omitted, the default value for this component is true, which will give your block the default values for its parameters (a collision box the size/shape of a regular block).
#[derive(Clone, Debug, Serialize, Deserialize)]
struct CollisionBoxBlockComponent {
    origin: Vec<f64>,
    size: Vec<f64>,
}

/// Makes your block into a custom crafting table which enables the crafting table UI and the ability to craft recipes.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct CraftingTableBlockComponent {
    crafting_tags: Vec<String>,
    table_name: String,
}

/// Describes the destructible by explosion properties for this block. If set to true, the block will have the default explosion resistance. If set to false, this block is indestructible by explosion. If the component is omitted, the block will have the default explosion resistance.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DestructibleByExplosionBlockComponent {
    explosion_resistance: f64,
}

/// Describes the destructible by mining properties for this block. If set to true, the block will take the default number of seconds to destroy. If set to false, this block is indestructible by mining. If the component is omitted, the block will take the default number of seconds to destroy.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DestructibleByMiningBlockComponent {
    seconds_to_destroy: f64,
}

/// Describes the flammable properties for this block. If set to true, default values are used. If set to false, or if this component is omitted, the block will not be able to catch on fire naturally from neighbors, but it can still be directly ignited.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct FlammableBlockComponent {
    catch_chance_modifier: f64,
    destroy_chance_modifier: f64,
}

