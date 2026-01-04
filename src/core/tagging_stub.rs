pub(crate) trait TagWriter {}

pub(crate) struct TagWriterNoOp;
impl TagWriter for TagWriterNoOp {}

pub(crate) struct TagWriterImpl {}
impl TagWriter for TagWriterImpl {}
