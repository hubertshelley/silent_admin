use sea_orm::prelude::Json;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};

use crate::{BpmError, Result};

pub fn get_active_model_from_value<T, V>(model: &mut T, value: Json) -> Result<()>
where
    T: ActiveModelTrait,
    <<T as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<T>,
    for<'de> <<T as ActiveModelTrait>::Entity as EntityTrait>::Model: serde::de::Deserialize<'de>,
{
    model
        .set_from_json(value)
        .map_err(|e| BpmError::num_msg(422, e.to_string()))
}
