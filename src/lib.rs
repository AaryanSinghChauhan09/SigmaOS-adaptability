//! SigmaOS-adaptability — comprehensive adaptability assessment component.
//!
//! Provides adaptability assessment for systems, applications, and organizations,
//! including flexibility metrics, adaptation strategies, and resilience analysis.

use std::collections::HashMap;

/// Adaptability domain
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdaptabilityDomain {
    /// Technical adaptability
    Technical,
    /// Organizational adaptability
    Organizational,
    /// Process adaptability
    Process,
    /// Technology adaptability
    Technology,
    /// Business adaptability
    Business,
    /// Environmental adaptability
    Environmental,
}

/// Adaptability level
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdaptabilityLevel {
    /// Low adaptability
    Low,
    /// Moderate adaptability
    Moderate,
    /// High adaptability
    High,
    /// Excellent adaptability
    Excellent,
}

/// Adaptability metric
#[derive(Debug, Clone)]
pub struct AdaptabilityMetric {
    /// Metric name
    pub name: String,
    /// Metric value
    pub value: f64,
    /// Metric weight
    pub weight: f64,
    /// Metric description
    pub description: String,
    /// Domain
    pub domain: AdaptabilityDomain,
}

/// Adaptability assessment
#[derive(Debug, Clone)]
pub struct AdaptabilityAssessment {
    /// Assessment ID
    pub id: String,
    /// Assessment name
    pub name: String,
    /// Subject type
    pub subject_type: SubjectType,
    /// Subject ID
    pub subject_id: String,
    /// Overall adaptability score
    pub overall_score: f64,
    /// Adaptability level
    pub level: AdaptabilityLevel,
    /// Domain scores
    pub domain_scores: HashMap<AdaptabilityDomain, f64>,
    /// Metrics
    pub metrics: Vec<AdaptabilityMetric>,
    /// Strengths
    pub strengths: Vec<String>,
    /// Weaknesses
    pub weaknesses: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Assessment date
    pub assessment_date: u64,
    /// Valid until
    pub valid_until: u64,
}

/// Subject type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubjectType {
    /// System
    System,
    /// Application
    Application,
    /// Organization
    Organization,
    /// Process
    Process,
    /// Technology
    Technology,
}

/// Adaptability strategy
#[derive(Debug, Clone)]
pub struct AdaptabilityStrategy {
    /// Strategy ID
    pub id: String,
    /// Strategy name
    pub name: String,
    /// Strategy description
    pub description: String,
    /// Domain
    pub domain: AdaptabilityDomain,
    /// Expected improvement
    pub expected_improvement: f64,
    /// Implementation cost
    pub implementation_cost: f64,
    /// Time to implement
    pub time_to_implement: u64,
    /// Priority
    pub priority: StrategyPriority,
}

/// Strategy priority
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StrategyPriority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// Adaptability tracker
#[derive(Debug, Clone)]
pub struct AdaptabilityTracker {
    /// Assessments
    pub assessments: HashMap<String, AdaptabilityAssessment>,
    /// Strategies
    pub strategies: HashMap<String, AdaptabilityStrategy>,
    /// Metrics history
    pub metrics_history: Vec<AdaptabilityMetric>,
    /// Statistics
    pub stats: AdaptabilityStatistics,
}

/// Adaptability statistics
#[derive(Debug, Clone, Default)]
pub struct AdaptabilityStatistics {
    /// Total assessments
    pub total_assessments: u64,
    /// Total strategies
    pub total_strategies: u64,
    /// Average adaptability score
    pub avg_adaptability_score: f64,
    /// High adaptability count
    pub high_adaptability_count: u64,
    /// Low adaptability count
    pub low_adaptability_count: u64,
    /// Last assessment date
    pub last_assessment_date: u64,
}

impl Default for AdaptabilityTracker {
    fn default() -> Self {
        Self {
            assessments: HashMap::new(),
            strategies: HashMap::new(),
            metrics_history: Vec::new(),
            stats: AdaptabilityStatistics::default(),
        }
    }
}

