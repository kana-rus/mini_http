use serde::Deserialize;
use crate::{
    components::json::JSON,
    context::Context,
};


pub struct Request<'str> {
    // pub headers: Vec<HeaderOfReq>,
    pub param:       Option<&'str str>, // PathParam<'str>,
    pub(crate) body: Option<JSON>,
}
// enum PathParam<'str> {
//     Int(isize),
//     Str(&'str str),
// }

impl<'d, 'str> Request<'str> {
    pub fn get_body<D: Deserialize<'d>>(&'d self) -> Context<Option<D>> {
        let Some(json) = &self.body else {
            return Ok(None)
        };
        let body = json.to_struct()?;
        Ok(Some(body))
    }
}