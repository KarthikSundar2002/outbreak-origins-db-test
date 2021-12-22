CREATE TABLE levels(
    id INT PRIMARY KEY,
    money INT NOT NULL,
    score INT NOT NULL,
    sections VARCHAR(255) NULL,
    compliance INT NOT NULL,
    map_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL

);

CREATE TABLE events(
    id INT PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    reward DECIMAL(8, 2) NOT NULL,
    current_state INT NOT NULL,
    compliance_reward INT NOT NULL,
    infection INT NOT NULL,
    next_event_time INT NOT NULL
);

CREATE TABLE section_stats(
    id INT PRIMARY KEY NOT NULL,
    simulation_fields DECIMAL(8, 2) NOT NULL,
    no_alive INT NOT NULL,
    no_dead INT NOT NULL,
    no_recovering INT NOT NULL,
    healthcare_level TEXT NOT NULL,
    travel_restrictions INT NOT NULL,
    event_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    CONSTRAINT fk_event
        FOREIGN KEY(event_id)
            REFERENCES events(id)
);