impl AdaptabilityTracker {
    /// Create new adaptability tracker
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize with default strategies
    pub fn initialize(&mut self) {
        // Add default adaptability strategies
        let strategies = vec![
            ("modular_architecture", "Modular Architecture", "Break down system into independent modules", AdaptabilityDomain::Technical, 15.0, 50.0, 90, StrategyPriority::High),
            ("api_first_design", "API-First Design", "Design APIs before implementation", AdaptabilityDomain::Technical, 20.0, 30.0, 60, StrategyPriority::High),
            ("microservices", "Microservices Architecture", "Decompose monolith into microservices", AdaptabilityDomain::Technical, 25.0, 80.0, 180, StrategyPriority::Medium),
            ("continuous_integration", "Continuous Integration", "Automate build and test process", AdaptabilityDomain::Process, 10.0, 20.0, 30, StrategyPriority::High),
            ("agile_methodology", "Agile Methodology", "Adopt agile development practices", AdaptabilityDomain::Organizational, 18.0, 10.0, 90, StrategyPriority::Medium),
            ("cloud_native", "Cloud-Native Architecture", "Design for cloud deployment", AdaptabilityDomain::Technology, 22.0, 60.0, 120, StrategyPriority::Medium),
        ];
        
        for (id, name, description, domain, improvement, cost, time, priority) in strategies {
            let strategy = AdaptabilityStrategy {
                id: id.to_string(),
                name: name.to_string(),
                description: description.to_string(),
                domain,
                expected_improvement: improvement,
                implementation_cost: cost,
                time_to_implement: time,
                priority,
            };
            self.strategies.insert(id.to_string(), strategy);
        }
        
        self.update_statistics();
    }
    
    /// Create adaptability assessment
    pub fn create_assessment(&mut self, name: String, subject_type: SubjectType, subject_id: String) -> String {
        let id = format!("assessment-{}", self.stats.total_assessments + 1);
        
        let assessment = AdaptabilityAssessment {
            id: id.clone(),
            name,
            subject_type,
            subject_id,
            overall_score: 0.0,
            level: AdaptabilityLevel::Low,
            domain_scores: HashMap::new(),
            metrics: vec![],
            strengths: vec![],
            weaknesses: vec![],
            recommendations: vec![],
            assessment_date: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            valid_until: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() + 2592000, // 30 days
        };
        
        self.assessments.insert(id.clone(), assessment);
        self.update_statistics();
        id
    }
    
    /// Add metric to assessment
    pub fn add_metric_to_assessment(&mut self, assessment_id: &str, metric: AdaptabilityMetric) -> bool {
        if let Some(assessment) = self.assessments.get_mut(assessment_id) {
            assessment.metrics.push(metric.clone());
            self.metrics_history.push(metric);
            self.update_assessment_score(assessment_id);
            return true;
        }
        false
    }
    
    /// Update assessment score
    fn update_assessment_score(&mut self, assessment_id: &str) {
        // First get the assessment data we need
        let assessment_data = if let Some(assessment) = self.assessments.get(assessment_id) {
            (assessment.metrics.clone(), assessment.domain_scores.clone())
        } else {
            return;
        };
        
        let (metrics, _) = assessment_data;
        
        if metrics.is_empty() {
            return;
        }
        
        // Calculate weighted average score
        let total_weight: f64 = metrics.iter().map(|m| m.weight).sum();
        if total_weight == 0.0 {
            return;
        }
        
        let weighted_sum: f64 = metrics.iter()
            .map(|m| m.value * m.weight)
            .sum();
        
        let overall_score = weighted_sum / total_weight;
        
        // Determine adaptability level
        let level = match overall_score {
            s if s < 30.0 => AdaptabilityLevel::Low,
            s if s < 60.0 => AdaptabilityLevel::Moderate,
            s if s < 80.0 => AdaptabilityLevel::High,
            _ => AdaptabilityLevel::Excellent,
        };
        
        // Calculate domain scores
        let mut domain_scores: HashMap<AdaptabilityDomain, f64> = HashMap::new();
        let mut domain_weights: HashMap<AdaptabilityDomain, f64> = HashMap::new();
        
        for metric in &metrics {
            *domain_scores.entry(metric.domain.clone()).or_insert(0.0) += metric.value * metric.weight;
            *domain_weights.entry(metric.domain.clone()).or_insert(0.0) += metric.weight;
        }
        
        for (domain, weight) in domain_weights {
            if let Some(score) = domain_scores.get_mut(&domain) {
                *score /= weight;
            }
        }
        
        // Generate recommendations based on domain scores
        let mut recommendations = vec![];
        for (domain, score) in &domain_scores {
            if *score < 50.0 {
                match domain {
                    AdaptabilityDomain::Technical => recommendations.push("Improve technical architecture and modularity".into()),
                    AdaptabilityDomain::Organizational => recommendations.push("Enhance organizational agility and decision-making".into()),
                    AdaptabilityDomain::Process => recommendations.push("Optimize processes and workflows for flexibility".into()),
                    AdaptabilityDomain::Technology => recommendations.push("Adopt modern technologies and platforms".into()),
                    AdaptabilityDomain::Business => recommendations.push("Develop business agility and market responsiveness".into()),
                    AdaptabilityDomain::Environmental => recommendations.push("Improve environmental adaptability and sustainability".into()),
                }
            }
        }
        
        if overall_score < 40.0 {
            recommendations.push("Consider implementing modular architecture".into());
            recommendations.push("Invest in automation and CI/CD".into());
        } else if overall_score < 70.0 {
            recommendations.push("Focus on continuous improvement".into());
            recommendations.push("Monitor adaptability metrics regularly".into());
        }
        
        // Now update the assessment
        if let Some(assessment) = self.assessments.get_mut(assessment_id) {
            assessment.overall_score = overall_score;
            assessment.level = level;
            assessment.domain_scores = domain_scores;
            assessment.recommendations = recommendations;
        }
    }
    
