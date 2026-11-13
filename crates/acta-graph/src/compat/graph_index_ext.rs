//! Extension trait for  providing entity removal functionality.
use crate::types::EntityId;

pub trait GraphIndexExt: acta_ports::GraphIndex {
    fn remove_entity<'a>(
        &'a self,
        tx: acta_types::TxId,
        entity: EntityId,
    ) -> acta_ports::BoxFuture<'a, acta_types::Result<()>>;
}
