pub(crate) trait TagWriter {}

pub(crate) struct TagWriterNoOp;
impl TagWriter for TagWriterNoOp {}