    /// Generate recommendations for assessment
    fn generate_recommendations(&self, assessment: &AdaptabilityAssessment) -> Vec<String> {
        let mut recommendations = vec![];
        
        // Check domain scores and add recommendations
        for (domain, score) in &assessment.domain_scores {
            if *score < 50.0 {
                match domain {
                    AdaptabilityDomain::Technical => recommendations.push("Improve technical architecture and modularity".into()),
                    AdaptabilityDomain::Organizational => recommendations.push("Enhance organizational agility and decision-making".into()),
                    AdaptabilityDomain::Process => recommendations.push("Optimize processes and workflows for flexibility".into()),
                    AdaptabilityDomain::Technology => recommendations.push("Adopt modern technologies and platforms".into()),
                    AdaptabilityDomain::Business => recommendations.push("Develop business agility and market responsiveness".into()),
                    AdaptabilityDomain::Environmental => recommendations.push("Improve environmental adaptability and sustainability".into()),
                }
            }
        }
        
        // Add general recommendations based on score
        if assessment.overall_score < 40.0 {
            recommendations.push("Consider implementing modular architecture".into());
            recommendations.push("Invest in automation and CI/CD".into());
        } else if assessment.overall_score < 70.0 {
            recommendations.push("Focus on continuous improvement".into());
            recommendations.push("Monitor adaptability metrics regularly".into());
        }
        
        recommendations
    }
    
    /// Get assessment
    pub fn get_assessment(&self, assessment_id: &str) -> Option<&AdaptabilityAssessment> {
        self.assessments.get(assessment_id)
    }
    
    /// Get all assessments
    pub fn get_all_assessments(&self) -> Vec<&AdaptabilityAssessment> {
        self.assessments.values().collect()
    }
    
    /// Get assessments by subject type
    pub fn get_assessments_by_subject_type(&self, subject_type: &SubjectType) -> Vec<&AdaptabilityAssessment> {
        self.assessments.values()
            .filter(|a| a.subject_type == *subject_type)
            .collect()
    }
    
    /// Get assessments by level
    pub fn get_assessments_by_level(&self, level: &AdaptabilityLevel) -> Vec<&AdaptabilityAssessment> {
        self.assessments.values()
            .filter(|a| a.level == *level)
            .collect()
    }
    
    /// Get assessment history for subject
    pub fn get_assessment_history(&self, subject_id: &str) -> Vec<&AdaptabilityAssessment> {
        let mut assessments: Vec<&AdaptabilityAssessment> = self.assessments.values()
            .filter(|a| a.subject_id == subject_id)
            .collect();
        assessments.sort_by(|a, b| b.assessment_date.cmp(&a.assessment_date));
        assessments
    }
    
    /// Add strategy
    pub fn add_strategy(&mut self, strategy: AdaptabilityStrategy) -> String {
        let id = strategy.id.clone();
        self.strategies.insert(id.clone(), strategy);
        self.update_statistics();
        id
    }
    
    /// Get strategy
    pub fn get_strategy(&self, strategy_id: &str) -> Option<&AdaptabilityStrategy> {
        self.strategies.get(strategy_id)
    }
    
    /// Get all strategies
    pub fn get_all_strategies(&self) -> Vec<&AdaptabilityStrategy> {
        self.strategies.values().collect()
    }
    
