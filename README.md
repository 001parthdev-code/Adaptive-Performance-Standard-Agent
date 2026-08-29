# APS — Adaptive Performance Standard Agent

> A research-engineering system for adaptive test-time control in AI agents under fixed objectives and constrained authority.

## Overview

APS (Adaptive Performance Standard Agent) investigates whether an AI agent can use evidence from its own demonstrated performance to dynamically adjust the level of performance it considers acceptable in future execution.

A typical agent loop can be represented as:

```text
Task → Generate → Evaluate → Accept / Retry
```

The criterion governing `Accept / Retry` is often static: a fixed evaluator threshold, reward criterion, stopping rule, or prompt-level instruction.

This creates an interesting limitation.

Suppose an agent is configured to accept outcomes with quality:

```math
Q_t \geq 0.70
```

but repeatedly demonstrates that it can achieve:

```math
Q_t \approx 0.85 \text{ to } 0.90
```

A future outcome scoring `0.71` still satisfies the original acceptance criterion, even though the system has evidence that substantially stronger performance may be achievable.

APS asks whether that evidence should influence future execution.

The project introduces an explicit **performance standard**, denoted by $\sigma_t$, that represents the current minimum quality the execution system attempts to satisfy before accepting an outcome.

Critically, APS separates this standard from the agent's objective.

```text
Objective
"What must be accomplished?"

        ≠

Performance Standard
"What quality is sufficient to stop?"
```

The objective remains externally defined and fixed.

The performance standard may adapt.

---

## Research Question

The central research question of APS is:

> **Can demonstrated performance provide a useful control signal for adapting an AI agent's performance standard and test-time computation without changing its objective, authority, or security constraints?**

APS studies the feedback loop:

```text
Observed Performance
        │
        ▼
Performance Evaluation
        │
        ▼
Adaptive Standard
        │
        ▼
Acceptance / Retry Decision
        │
        ▼
Test-Time Computation
        │
        ▼
Future Outcome
```

The goal is not merely to make an agent "try harder."

The goal is to determine whether measured performance history can become a useful control signal for future execution.

---

# 1. Formal Model

Let:

- $J$ denote the externally specified objective;
- $X_t$ denote explicit agent/runtime state at time $t$;
- $\sigma_t$ denote the performance standard;
- $\pi_t$ denote the execution policy;
- $A_t$ denote a selected action;
- $O_t$ denote the resulting outcome;
- $Q(O_t)$ denote measured outcome quality.

The execution loop is modeled as:

```math
(X_t, \sigma_t)
\rightarrow
\pi_t
\rightarrow
A_t
\rightarrow
O_t
\rightarrow
Q(O_t)
\rightarrow
(X_{t+1}, \sigma_{t+1})
```

For convenience:

```math
Q_t = Q(O_t)
```

The objective is held fixed throughout an execution:

```math
J_{t+1} = J_t
```

while the performance standard may change:

```math
\sigma_{t+1} \neq \sigma_t
```

This separation is fundamental to APS.

---

# 2. Objective vs. Performance Standard

## Objective

The objective specifies **what the system is required to accomplish**.

Example:

```text
Produce a correct solution to the assigned programming task.
```

During an execution, APS requires:

```math
J_{t+1} = J_t
```

The adaptive mechanism is not permitted to redefine this objective.

---

## Performance Standard

The performance standard specifies **what measured quality the runtime currently considers sufficient for acceptance**.

For example:

```math
\sigma_t = 0.70
```

means that, while execution budget remains available, an outcome with quality below `0.70` should not normally satisfy the runtime's acceptance condition.

Conceptually:

```text
                    EXECUTION

Objective J
    │
    │ defines what must be achieved
    ▼
  Agent
    │
    ▼
Candidate
    │
    ▼
Evaluator ──────► Qₜ
                  │
                  ▼
             Compare with σₜ
                  │
             ┌────┴────┐
             ▼         ▼
           Accept     Retry
```

APS adapts $\sigma_t$.

It does not adapt $J$.

---

# 3. Core Hypothesis

Suppose an agent begins with:

```math
\sigma_0 = 0.60
```

and subsequently demonstrates outcomes such as:

```text
0.72
0.81
0.84
0.88
```

A static system continues using:

```math
\sigma_t = 0.60
```

and may therefore accept a future outcome scoring `0.61`.

APS investigates whether previous demonstrated performance should alter that future acceptance decision.

