use chrono::{DateTime, Utc};

/// Status of a mead batch
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeadStatus {
    Planning,
    Primary,
    Secondary,
    Aging,
    Bottled,
    Finished,
}

impl MeadStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MeadStatus::Planning => "Planning",
            MeadStatus::Primary => "Primary",
            MeadStatus::Secondary => "Secondary",
            MeadStatus::Aging => "Aging",
            MeadStatus::Bottled => "Bottled",
            MeadStatus::Finished => "Finished",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "planning" => MeadStatus::Planning,
            "primary" => MeadStatus::Primary,
            "secondary" => MeadStatus::Secondary,
            "aging" => MeadStatus::Aging,
            "bottled" => MeadStatus::Bottled,
            "finished" => MeadStatus::Finished,
            _ => MeadStatus::Planning,
        }
    }

    pub fn all() -> Vec<MeadStatus> {
        vec![
            MeadStatus::Planning,
            MeadStatus::Primary,
            MeadStatus::Secondary,
            MeadStatus::Aging,
            MeadStatus::Bottled,
            MeadStatus::Finished,
        ]
    }

    pub fn next(&self) -> Self {
        match self {
            MeadStatus::Planning => MeadStatus::Primary,
            MeadStatus::Primary => MeadStatus::Secondary,
            MeadStatus::Secondary => MeadStatus::Aging,
            MeadStatus::Aging => MeadStatus::Bottled,
            MeadStatus::Bottled => MeadStatus::Finished,
            MeadStatus::Finished => MeadStatus::Planning,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            MeadStatus::Planning => MeadStatus::Finished,
            MeadStatus::Primary => MeadStatus::Planning,
            MeadStatus::Secondary => MeadStatus::Primary,
            MeadStatus::Aging => MeadStatus::Secondary,
            MeadStatus::Bottled => MeadStatus::Aging,
            MeadStatus::Finished => MeadStatus::Bottled,
        }
    }
}

/// Main mead batch data
#[derive(Debug, Clone)]
pub struct Mead {
    pub id: i64,
    pub name: String,
    pub start_date: String,
    pub honey_type: String,
    pub honey_amount_lbs: f64,
    pub yeast_strain: String,
    pub target_abv: f64,
    pub starting_gravity: f64,
    pub current_gravity: f64,
    pub yan_required: f64,
    pub yan_added: f64,
    pub volume_gallons: f64,
    pub status: MeadStatus,
    pub notes: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for Mead {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            name: String::new(),
            start_date: now.format("%Y-%m-%d").to_string(),
            honey_type: String::new(),
            honey_amount_lbs: 0.0,
            yeast_strain: String::new(),
            target_abv: 14.0,
            starting_gravity: 1.100,
            current_gravity: 1.100,
            yan_required: 0.0,
            yan_added: 0.0,
            volume_gallons: 1.0,
            status: MeadStatus::Planning,
            notes: String::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

/// Type of ingredient added to mead
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngredientType {
    Fruit,
    Spice,
    Nutrient,
    Adjunct,
    Other,
}

impl IngredientType {
    pub fn as_str(&self) -> &'static str {
        match self {
            IngredientType::Fruit => "Fruit",
            IngredientType::Spice => "Spice",
            IngredientType::Nutrient => "Nutrient",
            IngredientType::Adjunct => "Adjunct",
            IngredientType::Other => "Other",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "fruit" => IngredientType::Fruit,
            "spice" => IngredientType::Spice,
            "nutrient" => IngredientType::Nutrient,
            "adjunct" => IngredientType::Adjunct,
            _ => IngredientType::Other,
        }
    }

    pub fn all() -> Vec<IngredientType> {
        vec![
            IngredientType::Fruit,
            IngredientType::Spice,
            IngredientType::Nutrient,
            IngredientType::Adjunct,
            IngredientType::Other,
        ]
    }
}

/// Ingredient added to a mead batch
#[derive(Debug, Clone)]
pub struct Ingredient {
    pub id: i64,
    pub mead_id: i64,
    pub ingredient_type: IngredientType,
    pub name: String,
    pub amount: f64,
    pub unit: String,
    pub added_date: String,
}

impl Default for Ingredient {
    fn default() -> Self {
        Self {
            id: 0,
            mead_id: 0,
            ingredient_type: IngredientType::Other,
            name: String::new(),
            amount: 0.0,
            unit: String::from("oz"),
            added_date: Utc::now().format("%Y-%m-%d").to_string(),
        }
    }
}

/// Log entry for tracking changes/events
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub id: i64,
    pub mead_id: i64,
    pub timestamp: DateTime<Utc>,
    pub entry_text: String,
}

impl Default for LogEntry {
    fn default() -> Self {
        Self {
            id: 0,
            mead_id: 0,
            timestamp: Utc::now(),
            entry_text: String::new(),
        }
    }
}

