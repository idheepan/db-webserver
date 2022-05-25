table! {
    data_en (id) {
        id -> Int4,
        ts -> Timestamptz,
        sensor -> Int4,
        temperature -> Float4,
        rhumidity -> Float4,
        enthalpy -> Float4,
    }
}