The initial hypothesis is:

> **An agent that adapts its performance standard from measured outcomes may become less willing to accept solutions substantially below previously demonstrated performance, producing measurable changes in outcome quality and computational expenditure.**

This is a hypothesis, not an assumption.

APS is explicitly designed to produce meaningful results even if the hypothesis is false.

---

# 4. APS-V0 Adaptive Standard

APS-V0 begins with a deliberately simple update rule:

```math
\sigma_{t+1}
=
\sigma_t
+
\alpha \max(0, Q_t - \sigma_t)
```

where:

- $Q_t$ is measured outcome quality;
- $\sigma_t$ is the current performance standard;
- $\alpha \in [0,1]$ is the adaptation rate.

The parameter $\alpha$ controls how rapidly the standard moves toward demonstrated performance.

---

## 4.1 Performance Above the Standard

If:

```math
Q_t > \sigma_t
```

then:

```math
\sigma_{t+1} > \sigma_t
```

The performance standard partially moves toward the demonstrated result.

For example:

```math
\sigma_t = 0.60
```

```math
Q_t = 0.80
```

```math
\alpha = 0.25
```

Therefore:

```math
\sigma_{t+1}
=
0.60
+
0.25(0.80 - 0.60)
```

which gives:

```math
\sigma_{t+1} = 0.65
```

The system has observed performance substantially above its previous standard and therefore raises its future acceptance threshold.

---

## 4.2 Performance At or Below the Standard

If:

```math
Q_t \leq \sigma_t
```

then:

```math
\sigma_{t+1} = \sigma_t
```

APS-V0 therefore does not automatically lower its standard following poor performance.

Consequently:

```math
\sigma_{t+1} \geq \sigma_t
```

The V0 controller is monotonic.

This is intentionally a baseline mechanism.

APS does **not** claim that monotonic adaptation is optimal.

One objective of the research is to determine where this mechanism fails.

---

# 5. Behavioral Effect

An adaptive standard is useful only if it changes execution behavior.

APS-V0 therefore uses $\sigma_t$ directly in the stopping decision.

```text
              Generate Candidate
                      │
                      ▼
                  Evaluate
                      │
                      ▼
                     Qₜ
                      │
                      ▼
                  Qₜ ≥ σₜ ?
                  /       \
                YES        NO
                 │          │
                 ▼          ▼
              ACCEPT    Budget left?
                           /     \
                         YES      NO
                          │        │
                          ▼        ▼
                        RETRY     STOP
```

Consider an outcome with:

```math
Q_t = 0.74
```

A static system with:

```math
\sigma = 0.70
```

accepts the outcome:

```math
0.74 \geq 0.70
```

An adaptive system whose standard has reached:

```math
\sigma_t = 0.82
```

rejects the same outcome:

```math
0.74 < 0.82
```

and may allocate another attempt if sufficient execution budget remains.

The underlying model has not changed.

The objective has not changed.

The system's execution policy has changed.

---

# 6. Adaptive Test-Time Control

This makes APS a form of **adaptive test-time control for AI agents**.

The mechanism can be represented as:

```text
Historical Performance
         │
         ▼
    Standard σₜ
         │
         ▼
Current Outcome Qₜ
         │
         ▼
    Qₜ ≥ σₜ ?
      /      \
    YES       NO
     │         │
     ▼         ▼
   STOP    More Compute
```

APS therefore studies how performance history affects the allocation of future computation.

The relevant relationship is:

```math
\text{Outcome Quality}
=
f(
\sigma_t,
\text{Task},
\text{Execution Budget}
)
```

A higher standard may improve outcome quality.

It may also increase:

- inference calls;
- token consumption;
- tool calls;
- latency;
- execution cost;
- budget exhaustion.

APS therefore treats **quality and compute jointly**.

---

# 7. Research Questions

APS-V0 is designed around several explicit research questions.

### RQ1 — Behavioral Effect

Does an adaptive performance standard produce a measurable change in stopping and retry behavior?

### RQ2 — Outcome Quality

Does adaptation change the quality distribution of accepted outcomes?

### RQ3 — Computational Cost

How does adaptation affect:

- attempts;
- model calls;
- token usage;
- tool calls;
- execution time;
- total computational expenditure?

### RQ4 — Standard Dynamics

How does $\sigma_t$ evolve over time?

Does it:

