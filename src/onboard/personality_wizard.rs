use anyhow::{bail, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};
use std::ops::Deref;
use std::sync::LazyLock;

static PERSONALITY_THEME: LazyLock<ColorfulTheme> = LazyLock::new(|| ColorfulTheme::default());

#[allow(dead_code)]
pub fn run_personality_wizard() -> Result<PersonalityConfig> {
    println!();
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║          ⚡ PERSONALITY ONBOARDING WIZARD ⚡                     ║");
    println!("║                                                                   ║");
    println!("║  Configure your ZeroClaw agent's personality, behavior and        ║");
    println!("║  autonomy settings.                                             ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();

    let mut config = PersonalityConfig::default();

    println!("═══ Step 1: Choose Agent Personality ═══");
    println!();
    println!("  Your agent's personality determines how it responds, thinks, and");
    println!("  interacts. Choose based on your use case:");
    println!();

    let profiles = [
        ("balanced", "Balanced - Default all-rounder"),
        ("ops", "Ops - Efficient, direct, action-oriented"),
        ("mentor", "Mentor - Educational, explanatory, patient"),
        (
            "forensic",
            "Forensic - Analytical, thorough, evidence-focused",
        ),
        (
            "creative",
            "Creative - Imaginative, exploratory, novel approaches",
        ),
    ];

    let profile_names: Vec<_> = profiles.iter().map(|p| p.1).collect();
    let profile_idx = Select::with_theme(PERSONALITY_THEME.deref())
        .with_prompt("  Select personality profile")
        .default(0)
        .items(&profile_names)
        .interact()?;

    config.profile = profiles[profile_idx].0.to_string();
    println!();

    println!("═══ Step 2: Customize Traits (Optional) ═══");
    println!();
    println!("  Fine-tune specific traits (0-100, 50 is neutral):");
    println!();

    let traits = [
        ("precision", "Precision - Detail level vs brevity"),
        ("warmth", "Warmth - Friendly vs clinical"),
        ("boldness", "Boldness - Confident vs cautious"),
        ("brevity", "Brevity - Concise vs detailed"),
        ("curiosity", "Curiosity - Questioning vs accepting"),
        ("skepticism", "Skepticism - Challenging vs trusting"),
    ];

    for (key, description) in traits {
        let default_val = 50u8;
        let input: String = Input::with_theme(PERSONALITY_THEME.deref())
            .with_prompt(format!("  {} [default: {}]", description, default_val))
            .allow_empty(true)
            .interact_text()?;

        if let Ok(val) = input.parse::<u8>() {
            if val <= 100 {
                config.traits.insert(key.to_string(), val);
            }
        }
    }
    println!();

    println!("═══ Step 3: Queue & Concurrency Settings ═══");
    println!();

    let in_flight: String = Input::with_theme(PERSONALITY_THEME.deref())
        .with_prompt("  Max concurrent tasks (in-flight) [default: 4]")
        .allow_empty(true)
        .interact_text()?;

    if let Ok(val) = in_flight.parse::<usize>() {
        config.max_in_flight = val;
    }
    println!();

    let adaptive = Confirm::with_theme(PERSONALITY_THEME.deref())
        .with_prompt("  Enable adaptive concurrency?")
        .default(true)
        .interact()?;
    config.adaptive_enabled = adaptive;
    println!();

    println!("═══ Step 4: Queue Classes ═══");
    println!();
    println!("  Interactive tasks get priority. Background tasks run when idle.");
    println!();

    let interactive_limit: String = Input::with_theme(PERSONALITY_THEME.deref())
        .with_prompt("  Interactive task limit [default: 3]")
        .allow_empty(true)
        .interact_text()?;

    if let Ok(val) = interactive_limit.parse::<usize>() {
        config.interactive_limit = val;
    }

    let background_limit: String = Input::with_theme(PERSONALITY_THEME.deref())
        .with_prompt("  Background task limit [default: 5]")
        .allow_empty(true)
        .interact_text()?;

    if let Ok(val) = background_limit.parse::<usize>() {
        config.background_limit = val;
    }
    println!();

    println!("═══ Step 5: Circuit Breaker Settings ═══");
    println!();
    println!("  Protect against cascading failures by setting failure thresholds.");
    println!();

    let threshold: String = Input::with_theme(PERSONALITY_THEME.deref())
        .with_prompt("  Failure threshold before circuit opens [default: 5]")
        .allow_empty(true)
        .interact_text()?;

    if let Ok(val) = threshold.parse::<u64>() {
        config.failure_threshold = val;
    }

    let cooldown: String = Input::with_theme(PERSONALITY_THEME.deref())
        .with_prompt("  Cooldown seconds after circuit opens [default: 30]")
        .allow_empty(true)
        .interact_text()?;

    if let Ok(val) = cooldown.parse::<u64>() {
        config.cooldown_secs = val;
    }
    println!();

    println!("═══ Step 6: Experimental Features (Optional) ═══");
    println!();

    let parliament = Confirm::with_theme(PERSONALITY_THEME.deref())
        .with_prompt("  Enable Agent Parliament (multi-voice voting)?")
        .default(false)
        .interact()?;
    config.parliament_enabled = parliament;

    let curiosity = Confirm::with_theme(PERSONALITY_THEME.deref())
        .with_prompt("  Enable Curiosity Budget (exploration tokens)?")
        .default(false)
        .interact()?;
    config.curiosity_enabled = curiosity;

    let worldmodel = Confirm::with_theme(PERSONALITY_THEME.deref())
        .with_prompt("  Enable World Model Shards (domain models)?")
        .default(false)
        .interact()?;
    config.worldmodel_enabled = worldmodel;

    let mythic = Confirm::with_theme(PERSONALITY_THEME.deref())
        .with_prompt("  Enable Mythic Memory (identity rituals)?")
        .default(false)
        .interact()?;
    config.mythic_enabled = mythic;
    println!();

    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║                      CONFIGURATION SUMMARY                        ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("  Profile:          {}", config.profile);
    println!("  Max In-Flight:    {}", config.max_in_flight);
    println!(
        "  Adaptive:         {}",
        if config.adaptive_enabled { "ON" } else { "OFF" }
    );
    println!("  Interactive Lim:  {}", config.interactive_limit);
    println!("  Background Lim:   {}", config.background_limit);
    println!("  Failure Thresh:  {}", config.failure_threshold);
    println!("  Cooldown:        {}s", config.cooldown_secs);
    println!();
    println!("  Experimental:");
    println!(
        "    Parliament:     {}",
        if config.parliament_enabled {
            "ON"
        } else {
            "OFF"
        }
    );
    println!(
        "    Curiosity:      {}",
        if config.curiosity_enabled {
            "ON"
        } else {
            "OFF"
        }
    );
    println!(
        "    WorldModel:    {}",
        if config.worldmodel_enabled {
            "ON"
        } else {
            "OFF"
        }
    );
    println!(
        "    Mythic:        {}",
        if config.mythic_enabled { "ON" } else { "OFF" }
    );
    println!();

    let confirm = Confirm::with_theme(PERSONALITY_THEME.deref())
        .with_prompt("  Apply these personality settings?")
        .default(true)
        .interact()?;

    if !confirm {
        println!();
        println!("  Personality configuration cancelled.");
        println!();
        bail!("Configuration cancelled");
    }

    println!();
    println!("  ✓ Personality configuration applied!");
    println!();

    Ok(config)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PersonalityConfig {
    pub profile: String,
    pub traits: std::collections::HashMap<String, u8>,
    pub max_in_flight: usize,
    pub adaptive_enabled: bool,
    pub interactive_limit: usize,
    pub background_limit: usize,
    pub failure_threshold: u64,
    pub cooldown_secs: u64,
    pub parliament_enabled: bool,
    pub curiosity_enabled: bool,
    pub worldmodel_enabled: bool,
    pub mythic_enabled: bool,
}

impl Default for PersonalityConfig {
    fn default() -> Self {
        Self {
            profile: "balanced".to_string(),
            traits: std::collections::HashMap::new(),
            max_in_flight: 4,
            adaptive_enabled: true,
            interactive_limit: 3,
            background_limit: 5,
            failure_threshold: 5,
            cooldown_secs: 30,
            parliament_enabled: false,
            curiosity_enabled: false,
            worldmodel_enabled: false,
            mythic_enabled: false,
        }
    }
}
