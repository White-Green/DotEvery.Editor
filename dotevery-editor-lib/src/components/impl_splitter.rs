pub(crate) trait ImplSplitter<ENUM> {
    const VALUE: ENUM;
    type Next: ImplSplitter<ENUM>;
}