// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Introspection {
    pub reflection_id: String,
    pub mode: IntrospectionMode,
    pub depth: u8,
    pub findings: Vec<String>,
    pub questions_raised: Vec<String>,
    pub resolution_status: ResolutionStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntrospectionMode {
    SelfAnalysis,
    MetaCognition,
    ValueAlignment,
    CapabilityAssessment,
    Existential,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResolutionStatus {
    Unresolved,
    Resolved,
    NeedsFurtherInquiry,
    AcceptedAmbiguity,
}

impl Introspection {
    pub fn new<S: Into<String>>(reflection_id: S, mode: IntrospectionMode) -> Self {
        Self {
            reflection_id: reflection_id.into(),
            mode,
            depth: 1,
            findings: Vec::new(),
            questions_raised: Vec::new(),
            resolution_status: ResolutionStatus::Unresolved,
        }
    }

    pub fn with_depth(mut self, depth: u8) -> Self {
        self.depth = depth.min(10);
        self
    }

    pub fn add_finding<S: Into<String>>(&mut self, finding: S) {
        self.findings.push(finding.into());
    }

    pub fn add_question<S: Into<String>>(&mut self, question: S) {
        self.questions_raised.push(question.into());
    }

    pub fn resolve(mut self) -> Self {
        self.resolution_status = ResolutionStatus::Resolved;
        self
    }

    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    pub fn question_count(&self) -> usize {
        self.questions_raised.len()
    }

    pub fn is_resolved(&self) -> bool {
        matches!(self.resolution_status, ResolutionStatus::Resolved)
    }
}

impl fmt::Display for Introspection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Introspection {} ({:?}, depth {})", self.reflection_id, self.mode, self.depth)
    }
}

impl fmt::Display for IntrospectionMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IntrospectionMode::SelfAnalysis => write!(f, "self-analysis"),
            IntrospectionMode::MetaCognition => write!(f, "meta-cognition"),
            IntrospectionMode::ValueAlignment => write!(f, "value-alignment"),
            IntrospectionMode::CapabilityAssessment => write!(f, "capability-assessment"),
            IntrospectionMode::Existential => write!(f, "existential"),
        }
    }
}

impl fmt::Display for ResolutionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolutionStatus::Unresolved => write!(f, "unresolved"),
            ResolutionStatus::Resolved => write!(f, "resolved"),
            ResolutionStatus::NeedsFurtherInquiry => write!(f, "needs further inquiry"),
            ResolutionStatus::AcceptedAmbiguity => write!(f, "accepted ambiguity"),
        }
    }
}

