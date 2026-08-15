use crate::content::{ContractChainDef, ContractChainObjectiveDef, ContractChainObjectiveKind};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractChainEventKind {
    Reached,
    Scanned,
    Acquired,
    Looted,
    Manufactured,
    Delivered,
    Mined,
    Destroyed,
    Salvaged,
    Protected,
    ResearchCompleted,
    Survived,
    Returned,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractChainEvent {
    pub id: String,
    pub kind: ContractChainEventKind,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub item: Option<String>,
    #[serde(default = "default_event_amount")]
    pub amount: u32,
    #[serde(default)]
    pub elapsed_seconds: f32,
}

fn default_event_amount() -> u32 {
    1
}

impl ContractChainEvent {
    pub fn new(id: impl Into<String>, kind: ContractChainEventKind) -> Self {
        Self {
            id: id.into(),
            kind,
            target: None,
            item: None,
            amount: 1,
            elapsed_seconds: 0.0,
        }
    }

    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn item(mut self, item: impl Into<String>, amount: u32) -> Self {
        self.item = Some(item.into());
        self.amount = amount;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractChainRuntimeStatus {
    #[default]
    Active,
    Blocked,
    Unavailable,
    Abandoned,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ObjectiveProgress {
    #[serde(default)]
    pub amount: u32,
    #[serde(default)]
    pub elapsed_seconds: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveContractChain {
    #[serde(alias = "quest_id")]
    pub contract_chain_id: String,
    pub stage_id: String,
    #[serde(default)]
    pub progress: HashMap<String, ObjectiveProgress>,
    #[serde(default)]
    pub processed_events: HashSet<String>,
    #[serde(default)]
    pub status: ContractChainRuntimeStatus,
    #[serde(default)]
    pub blocker: Option<String>,
}

impl ActiveContractChain {
    pub fn start(definition: &ContractChainDef) -> Option<Self> {
        Some(Self {
            contract_chain_id: definition.id.clone(),
            stage_id: definition.stages.first()?.id.clone(),
            progress: HashMap::new(),
            processed_events: HashSet::new(),
            status: ContractChainRuntimeStatus::Active,
            blocker: None,
        })
    }

    pub fn mark_unavailable(&mut self, reason: impl Into<String>) {
        self.status = ContractChainRuntimeStatus::Unavailable;
        self.blocker = Some(reason.into());
    }

    pub fn objective_progress(&self, stage_id: &str, objective_id: &str) -> ObjectiveProgress {
        self.progress
            .get(&progress_key(stage_id, objective_id))
            .cloned()
            .unwrap_or_default()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ContractChainUpdate {
    pub changed: bool,
    pub completed_stages: Vec<String>,
    pub completed_contract_chain: bool,
}

pub fn apply_event(
    definition: &ContractChainDef,
    active: &mut ActiveContractChain,
    event: &ContractChainEvent,
) -> ContractChainUpdate {
    let mut update = ContractChainUpdate::default();
    if active.status != ContractChainRuntimeStatus::Active
        || active.processed_events.contains(&event.id)
    {
        return update;
    }
    let Some(stage) = definition
        .stages
        .iter()
        .find(|stage| stage.id == active.stage_id)
    else {
        active.mark_unavailable("Current stage is missing from the enabled content pack");
        update.changed = true;
        return update;
    };

    let mut matched = false;
    for objective in &stage.objectives {
        if !event_matches(objective, definition, event) {
            continue;
        }
        matched = true;
        let progress = active
            .progress
            .entry(progress_key(&stage.id, &objective.id))
            .or_default();
        if objective.kind == ContractChainObjectiveKind::SurviveDuration {
            progress.elapsed_seconds = (progress.elapsed_seconds + event.elapsed_seconds.max(0.0))
                .min(objective.duration_seconds);
        } else {
            progress.amount = progress
                .amount
                .saturating_add(event.amount)
                .min(objective.amount);
        }
    }
    if !matched {
        return update;
    }
    active.processed_events.insert(event.id.clone());
    update.changed = true;

    if stage
        .objectives
        .iter()
        .all(|objective| objective.optional || objective_complete(active, &stage.id, objective))
    {
        update.completed_stages.push(stage.id.clone());
        if let Some(next) = &stage.next_stage {
            active.stage_id = next.clone();
        } else {
            update.completed_contract_chain = true;
        }
    }
    update
}

pub fn objective_complete(
    active: &ActiveContractChain,
    stage_id: &str,
    objective: &ContractChainObjectiveDef,
) -> bool {
    let progress = active.objective_progress(stage_id, &objective.id);
    if objective.kind == ContractChainObjectiveKind::SurviveDuration {
        progress.elapsed_seconds + f32::EPSILON >= objective.duration_seconds
    } else {
        progress.amount >= objective.amount
    }
}

fn progress_key(stage_id: &str, objective_id: &str) -> String {
    format!("{stage_id}:{objective_id}")
}

fn event_matches(
    objective: &ContractChainObjectiveDef,
    definition: &ContractChainDef,
    event: &ContractChainEvent,
) -> bool {
    let kind_matches = matches!(
        (objective.kind, event.kind),
        (
            ContractChainObjectiveKind::ReachLocation,
            ContractChainEventKind::Reached
        ) | (
            ContractChainObjectiveKind::ScanIdentify,
            ContractChainEventKind::Scanned
        ) | (
            ContractChainObjectiveKind::AcquireItem,
            ContractChainEventKind::Acquired
        ) | (
            ContractChainObjectiveKind::LootItem,
            ContractChainEventKind::Looted
        ) | (
            ContractChainObjectiveKind::ManufactureItem,
            ContractChainEventKind::Manufactured
        ) | (
            ContractChainObjectiveKind::DeliverItem,
            ContractChainEventKind::Delivered
        ) | (
            ContractChainObjectiveKind::MineQuota,
            ContractChainEventKind::Mined
        ) | (
            ContractChainObjectiveKind::DestroyTarget,
            ContractChainEventKind::Destroyed
        ) | (
            ContractChainObjectiveKind::ProtectTarget,
            ContractChainEventKind::Protected
        ) | (
            ContractChainObjectiveKind::CompleteResearch,
            ContractChainEventKind::ResearchCompleted
        ) | (
            ContractChainObjectiveKind::SurviveDuration,
            ContractChainEventKind::Survived
        ) | (
            ContractChainObjectiveKind::ReturnOrigin,
            ContractChainEventKind::Returned
        )
    );
    if !kind_matches {
        return false;
    }
    let target_matches = match objective.kind {
        ContractChainObjectiveKind::ReturnOrigin => {
            event.target.as_deref() == Some(&definition.origin_station)
        }
        _ => objective
            .target
            .as_ref()
            .is_none_or(|target| event.target.as_ref() == Some(target)),
    };
    let item_matches = objective
        .item
        .as_ref()
        .is_none_or(|item| event.item.as_ref() == Some(item));
    target_matches && item_matches
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{ContractChainRewardDef, ContractChainStageDef};

    fn definition() -> ContractChainDef {
        ContractChainDef {
            id: "core:test".to_string(),
            name: "Test".to_string(),
            summary: "Test chain".to_string(),
            origin_station: "core:origin".to_string(),
            origin_service: "core:desk".to_string(),
            stages: vec![
                ContractChainStageDef {
                    id: "scan".to_string(),
                    name: "Scan".to_string(),
                    description: "Scan the target".to_string(),
                    objectives: vec![ContractChainObjectiveDef {
                        id: "identify".to_string(),
                        kind: ContractChainObjectiveKind::ScanIdentify,
                        target: Some("core:target".to_string()),
                        item: None,
                        amount: 1,
                        duration_seconds: 0.0,
                        optional: false,
                    }],
                    next_stage: Some("return".to_string()),
                },
                ContractChainStageDef {
                    id: "return".to_string(),
                    name: "Return".to_string(),
                    description: "Return home".to_string(),
                    objectives: vec![ContractChainObjectiveDef {
                        id: "home".to_string(),
                        kind: ContractChainObjectiveKind::ReturnOrigin,
                        target: None,
                        item: None,
                        amount: 1,
                        duration_seconds: 0.0,
                        optional: false,
                    }],
                    next_stage: None,
                },
            ],
            rewards: vec![ContractChainRewardDef {
                kind: "credits".to_string(),
                target: None,
                amount: 10,
            }],
        }
    }

    #[test]
    fn matching_events_advance_sequential_stages_and_deduplicate() {
        let definition = definition();
        let mut active = ActiveContractChain::start(&definition).unwrap();
        let scan = ContractChainEvent::new("scan:1", ContractChainEventKind::Scanned)
            .target("core:target");
        let update = apply_event(&definition, &mut active, &scan);
        assert_eq!(update.completed_stages, ["scan"]);
        assert_eq!(active.stage_id, "return");
        assert!(!apply_event(&definition, &mut active, &scan).changed);

        let returned = ContractChainEvent::new("return:1", ContractChainEventKind::Returned)
            .target("core:origin");
        assert!(apply_event(&definition, &mut active, &returned).completed_contract_chain);
    }

    #[test]
    fn irrelevant_events_remain_available_for_later_stages() {
        let definition = definition();
        let mut active = ActiveContractChain::start(&definition).unwrap();
        let returned = ContractChainEvent::new("return:1", ContractChainEventKind::Returned)
            .target("core:origin");
        assert!(!apply_event(&definition, &mut active, &returned).changed);
        assert!(!active.processed_events.contains("return:1"));
    }

    #[test]
    fn loot_objectives_require_matching_source_and_loot_provenance() {
        let mut definition = definition();
        definition.stages[0].objectives[0] = ContractChainObjectiveDef {
            id: "archive".to_string(),
            kind: ContractChainObjectiveKind::LootItem,
            target: Some("core:wreck".to_string()),
            item: Some("core:archive".to_string()),
            amount: 1,
            duration_seconds: 0.0,
            optional: false,
        };
        let mut active = ActiveContractChain::start(&definition).unwrap();
        let acquired = ContractChainEvent::new("buy:1", ContractChainEventKind::Acquired)
            .target("core:wreck")
            .item("core:archive", 1);
        let wrong_wreck = ContractChainEvent::new("loot:1", ContractChainEventKind::Looted)
            .target("core:other_wreck")
            .item("core:archive", 1);
        assert!(!apply_event(&definition, &mut active, &acquired).changed);
        assert!(!apply_event(&definition, &mut active, &wrong_wreck).changed);

        let recovered = ContractChainEvent::new("loot:2", ContractChainEventKind::Looted)
            .target("core:wreck")
            .item("core:archive", 1);
        assert!(apply_event(&definition, &mut active, &recovered).changed);
    }
}