    /// Get strategies by domain
    pub fn get_strategies_by_domain(&self, domain: &AdaptabilityDomain) -> Vec<&AdaptabilityStrategy> {
        self.strategies.values()
            .filter(|s| s.domain == *domain)
            .collect()
    }
    
    /// Get strategies by priority
    pub fn get_strategies_by_priority(&self, priority: &StrategyPriority) -> Vec<&AdaptabilityStrategy> {
        self.strategies.values()
            .filter(|s| s.priority == *priority)
            .collect()
    }
    
    /// Search assessments
    pub fn search_assessments(&self, query: &str) -> Vec<&AdaptabilityAssessment> {
        let query_lower = query.to_lowercase();
        self.assessments.values()
            .filter(|a| {
                a.name.to_lowercase().contains(&query_lower) ||
                a.subject_id.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
    
    /// Calculate adaptability trend
    pub fn calculate_adaptability_trend(&self, subject_id: &str) -> f64 {
        let history = self.get_assessment_history(subject_id);
        if history.len() < 2 {
            return 0.0;
        }
        
        // history is sorted by date descending, so history[0] is latest
        let latest = history[0].overall_score;
        let previous = history[1].overall_score;
        
        // Return absolute difference for trend (positive = improvement, negative = decline)
        latest - previous
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.stats.total_assessments = self.assessments.len() as u64;
        self.stats.total_strategies = self.strategies.len() as u64;
        
        if !self.assessments.is_empty() {
            self.stats.avg_adaptability_score = self.assessments.values()
                .map(|a| a.overall_score)
                .sum::<f64>() / self.assessments.len() as f64;
        }
        
        self.stats.high_adaptability_count = self.assessments.values()
            .filter(|a| a.level == AdaptabilityLevel::High || a.level == AdaptabilityLevel::Excellent)
            .count() as u64;
        
        self.stats.low_adaptability_count = self.assessments.values()
            .filter(|a| a.level == AdaptabilityLevel::Low)
            .count() as u64;
        
        self.stats.last_assessment_date = self.assessments.values()
            .map(|a| a.assessment_date)
            .max()
            .unwrap_or(0);
    }
    
    /// Get adaptability statistics
    pub fn get_statistics(&self) -> &AdaptabilityStatistics {
        &self.stats
    }
    
    /// Export adaptability data
    pub fn export_data(&self) -> AdaptabilityExport {
        AdaptabilityExport {
            assessments: self.assessments.values().cloned().collect(),
            strategies: self.strategies.values().cloned().collect(),
            metrics_history: self.metrics_history.clone(),
        }
    }
    
    /// Import adaptability data
    pub fn import_data(&mut self, export: AdaptabilityExport) {
        for assessment in export.assessments {
            self.assessments.insert(assessment.id.clone(), assessment);
        }
        
        for strategy in export.strategies {
            self.strategies.insert(strategy.id.clone(), strategy);
        }
        
        self.metrics_history = export.metrics_history;
        self.update_statistics();
    }
}

/// Adaptability export data
#[derive(Debug, Clone)]
pub struct AdaptabilityExport {
    /// Assessments
    pub assessments: Vec<AdaptabilityAssessment>,
    /// Strategies
    pub strategies: Vec<AdaptabilityStrategy>,
    /// Metrics history
    pub metrics_history: Vec<AdaptabilityMetric>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_adaptability_domain() {
        let domains = vec![
            AdaptabilityDomain::Technical,
            AdaptabilityDomain::Organizational,
            AdaptabilityDomain::Process,
            AdaptabilityDomain::Technology,
            AdaptabilityDomain::Business,
            AdaptabilityDomain::Environmental,
        ];
        assert_eq!(domains.len(), 6);
    }
    
    #[test]
    fn test_adaptability_level() {
        let levels = vec![
            AdaptabilityLevel::Low,
            AdaptabilityLevel::Moderate,
            AdaptabilityLevel::High,
            AdaptabilityLevel::Excellent,
        ];
        assert_eq!(levels.len(), 4);
    }
    
    #[test]
    fn test_subject_type() {
        let types = vec![
            SubjectType::System,
            SubjectType::Application,
            SubjectType::Organization,
            SubjectType::Process,
            SubjectType::Technology,
        ];
        assert_eq!(types.len(), 5);
    }
    
    #[test]
    fn test_strategy_priority() {
        let priorities = vec![
            StrategyPriority::Low,
            StrategyPriority::Medium,
            StrategyPriority::High,
            StrategyPriority::Critical,
        ];
        assert_eq!(priorities.len(), 4);
    }
    
    #[test]
    fn test_tracker_creation() {
        let tracker = AdaptabilityTracker::new();
        assert_eq!(tracker.get_statistics().total_assessments, 0);
    }
    
    #[test]
    fn test_initialize() {
        let mut tracker = AdaptabilityTracker::new();
        tracker.initialize();
        
        assert!(tracker.get_statistics().total_strategies > 0);
        assert!(tracker.get_strategy("modular_architecture").is_some());
    }
    
    #[test]
    fn test_create_assessment() {
        let mut tracker = AdaptabilityTracker::new();
        
        let assessment_id = tracker.create_assessment(
            "Test Assessment".into(),
            SubjectType::System,
            "test-system".into()
        );
        
        assert!(tracker.get_assessment(&assessment_id).is_some());
        assert_eq!(tracker.get_statistics().total_assessments, 1);
    }
    
    #[test]
    fn test_add_metric_to_assessment() {
        let mut tracker = AdaptabilityTracker::new();
        
        let assessment_id = tracker.create_assessment(
            "Test Assessment".into(),
            SubjectType::System,
            "test-system".into()
        );
        
        let metric = AdaptabilityMetric {
            name: "Modularity".into(),
            value: 75.0,
            weight: 1.0,
            description: "System modularity score".into(),
            domain: AdaptabilityDomain::Technical,
        };
        
        assert!(tracker.add_metric_to_assessment(&assessment_id, metric));
        
        let assessment = tracker.get_assessment(&assessment_id).unwrap();
        assert_eq!(assessment.metrics.len(), 1);
        assert_eq!(assessment.overall_score, 75.0);
        assert_eq!(assessment.level, AdaptabilityLevel::High);
    }
    
    #[test]
    fn test_multiple_metrics() {
        let mut tracker = AdaptabilityTracker::new();
        
        let assessment_id = tracker.create_assessment(
            "Test Assessment".into(),
            SubjectType::System,
            "test-system".into()
        );
        
        // Add multiple metrics
        tracker.add_metric_to_assessment(&assessment_id, AdaptabilityMetric {
            name: "Modularity".into(),
            value: 80.0,
            weight: 2.0,
            description: "System modularity".into(),
            domain: AdaptabilityDomain::Technical,
        });
        
        tracker.add_metric_to_assessment(&assessment_id, AdaptabilityMetric {
            name: "Agility".into(),
            value: 60.0,
            weight: 1.0,
            description: "Organizational agility".into(),
            domain: AdaptabilityDomain::Organizational,
        });
        
        let assessment = tracker.get_assessment(&assessment_id).unwrap();
        // Weighted average: (80*2 + 60*1) / (2+1) = 220/3 = 73.33
        assert!(assessment.overall_score > 73.0 && assessment.overall_score < 74.0);
        assert_eq!(assessment.level, AdaptabilityLevel::High);
    }
    
    #[test]
    fn test_get_assessments_by_subject_type() {
        let mut tracker = AdaptabilityTracker::new();
        
        tracker.create_assessment("System 1".into(), SubjectType::System, "sys1".into());
        tracker.create_assessment("App 1".into(), SubjectType::Application, "app1".into());
        tracker.create_assessment("System 2".into(), SubjectType::System, "sys2".into());
        
        let systems = tracker.get_assessments_by_subject_type(&SubjectType::System);
        assert_eq!(systems.len(), 2);
    }
    
    #[test]
    fn test_get_assessments_by_level() {
        let mut tracker = AdaptabilityTracker::new();
        
        let assessment_id = tracker.create_assessment(
            "Test Assessment".into(),
            SubjectType::System,
            "test-system".into()
        );
        
        tracker.add_metric_to_assessment(&assessment_id, AdaptabilityMetric {
            name: "Modularity".into(),
            value: 75.0, // High level (60-80)
            weight: 1.0,
            description: "System modularity".into(),
            domain: AdaptabilityDomain::Technical,
        });
        
        let high_assessments = tracker.get_assessments_by_level(&AdaptabilityLevel::High);
        assert_eq!(high_assessments.len(), 1);
    }
    
    #[test]
    fn test_get_assessment_history() {
        let mut tracker = AdaptabilityTracker::new();
        
        // Create multiple assessments for same subject
        let assessment1 = tracker.create_assessment("Assessment 1".into(), SubjectType::System, "sys1".into());
        let assessment2 = tracker.create_assessment("Assessment 2".into(), SubjectType::System, "sys1".into());
        
        tracker.add_metric_to_assessment(&assessment1, AdaptabilityMetric {
            name: "Metric1".into(),
            value: 50.0,
            weight: 1.0,
            description: "Test".into(),
            domain: AdaptabilityDomain::Technical,
        });
        
        tracker.add_metric_to_assessment(&assessment2, AdaptabilityMetric {
            name: "Metric2".into(),
            value: 70.0,
            weight: 1.0,
            description: "Test".into(),
            domain: AdaptabilityDomain::Technical,
        });
        
        let history = tracker.get_assessment_history("sys1");
        assert_eq!(history.len(), 2);
        assert!(history[0].assessment_date >= history[1].assessment_date);
    }
    
    #[test]
    fn test_add_strategy() {
        let mut tracker = AdaptabilityTracker::new();
        tracker.initialize();
        
        let strategy = AdaptabilityStrategy {
            id: "test_strategy".into(),
            name: "Test Strategy".into(),
            description: "Test strategy description".into(),
            domain: AdaptabilityDomain::Technical,
            expected_improvement: 10.0,
            implementation_cost: 25.0,
            time_to_implement: 30,
            priority: StrategyPriority::High,
        };
        
        tracker.add_strategy(strategy);
        assert!(tracker.get_strategy("test_strategy").is_some());
        assert_eq!(tracker.get_statistics().total_strategies, 7); // 6 default + 1 new
    }
    
    #[test]
    fn test_get_strategies_by_domain() {
        let mut tracker = AdaptabilityTracker::new();
        tracker.initialize();
        
        let technical_strategies = tracker.get_strategies_by_domain(&AdaptabilityDomain::Technical);
        assert!(!technical_strategies.is_empty());
    }
    
    #[test]
    fn test_get_strategies_by_priority() {
        let mut tracker = AdaptabilityTracker::new();
        tracker.initialize();
        
        let high_priority = tracker.get_strategies_by_priority(&StrategyPriority::High);
        assert!(!high_priority.is_empty());
    }
    
    #[test]
    fn test_search_assessments() {
        let mut tracker = AdaptabilityTracker::new();
        
        tracker.create_assessment("System Assessment".into(), SubjectType::System, "sys1".into());
        tracker.create_assessment("App Assessment".into(), SubjectType::Application, "app1".into());
        
        let results = tracker.search_assessments("system");
        assert_eq!(results.len(), 1);
    }
    
    #[test]
    fn test_calculate_adaptability_trend() {
        let mut tracker = AdaptabilityTracker::new();
        
        let assessment1 = tracker.create_assessment("Assessment 1".into(), SubjectType::System, "sys1".into());
        let assessment2 = tracker.create_assessment("Assessment 2".into(), SubjectType::System, "sys1".into());
        
        tracker.add_metric_to_assessment(&assessment1, AdaptabilityMetric {
            name: "Metric".into(),
            value: 50.0,
            weight: 1.0,
            description: "Test".into(),
            domain: AdaptabilityDomain::Technical,
        });
        
        tracker.add_metric_to_assessment(&assessment2, AdaptabilityMetric {
            name: "Metric".into(),
            value: 70.0,
            weight: 1.0,
            description: "Test".into(),
            domain: AdaptabilityDomain::Technical,
        });
        
        let trend = tracker.calculate_adaptability_trend("sys1");
        // Trend can be positive or negative depending on order, but should be ±20
        assert!(trend.abs() == 20.0);
    }
    
    #[test]
    fn test_export_import() {
        let mut tracker = AdaptabilityTracker::new();
        tracker.initialize();
        
        let assessment_id = tracker.create_assessment("Test".into(), SubjectType::System, "sys1".into());
        tracker.add_metric_to_assessment(&assessment_id, AdaptabilityMetric {
            name: "Test".into(),
            value: 75.0,
            weight: 1.0,
            description: "Test".into(),
            domain: AdaptabilityDomain::Technical,
        });
        
        let export = tracker.export_data();
        
        let mut tracker2 = AdaptabilityTracker::new();
        tracker2.import_data(export);
        
        assert_eq!(tracker2.get_statistics().total_assessments, tracker.get_statistics().total_assessments);
        assert_eq!(tracker2.get_statistics().total_strategies, tracker.get_statistics().total_strategies);
    }
}
