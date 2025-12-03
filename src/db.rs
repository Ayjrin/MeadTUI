use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, params};
use std::path::PathBuf;

use crate::models::{Ingredient, IngredientType, LogEntry, Mead, MeadStatus};

/// Database handler for mead tracking
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create or open the database
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path();
        let conn = Connection::open(&db_path)?;
        let db = Self { conn };
        db.init_tables()?;
        Ok(db)
    }

    /// Get the database file path
    fn get_db_path() -> PathBuf {
        let mut path = dirs_next().unwrap_or_else(|| PathBuf::from("."));
        path.push("mead_tracker.db");
        path
    }

    /// Initialize database tables
    fn init_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS meads (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                start_date TEXT NOT NULL,
                honey_type TEXT NOT NULL,
                honey_amount_lbs REAL NOT NULL,
                yeast_strain TEXT NOT NULL,
                target_abv REAL NOT NULL,
                starting_gravity REAL NOT NULL,
                current_gravity REAL NOT NULL,
                yan_required REAL NOT NULL,
                yan_added REAL NOT NULL,
                volume_gallons REAL NOT NULL,
                status TEXT NOT NULL,
                notes TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS ingredients (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                mead_id INTEGER NOT NULL,
                ingredient_type TEXT NOT NULL,
                name TEXT NOT NULL,
                amount REAL NOT NULL,
                unit TEXT NOT NULL,
                added_date TEXT NOT NULL,
                FOREIGN KEY (mead_id) REFERENCES meads(id) ON DELETE CASCADE
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS log_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                mead_id INTEGER NOT NULL,
                timestamp TEXT NOT NULL,
                entry_text TEXT NOT NULL,
                FOREIGN KEY (mead_id) REFERENCES meads(id) ON DELETE CASCADE
            )",
            [],
        )?;

        Ok(())
    }

    // ==================== MEAD CRUD ====================

    /// Create a new mead
    pub fn create_mead(&self, mead: &Mead) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO meads (name, start_date, honey_type, honey_amount_lbs, yeast_strain,
                target_abv, starting_gravity, current_gravity, yan_required, yan_added,
                volume_gallons, status, notes, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                mead.name,
                mead.start_date,
                mead.honey_type,
                mead.honey_amount_lbs,
                mead.yeast_strain,
                mead.target_abv,
                mead.starting_gravity,
                mead.current_gravity,
                mead.yan_required,
                mead.yan_added,
                mead.volume_gallons,
                mead.status.as_str(),
                mead.notes,
                mead.created_at.to_rfc3339(),
                mead.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Get all meads
    pub fn get_all_meads(&self) -> Result<Vec<Mead>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, start_date, honey_type, honey_amount_lbs, yeast_strain,
                target_abv, starting_gravity, current_gravity, yan_required, yan_added,
                volume_gallons, status, notes, created_at, updated_at
            FROM meads ORDER BY created_at DESC"
        )?;

        let meads = stmt.query_map([], |row| {
            Ok(Mead {
                id: row.get(0)?,
                name: row.get(1)?,
                start_date: row.get(2)?,
                honey_type: row.get(3)?,
                honey_amount_lbs: row.get(4)?,
                yeast_strain: row.get(5)?,
                target_abv: row.get(6)?,
                starting_gravity: row.get(7)?,
                current_gravity: row.get(8)?,
                yan_required: row.get(9)?,
                yan_added: row.get(10)?,
                volume_gallons: row.get(11)?,
                status: MeadStatus::from_str(&row.get::<_, String>(12)?),
                notes: row.get(13)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(14)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(15)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?;

        meads.collect()
    }

    /// Get a mead by ID
    pub fn get_mead(&self, id: i64) -> Result<Option<Mead>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, start_date, honey_type, honey_amount_lbs, yeast_strain,
                target_abv, starting_gravity, current_gravity, yan_required, yan_added,
                volume_gallons, status, notes, created_at, updated_at
            FROM meads WHERE id = ?1"
        )?;

        let mut rows = stmt.query(params![id])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(Mead {
                id: row.get(0)?,
                name: row.get(1)?,
                start_date: row.get(2)?,
                honey_type: row.get(3)?,
                honey_amount_lbs: row.get(4)?,
                yeast_strain: row.get(5)?,
                target_abv: row.get(6)?,
                starting_gravity: row.get(7)?,
                current_gravity: row.get(8)?,
                yan_required: row.get(9)?,
                yan_added: row.get(10)?,
                volume_gallons: row.get(11)?,
                status: MeadStatus::from_str(&row.get::<_, String>(12)?),
                notes: row.get(13)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(14)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(15)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            }))
        } else {
            Ok(None)
        }
    }

    /// Update a mead
    pub fn update_mead(&self, mead: &Mead) -> Result<()> {
        self.conn.execute(
            "UPDATE meads SET
                name = ?1, start_date = ?2, honey_type = ?3, honey_amount_lbs = ?4,
                yeast_strain = ?5, target_abv = ?6, starting_gravity = ?7, current_gravity = ?8,
                yan_required = ?9, yan_added = ?10, volume_gallons = ?11, status = ?12,
                notes = ?13, updated_at = ?14
            WHERE id = ?15",
            params![
                mead.name,
                mead.start_date,
                mead.honey_type,
                mead.honey_amount_lbs,
                mead.yeast_strain,
                mead.target_abv,
                mead.starting_gravity,
                mead.current_gravity,
                mead.yan_required,
                mead.yan_added,
                mead.volume_gallons,
                mead.status.as_str(),
                mead.notes,
                Utc::now().to_rfc3339(),
                mead.id,
            ],
        )?;
        Ok(())
    }

    /// Delete a mead
    pub fn delete_mead(&self, id: i64) -> Result<()> {
        // Delete related entries first
        self.conn.execute("DELETE FROM ingredients WHERE mead_id = ?1", params![id])?;
        self.conn.execute("DELETE FROM log_entries WHERE mead_id = ?1", params![id])?;
        self.conn.execute("DELETE FROM meads WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ==================== INGREDIENT CRUD ====================

    /// Add an ingredient to a mead
    pub fn create_ingredient(&self, ingredient: &Ingredient) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO ingredients (mead_id, ingredient_type, name, amount, unit, added_date)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                ingredient.mead_id,
                ingredient.ingredient_type.as_str(),
                ingredient.name,
                ingredient.amount,
                ingredient.unit,
                ingredient.added_date,
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Get all ingredients for a mead
    pub fn get_ingredients(&self, mead_id: i64) -> Result<Vec<Ingredient>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, mead_id, ingredient_type, name, amount, unit, added_date
            FROM ingredients WHERE mead_id = ?1 ORDER BY added_date DESC"
        )?;

        let ingredients = stmt.query_map(params![mead_id], |row| {
            Ok(Ingredient {
                id: row.get(0)?,
                mead_id: row.get(1)?,
                ingredient_type: IngredientType::from_str(&row.get::<_, String>(2)?),
                name: row.get(3)?,
                amount: row.get(4)?,
                unit: row.get(5)?,
                added_date: row.get(6)?,
            })
        })?;

        ingredients.collect()
    }

    /// Delete an ingredient
    pub fn delete_ingredient(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM ingredients WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ==================== LOG ENTRY CRUD ====================

    /// Add a log entry to a mead
    pub fn create_log_entry(&self, entry: &LogEntry) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO log_entries (mead_id, timestamp, entry_text)
            VALUES (?1, ?2, ?3)",
            params![
                entry.mead_id,
                entry.timestamp.to_rfc3339(),
                entry.entry_text,
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Get all log entries for a mead
    pub fn get_log_entries(&self, mead_id: i64) -> Result<Vec<LogEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, mead_id, timestamp, entry_text
            FROM log_entries WHERE mead_id = ?1 ORDER BY timestamp DESC"
        )?;

        let entries = stmt.query_map(params![mead_id], |row| {
            Ok(LogEntry {
                id: row.get(0)?,
                mead_id: row.get(1)?,
                timestamp: DateTime::parse_from_rfc3339(&row.get::<_, String>(2)?)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                entry_text: row.get(3)?,
            })
        })?;

        entries.collect()
    }

    /// Delete a log entry
    pub fn delete_log_entry(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM log_entries WHERE id = ?1", params![id])?;
        Ok(())
    }
}

/// Get the data directory for the application
fn dirs_next() -> Option<PathBuf> {
    // Try to get the user's data directory, fall back to current directory
    std::env::var("HOME")
        .ok()
        .map(|home| {
            let mut path = PathBuf::from(home);
            path.push(".local");
            path.push("share");
            path.push("mead_tracker");
            // Create directory if it doesn't exist
            let _ = std::fs::create_dir_all(&path);
            path
        })
}

