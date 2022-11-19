use serde::Deserialize;
use crate::{
    components::json::JSON,
    context::Context,
};


pub struct Request {
    // pub headers: Vec<HeaderOfReq>,
    pub(crate) body:    Option<JSON>,
}
impl<'d> Request {
    pub fn get_body<D: Deserialize<'d>>(&'d self) -> Context<Option<D>> {
        let Some(json) = &self.body else {
            return Ok(None)
        };
        let body = json.to_struct()?;
        Ok(Some(body))
    }
}