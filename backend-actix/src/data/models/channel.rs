pub struct Channel{
    id: String,
    name: String,
    description: String,
    is_public: bool,
    owner_id: String,
    members: Vec<String>,
    messages: Vec<String>,
    created_at: String,
    updated_at: String
}