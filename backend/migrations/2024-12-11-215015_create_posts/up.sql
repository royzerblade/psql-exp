-- Create ENUM type for Stats
CREATE TYPE stats_enum AS ENUM (
    'STR',
    'DEX',
    'CON',
    'INT',
    'WIS',
    'CHA'
);

-- Create ENUM type for Shapes
CREATE TYPE shapes_enum AS ENUM (
    'CONE',
    'CYLINDER',
    'BOX',
    'SPHERE',
    'OTHER'
);

-- Create the spells table
CREATE TABLE spells (
    id SERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    saving_throw_type stats_enum NOT NULL,
    damage_type TEXT NOT NULL,
    damage TEXT NOT NULL,
    shape shapes_enum NOT NULL,
    area_affected TEXT NOT NULL,
    range TEXT NOT NULL,
    duration TEXT NOT NULL,
    is_concentration BOOLEAN NOT NULL
);
