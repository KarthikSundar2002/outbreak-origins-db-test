#[derive(Queryable)]
pub struct Level {
    pub id: i32,
    pub money: i32,
    pub score: i32,
    pub sections: String,
    pub compliance: i32,
    pub map_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable)]
pub struct section_stat {
    pub id: i32,
    pub simulation_fields: f64,
    pub no_alive: i32,
    pub no_dead: i32,
    pub no_recovering: i32,
    pub healthcare_level: String,
    pub travel_restrictions: i32,
    pub event_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Queryable)]
pub struct event {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub reward: i32,
    pub current_state: i32,
    pub compliance_reward: i32,
    pub infection: i32,
    pub next_event_time: i32,
}
