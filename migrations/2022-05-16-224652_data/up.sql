CREATE TABLE public.data_en
(
    id serial PRIMARY KEY NOT NULL,
    ts timestamp with time zone NOT NULL,
    sensor integer NOT NULL,
    temperature real NOT NULL,
    rhumidity real NOT NULL,
    enthalpy real NOT NULL
)