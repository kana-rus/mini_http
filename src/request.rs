use serde::Deserialize;
use crate::{
    components::json::JSON,
    context::Context, Response,
};


pub struct Request<'param> {
    // pub headers: Vec<HeaderOfReq>,
    pub param:       Option<&'param str>, // PathParam<'param>,
    pub(crate) body: Option<JSON>,
}
// enum PathParam<'param> {
//     Int(isize),
//     Str(&'param str),
// }

impl<'d, 'param> Request<'param> {
    pub fn get_body<D: Deserialize<'d>>(&'d self) -> Context<D> {
        let json = self.body.as_ref().ok_or_else(|| Response::BadRequest("expected request body"))?;
        let json_struct = json.to_struct()?;
        Ok(json_struct)
    }
}