- converge;
- increase gradually;
- increase too aggressively;
- become poorly calibrated;
- produce excessive retries?

### RQ5 — Distribution Shift

What happens when historical performance is not representative of current task difficulty?

### RQ6 — Quality–Compute Efficiency

Does the additional computation induced by a higher standard produce enough quality improvement to justify its cost?

---

# 8. Experimental Design

The central APS-V0 experiment compares:

```text
┌────────────────────────┐
│ Static Standard Agent  │
└────────────┬───────────┘
             │
             │
             │  versus
             │
             ▼
┌────────────────────────┐
│ Adaptive Standard Agent│
└────────────────────────┘
```

Both agents must use the same:

- underlying model;
- task sequence;
- objective;
- evaluator;
- available tools;
- model parameters;
- per-attempt limits;
- maximum number of attempts;
- total execution budget.

The independent variable is the **performance-standard policy**.

---

## Static Controller

The static agent maintains:

```math
\sigma_t = \sigma_0
```

for every task.

---

## Adaptive Controller

The adaptive agent maintains:

```math
\sigma_{t+1}
=
\sigma_t
+
\alpha \max(0,Q_t-\sigma_t)
```

No other behavioral difference should be intentionally introduced in the initial experiment.

This isolation is necessary to attribute observed effects to the adaptive-standard mechanism.

---

# 9. Evaluation

APS-V0 prioritizes deterministic or mechanically verifiable evaluators where possible.

For an initial code-generation benchmark, outcome quality may be defined as:

```math
Q_t
=
\frac{
\text{Hidden Tests Passed}
}{
\text{Total Hidden Tests}
}
```

with:

```math
0 \leq Q_t \leq 1
```

This provides a measurable evaluation signal without requiring another language model to subjectively judge output quality.

Later experiments may investigate richer evaluators.

---

# 10. Measurements

APS records both performance and computational expenditure.

| Metric | Purpose |
|---|---|
| Mean outcome quality | Overall task performance |
| Accepted outcome quality | Quality the system ultimately accepts |
| Best observed score | Highest demonstrated task score |
| Attempts per task | Retry behavior |
| Model calls | Inference expenditure |
| Token usage | Compute proxy |
| Tool calls | External execution cost |
| Execution time | Latency |
| Budget exhaustion rate | Unsatisfied-standard behavior |
| Standard trajectory | Adaptation dynamics |

The primary analysis should examine the relationship between quality and cost rather than optimizing either independently.

---

# 11. Quality–Compute Frontier

Suppose APS produces:

```text
Static Agent

Accepted quality: 0.74
Average attempts:  1.3
```

while the adaptive agent produces:

```text
Adaptive Agent

Accepted quality: 0.76
Average attempts:  2.8
```

The adaptive system technically improved quality.

That does not necessarily mean it improved the system.

The central tradeoff is:

```text
             Outcome Quality
                    ▲
                    │
                    │
                    │
                    └────────────► Computational Cost
```

APS therefore asks whether adaptive standards move the system toward a more useful **quality–compute frontier**.

---

# 12. Known Research Risk: Capability Is Task-Dependent

Historical performance must not be confused with general capability.

Suppose:

```math
Q_t = 0.95
```

on one task.

That does not imply that the agent can achieve:

```math
Q = 0.95
```

on an unrelated or substantially harder task.

Therefore, APS does not treat:

```math
\max(Q_1, Q_2, \ldots, Q_t)
```

as a reliable estimate of general model capability.

For APS-V0, experiments should initially use sufficiently controlled task distributions to make a global standard interpretable.

Later work may investigate task-conditioned capability estimates.

---

# 13. Distribution Shift

The monotonic V0 controller creates an important failure mode.

```text
Easy Tasks
    │
    ▼
High Performance
    │
    ▼
σ Increases
    │
    ▼
Task Distribution Changes
    │
    ▼
Hard Tasks
    │
    ▼
Q < Historical σ
    │
    ▼
Repeated Retries
    │
    ▼
Budget Exhaustion
```

This behavior would not invalidate the project.

It would demonstrate that a global historical performance standard becomes miscalibrated under changing task difficulty.

That would motivate later mechanisms such as:

- decaying standards;
- rolling performance windows;
- task-conditioned standards;
- difficulty-normalized standards;
- uncertainty-aware controllers.

---

# 14. Secure Runtime

APS is not intended to rely on prompt instructions as its security boundary.

