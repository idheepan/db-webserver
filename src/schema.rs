table! {
    data (id) {
        id -> Int4,
        ts -> Timestamptz,
        sensor -> Int4,
        temperature -> Float4,
        rhumidity -> Float4,
    }
}

// table! {
//     data_en (id) {
//         id -> Int4,
//         ts -> Timestamptz,
//         sensor -> Int4,
//         temperature -> Float4,
//         rhumidity -> Float4,
//         enthalpy -> Float4,
//     }
// }

// allow_tables_to_appear_in_same_query!(data, data_en,);
