<!--
Sync Impact Report:
- Version change: 0.0.0 → 1.0.0
- List of modified principles:
    - [PRINCIPLE_1_NAME] → Unified Visibility
    - [PRINCIPLE_2_NAME] → LRCO Integration
    - [PRINCIPLE_3_NAME] → Assessment Focus
    - [PRINCIPLE_4_NAME] → Data Integrity
    - [PRINCIPLE_5_NAME] → Responsive Design
- Added sections: Core Principles, Additional Constraints, Development Workflow
- Removed sections: None
- Templates requiring updates:
    - .specify/templates/plan-template.md (✅ aligned)
    - .specify/templates/spec-template.md (✅ aligned)
    - .specify/templates/tasks-template.md (✅ aligned)
- Follow-up TODOs: None
-->

# LRCO Unified Manager Constitution

## Core Principles

### I. Unified Visibility
The system MUST provide a single, consolidated dashboard that aggregates class and assessment data from all three assigned state schools. Teachers should not need to switch contexts or login multiple times to see their full weekly schedule.

### II. LRCO Integration
All data structures and synchronization protocols MUST strictly adhere to the LRCO SEED Paraná system standards. Integration must be seamless, ensuring that data fetched from LRCO is accurately represented and any local organization remains consistent with the source.

### III. Assessment Focus (NON-NEGOTIABLE)
The primary function of the application is the organization and visualization of "go" assessment dates. Every feature added MUST directly or indirectly support the teacher's ability to plan, track, and view these assessments across their classes.

### IV. Data Integrity
Synchronization between the application and the LRCO system MUST be robust. The system must handle potential discrepancies gracefully and prioritize the LRCO system as the "source of truth" for official class lists and base schedules.

### V. Responsive Design
The application MUST be fully responsive and optimized for both desktop and mobile devices. Teachers often need to check their assessment schedule while moving between classrooms or schools, requiring a mobile-first approach to UI/UX.

## Additional Constraints

### Security & Privacy
The system MUST comply with SEED Paraná data privacy regulations. Student and teacher data must be handled securely, with local storage encrypted and no sensitive information exposed via logging or insecure APIs.

## Development Workflow

### Feature-Driven Development
All new features MUST be defined in a specification file (`spec.md`) following the `spec-template.md` before implementation begins. This ensures that every addition aligns with the core principles, especially the "Assessment Focus".

## Governance
This constitution supersedes all other development practices within this project. Any deviation from these principles must be documented and justified in the implementation plan's "Complexity Tracking" section.

### Amendment Procedure
Amendments to this constitution require a version bump and an update to the "Sync Impact Report". Significant changes to Core Principles require a MAJOR version bump.

**Version**: 1.0.0 | **Ratified**: 2026-05-15 | **Last Amended**: 2026-05-15