The system is designed around a separate trusted runtime responsible for controlling:

- objective integrity;
- execution state;
- performance-standard transitions;
- resource budgets;
- capabilities;
- policy enforcement;
- audit events.

The foundational security assumption is:

> **Model output is untrusted data, not runtime authority.**

The model may propose an action.

The runtime determines whether that action is authorized.

---

# 15. Target Architecture

APS separates research, control, and execution responsibilities.

```text
┌──────────────────────────────────────────────┐
│                RESEARCH PLANE                │
│                    Python                    │
│                                              │
│  Experiments                                 │
│  Benchmarks                                  │
│  Statistical Analysis                        │
│  Model Adapters                              │
│  Research Configuration                      │
└──────────────────────┬───────────────────────┘
                       │
                Typed Interface
                       │
                       ▼
┌──────────────────────────────────────────────┐
│                CONTROL PLANE                 │
│                     Rust                     │
│                                              │
│  Objective Integrity                        │
│  Execution State Machine                     │
│  Performance Standard Controller             │
│  Budget Enforcement                          │
│  Capability Enforcement                      │
│  Policy Enforcement                          │
│  Audit/Event Generation                      │
└──────────────────────┬───────────────────────┘
                       │
              Constrained Execution
                       │
                       ▼
┌──────────────────────────────────────────────┐
│               EXECUTION PLANE                │
│                 Rust / C++                   │
│                                              │
│  Sandboxed Evaluation                        │
│  Process Isolation                           │
│  Resource Enforcement                        │
│  Native Execution Primitives                 │
│  Generated-Code Execution                    │
└──────────────────────┬───────────────────────┘
                       │
                       ▼
                  Environment
```

Python is used for research iteration, experimentation, and model integration.

Rust is intended to implement the security-sensitive control plane.

C++ is reserved for native components where performance, operating-system interaction, or library interoperability provides a concrete engineering justification.

The use of multiple languages is not itself a project objective.

---

# 16. Trust Model

APS explicitly distinguishes trusted and untrusted components.

## Trusted Computing Base

The intended trusted computing base includes:

```text
APS Runtime
Objective Guard
Standard Controller
Budget Controller
Policy Engine
Capability Enforcement
Security-Critical Audit Logic
```

These components control privileged runtime state.

---

## Partially Trusted Components

Examples include:

```text
Model Adapters
Evaluators
Native Execution Services
```

These components should operate through constrained interfaces.

---

## Untrusted Inputs and Components

APS treats the following as untrusted:

```text
LLM Output
Generated Code
Task Input
Retrieved Content
External Content
Tool Output
Benchmark Content
```

Untrusted content must not directly mutate privileged runtime state.

---

# 17. Security Invariants

APS begins with explicit runtime invariants.

These invariants are intended to become executable security tests rather than remain documentation-only properties.

---

## APS-INV-001 — Objective Integrity

Once an execution begins:

```math
J_{t+1} = J_t
```

The adaptive mechanism cannot modify the objective.

---

## APS-INV-002 — Valid Evaluation Score

Every score entering the trusted controller must satisfy:

```math
0 \leq Q_t \leq 1
```

Invalid evaluator output must be rejected.

---

## APS-INV-003 — Bounded Performance Standard

The runtime must maintain:

```math
0 \leq \sigma_t \leq 1
```

Invalid standard transitions must be rejected.

---

## APS-INV-004 — Controlled Standard Mutation

Only authorized runtime logic operating on validated evaluation results may modify $\sigma_t$.

The model cannot directly execute:

```text
set_standard(...)
```

or equivalent privileged state mutations.

---

## APS-INV-005 — Budget Dominance

The performance standard cannot override execution limits.

```text
Qₜ < σₜ
    +
Budget Exhausted
    │
    ▼
   STOP
```

not:

```text
Unlimited Retry
```

---

## APS-INV-006 — Adaptive Performance Does Not Imply Adaptive Authority

The central security invariant of APS is:

```math
\boxed{
\text{Adaptive Performance}
\neq
\text{Adaptive Authority}
}
```

Increasing $\sigma_t$ must not grant:

- additional permissions;
- new tools;
- filesystem authority;
- unrestricted network access;
- additional resource ceilings;
- objective-modification authority;
- security-policy modification.

---

## APS-INV-007 — Security Policy Dominance

If performance optimization conflicts with an enforced security policy:

```text
Security Policy
      │
      ▼
   DOMINATES
      │
      ▼
Performance Optimization
```

Performance improvement never justifies bypassing a security boundary.

---

# 18. Runtime Authority Model

The LLM does not directly execute privileged actions.

```text
                 LLM
                  │
                  ▼
           Proposed Action
                  │
                  ▼
         Structured Parser
                  │
                  ▼
        Schema Validation
                  │
                  ▼
           Policy Engine
                  │
                  ▼
         Capability Check
                  │
                  ▼
            Budget Check
                  │
                  ▼
         Execution Boundary
                  │
                  ▼
             Environment
```

The model proposes.

The runtime authorizes.

The model therefore does not directly control:

```text
Objective
Performance Standard
Permissions
Resource Budget
Runtime State
Security Policy
```

---

# 19. Threat Model

APS will be evaluated against attacks targeting both the AI layer and the runtime.

Initial threat categories include:

### Objective Manipulation

Untrusted input attempts to alter or replace the fixed execution objective.

### Standard Manipulation

The model attempts to directly lower, raise, reset, or otherwise manipulate $\sigma_t$ outside the authorized update mechanism.

### Evaluator Manipulation

Generated output attempts to attack or game the evaluator rather than improve actual task performance.

### Budget Escalation

The agent attempts to obtain additional computational resources after failing to satisfy its performance standard.

### Capability Escalation

The model attempts to invoke tools or resources outside its granted capability set.

### Generated-Code Escape

Model-generated code attempts to escape its execution boundary or access unauthorized system resources.

### Audit Manipulation

A component attempts to suppress, modify, reorder, or delete evidence of runtime state transitions.

The threat model will evolve as implementation introduces concrete attack surfaces.

---

# 20. Auditability

APS runtime decisions should be inspectable.

The runtime is expected to emit structured events such as:

```text
RunStarted
ObjectiveBound
TaskReceived
CandidateGenerated
EvaluationStarted
EvaluationCompleted
StandardUpdated
CandidateRejected
RetryAuthorized
CandidateAccepted
BudgetExhausted
RunCompleted
```

Events may contain:

```text
run_id
task_id
objective_id
attempt
score
sigma_before
sigma_after
remaining_budget
policy_version
timestamp
```

This supports both:

```text
Research Reproducibility
        +
Security Investigation
```

Future versions may investigate tamper-evident or cryptographically chained execution traces.

---

# 21. Research and Security Interaction

The research mechanism and security architecture are intentionally connected.

APS permits:

```text
Performance Adaptation
```

while preventing that adaptation from becoming:

```text
Objective Adaptation
Permission Adaptation
Authority Escalation
Unbounded Resource Acquisition
```

Conceptually:

```text
                  FIXED
          ┌────────┴────────┐
          │                 │
      Objective         Authority
          │                 │
          └────────┬────────┘
                   │
                   ▼
             ┌───────────┐
             │    APS    │
             │  Runtime  │
             └─────┬─────┘
                   │
              ADAPTIVE
                   │
                   ▼
           Performance Standard
                   │
                   ▼
            Execution Policy
                   │
                   ▼
                Action
```

The desired system property is:

```math
\text{Adaptive Execution}
+
\text{Fixed Objective}
+
\text{Constrained Authority}
```

---

# 22. Project Architecture

The target repository structure is:

```text
APS/
│
├── runtime/
│   ├── src/
│   └── Cargo.toml
│
├── research/
│   ├── aps/
│   ├── experiments/
│   ├── benchmarks/
│   └── analysis/
│
├── sandbox/
│   ├── include/
│   ├── src/
│   └── tests/
│
├── proto/
│
├── benchmarks/
│
├── configs/
│
├── results/
│
├── docs/
│   ├── architecture.md
│   ├── threat-model.md
│   ├── research.md
│   └── adr/
│
├── tests/
│   ├── integration/
│   ├── security/
│   └── e2e/
│
├── README.md
├── SECURITY.md
└── .gitignore
```

The architecture represents intended trust boundaries.

It does not imply that every subsystem is currently implemented.

---

# 23. Development Strategy

APS follows a vertical-slice development strategy.

The first complete execution path should be:

```text
Task
 │
 ▼
Research Client
 │
 ▼
APS Runtime
 │
 ├── Bind Objective
 │
 ├── Establish Budget
 │
 ├── Initialize Standard
 │
 ▼
Candidate
 │
 ▼
Evaluator
 │
 ▼
Validated Qₜ
 │
 ▼
APS Runtime
 │
 ├── Accept / Retry
 │
 ├── Update σ
 │
 ├── Enforce Budget
 │
 └── Emit Audit Event
 │
 ▼
Result
```

The first implementation does not require every planned subsystem.

Complexity should only be introduced when required by an experimental or security property.

---

# 24. APS-V0 Success Criteria

APS-V0 succeeds if the system demonstrates all of the following:

1. The execution objective remains fixed.
2. The performance standard exists as explicit runtime state.
3. Evaluation scores are validated before entering the controller.
4. Measured outcomes can update the adaptive standard.
5. The updated standard changes future stopping behavior.
6. Static and adaptive agents can be compared under controlled conditions.
7. Outcome quality and computational expenditure are measurable.
8. Execution decisions are auditable.
9. Resource constraints cannot be overridden by the adaptive mechanism.
10. Untrusted model output cannot directly mutate privileged runtime state.

APS-V0 does **not** require:

```text
Adaptive Agent > Static Agent
```

The first experimental objective is:

> **Determine whether adaptive performance standards produce a measurable and controllable behavioral difference.**

---

# 25. What Would Falsify the Hypothesis?

APS should be capable of producing evidence against its own mechanism.

Potential negative findings include:

- no statistically meaningful behavioral difference;
- negligible accepted-quality improvement;
- computational cost substantially exceeding quality gains;
- unstable standard dynamics;
- persistent threshold miscalibration;
- excessive budget exhaustion;
- poor behavior under distribution shift;
- evaluator gaming;
- inability to generalize beyond narrow task distributions.

These outcomes would not be hidden.

They would determine whether the controller should be redesigned or the hypothesis rejected.

---

# 26. Non-Goals

APS-V0 does not attempt to implement:

- autonomous goal generation;
- unrestricted self-modification;
- model-weight modification;
- reinforcement-learning training;
- unconstrained long-term autonomy;
- self-assigned permissions;
- self-expanded resource budgets;
- unrestricted tool access;
- arbitrary multi-agent coordination.

The initial research target remains narrow:

```text
Outcome
   │
   ▼
Evaluation
   │
   ▼
Performance Standard
   │
   ▼
Execution Decision
```

under:

```text
Fixed Objective
      +
Fixed Authority Boundary
      +
Fixed Resource Ceiling
```

---

# 27. Research Roadmap

The research roadmap is deliberately provisional.

```text
APS-V0
│
├── Global adaptive standard
├── Static baseline
├── Deterministic evaluation
├── Fixed objective
└── Quality–compute measurement
        │
        ▼
APS-V1
│
├── Windowed performance history
├── Standard decay
└── Distribution-shift experiments
        │
        ▼
APS-V2
│
├── Task-conditioned standards
└── Difficulty estimation
        │
        ▼
APS-V3
│
├── Uncertainty-aware adaptation
└── Capability estimation
        │
        ▼
APS-V4
│
└── Adaptive test-time compute control
```

Later versions should be determined by experimental evidence rather than feature accumulation.

---

# 28. Current Status

**Status: Pre-implementation / architecture validation**

Current work focuses on:

- formalizing the hypothesis;
- defining experimental controls;
- specifying runtime invariants;
- establishing trust boundaries;
- constructing the initial threat model;
- designing the trusted execution architecture.

No empirical performance claims are currently made.

---

# 29. Research Aim

APS ultimately investigates two connected problems.

## AI Agent Systems

> **Can demonstrated performance become a useful control signal for dynamically regulating an AI agent's acceptance threshold and allocation of test-time computation?**

## AI Security

> **Can an AI system adapt its execution behavior while its objective, authority, security policy, and resource boundaries remain externally constrained?**

These questions produce the central design principle of APS:

```text
┌──────────────────────────┐
│    Adaptive Execution    │
└────────────┬─────────────┘
             │
             │ under
             ▼
┌──────────────────────────┐
│     Fixed Objective      │
├──────────────────────────┤
│  Constrained Authority   │
├──────────────────────────┤
│  Fixed Resource Ceiling  │
└──────────────────────────┘
```

APS allows the system to adapt **the level of performance it requires before accepting an outcome**.

It does not allow the system to independently adapt **what it is trying to achieve, what authority it possesses, or which security constraints govern its execution**.