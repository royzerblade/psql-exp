-- Create the spells table
CREATE TABLE spells (
    id SERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    saving_throw_type TEXT NOT NULL,
    damage_type TEXT NOT NULL,
    damage TEXT NOT NULL,
    shape TEXT NOT NULL,
    area_affected TEXT NOT NULL,
    range TEXT NOT NULL,
    duration TEXT NOT NULL,
    is_concentration BOOLEAN NOT NULL
);
