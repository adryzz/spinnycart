use std::ops::Index;

use hashbrown::HashMap;
use uuid::Uuid;

///
pub struct HashQueue<V>
where
    V: Clone,
{
    map: HashMap<Uuid, V>,
}
