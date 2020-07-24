CREATE TABLE schemas(
    id SERIAL NOT NULL,
    data JSONB NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE entries(
    id SERIAL NOT NULL,
    schema_id SERIAL NOT NULL,
    data JSONB NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (schema_id) REFERENCES schemas(id)
);
