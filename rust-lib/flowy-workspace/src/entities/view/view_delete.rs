use crate::{
    entities::view::parser::ViewId,
    errors::{ErrorBuilder, WorkspaceError, WsErrCode},
};
use flowy_derive::ProtoBuf;
use std::convert::TryInto;

#[derive(Default, ProtoBuf)]
pub struct DeleteViewRequest {
    #[pb(index = 1)]
    view_id: String,
}

pub struct DeleteViewParams {
    pub(crate) view_id: String,
}

impl TryInto<DeleteViewParams> for DeleteViewRequest {
    type Error = WorkspaceError;

    fn try_into(self) -> Result<DeleteViewParams, Self::Error> {
        let view_id = ViewId::parse(self.view_id)
            .map_err(|e| ErrorBuilder::new(WsErrCode::ViewIdInvalid).msg(e).build())?
            .0;

        Ok(DeleteViewParams { view_id })
    }
}