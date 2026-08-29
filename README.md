# APS — Adaptive Performance Standard Agent

> A research-engineering system for adaptive test-time control in AI agents under fixed objectives and constrained authority.

## Overview

APS (Adaptive Performance Standard Agent) investigates a simple question:

> **Can an AI agent use evidence from its own demonstrated performance to dynamically adjust what level of future performance it considers acceptable?**

Most agent systems separate execution into some variation of:

```text
Task → Generate → Evaluate → Accept / Retry
```

The acceptance criterion is typically fixed.

An agent may repeatedly demonstrate that it can achieve substantially better results than its configured acceptance threshold, yet this demonstrated performance does not necessarily affect what the system accepts in the future.

APS introduces an explicit adaptive variable called the **performance standard**.

The system distinguishes between:

- the **objective** — what the agent is required to accomplish;
- the **performance standard** — what measured quality the system currently considers sufficient.

The objective remains externally specified and immutable during execution.

The performance standard may change according to measured outcomes.

APS studies whether this distinction can provide a useful mechanism for controlling agent stopping behavior and test-time computation.

---

## Research Problem

Let:

- \(J\) denote the externally specified objective;
- \(\sigma_t\) denote the performance standard at time \(t\);
- \(Q_t\) denote the measured quality of an observed outcome.

APS requires the objective to remain fixed:

$$
J_{t+1} = J_t
$$

while allowing the performance standard to adapt:

$$
\sigma_{t+1} \neq \sigma_t
$$

The central research question is:

> **Can demonstrated performance provide a useful control signal for adapting an AI agent's acceptance threshold and test-time computation without changing its objective or authority?**

This produces the conceptual feedback loop:

```text
                    ┌──────────────────────┐
                    │ Performance Standard │
                    │         σₜ           │
                    └──────────┬───────────┘
                               │
                               ▼
Task → Agent → Candidate → Evaluator → Qₜ
         ▲                        │
         │                        │
         └──── Accept / Retry ◄───┘
                               │
                               ▼
                         Update σₜ₊₁
```

The system therefore studies:

$$
\text{Observed Performance}
\rightarrow
\text{Adaptive Standard}
\rightarrow
\text{Execution Decision}
\rightarrow
\text{Future Performance}
$$

---

## Objective and Performance Standard

APS treats the objective and performance standard as fundamentally different system variables.

### Objective

The objective specifies **what must be accomplished**.

For example:

```text
Produce a correct solution to the given programming problem.
```

During an execution:

$$
J_{t+1} = J_t
$$

The adaptive mechanism is not permitted to modify the objective.

### Performance Standard

The performance standard specifies **what measured level of execution quality is currently considered sufficient**.

For example:

$$
\sigma_t = 0.70
$$

means that an outcome with measured quality below `0.70` should not normally be accepted while additional execution budget remains.

The distinction is:

```text
Objective J
    │
    └── What should be achieved?

Performance Standard σ
    │
    └── How good must the current result be before stopping?
```

APS adapts the second quantity, not the first.

---

## Hypothesis

Suppose an agent begins with:

$$
\sigma_0 = 0.60
$$

but repeatedly demonstrates outcomes such as:

```text
0.72
0.81
0.84
0.88
```

A static system continues using:

$$
\sigma = 0.60
$$

and may therefore accept a future result scoring `0.61`.

APS investigates whether demonstrated performance should instead influence future acceptance decisions.

The hypothesis is:

> **An adaptive performance standard can cause an agent to become less willing to accept outcomes below previously demonstrated levels of performance, potentially improving accepted solution quality at the cost of additional computation.**

This hypothesis is not assumed to be correct.

APS is designed to test it.

---

## APS-V0 Standard Controller

The initial adaptive controller is intentionally simple.

For measured performance \(Q_t\):

$$
\sigma_{t+1}
=
\sigma_t
+
\alpha \max(0, Q_t - \sigma_t)
$$

where:

- \(Q_t\) is measured outcome quality;
- \(\sigma_t\) is the current performance standard;
- \(\alpha \in [0,1]\) is the adaptation rate.

### Performance exceeds the current standard

If:

$$
Q_t > \sigma_t
$$

then:

$$
\sigma_{t+1} > \sigma_t
$$

The standard moves partially toward the demonstrated performance.

For example, given:

$$
\sigma_t = 0.60
$$

$$
Q_t = 0.80
$$

$$
\alpha = 0.25
$$

the next standard becomes:

$$
\sigma_{t+1}
=
0.60 + 0.25(0.80 - 0.60)
=
0.65
$$

### Performance does not exceed the current standard

If:

$$
Q_t \leq \sigma_t
$$

then:

$$
\sigma_{t+1} = \sigma_t
$$

APS-V0 therefore implements a monotonic standard.

This is a baseline controller, not a claim that monotonic adaptation is optimal.

Determining where this controller fails is part of the research.

---

## How the Standard Changes Behavior

A performance standard is only meaningful if it affects execution.

APS-V0 uses \(\sigma_t\) as a stopping threshold:

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

Increasing \(\sigma_t\) therefore changes how readily the system accepts a result.

Consider:

```text
Candidate quality = 0.74
```

For a static agent:

```text
σ = 0.70

0.74 ≥ 0.70
→ Accept
```

For an adaptive agent whose standard has reached `0.82`:

```text
σ = 0.82

0.74 < 0.82
→ Retry
```

The model may be identical.

The task may be identical.

The objective may be identical.

What changed is the execution controller's willingness to stop.

---

## Adaptive Test-Time Control

APS can therefore be interpreted as an adaptive test-time control system.

```text
Historical Outcomes
        │
        ▼
Measured Performance
        │
        ▼
Performance Standard σ
        │
        ▼
Acceptance Decision
        │
   ┌────┴────┐
   ▼         ▼
 Accept     Retry
              │
              ▼
       Additional Compute
```

The important research problem is consequently not simply whether a larger \(\sigma\) produces better answers.

APS investigates the relationship:

$$
\text{Outcome Quality}
=
f(
\text{Performance Standard},
\text{Task},
\text{Compute Budget}
)
$$

A useful adaptive controller should improve the allocation of computation rather than merely consume more of it.

---

## Experimental Design

APS-V0 compares two agents.

### Static Standard Agent

The static agent maintains:

$$
\sigma_t = \sigma_0
$$

for all \(t\).

### Adaptive Standard Agent

The adaptive agent maintains:

$$
\sigma_{t+1}
=
\sigma_t
+
\alpha \max(0,Q_t-\sigma_t)
$$

Both agents must use the same:

- underlying model;
- task sequence;
- objective;
- evaluator;
- available tools;
- generation configuration;
- maximum attempts;
- execution budget.

The independent variable is:

```text
Performance-standard policy
```

Everything else should remain controlled.

---

## What APS Measures

APS does not evaluate performance using outcome quality alone.

### Outcome Quality

Mean measured quality:

$$
\bar{Q}
$$

### Accepted Quality

Quality of the solutions the system ultimately accepts.

This determines whether adaptation actually causes the agent to settle for stronger outcomes.

### Best Observed Performance

$$
Q_{\max}
$$

This records demonstrated performance.

It should not automatically be interpreted as general agent capability.

### Attempts

How many attempts are required before acceptance or budget exhaustion?

### Model Calls

How much inference is consumed?

### Token Usage

How does adaptation affect token expenditure?

### Latency

Does increased execution quality introduce unacceptable latency?

### Budget Exhaustion

How frequently does the adaptive standard become difficult or impossible to satisfy?

### Standard Trajectory

Track:

$$
\sigma_0,\sigma_1,\sigma_2,\ldots,\sigma_n
$$

to study the dynamics of adaptation.

---

## Quality–Compute Tradeoff

APS does not assume:

```text
higher quality = better system
```

If an adaptive agent produces:

```text
+2% accepted quality
+150% inference cost
```

the mechanism may not be practically useful.

The primary systems tradeoff is therefore:

$$
\boxed{
\text{Outcome Quality}
\quad \text{vs.} \quad
\text{Computational Cost}
}
$$

A successful adaptive controller should move the system toward a better quality–compute frontier.

---

# Security Model

APS allows part of the execution policy to adapt.

That creates a second research-engineering problem:

> **How can an AI system adapt its execution behavior without gaining the ability to modify its objective, authority, security policy, or resource limits?**

APS addresses this by separating the model from the trusted execution runtime.

The foundational rule is:

> **Model output is untrusted data, not runtime authority.**

---

## Target Architecture

APS is designed around three architectural planes.

```text
┌─────────────────────────────────────────────┐
│               RESEARCH PLANE                │
│                   Python                    │
│                                             │
│  Experiments                                │
│  Benchmarks                                 │
│  Analysis                                   │
│  Model adapters                             │
└──────────────────────┬──────────────────────┘
                       │
                  Typed interface
                       │
                       ▼
┌─────────────────────────────────────────────┐
│                CONTROL PLANE                │
│                    Rust                     │
│                                             │
│  Objective integrity                       │
│  Execution state machine                    │
│  Performance-standard controller            │
│  Budget enforcement                         │
│  Capability enforcement                     │
│  Policy enforcement                         │
│  Audit events                               │
└──────────────────────┬──────────────────────┘
                       │
                Restricted execution
                       │
                       ▼
┌─────────────────────────────────────────────┐
│               EXECUTION PLANE               │
│                 Rust / C++                  │
│                                             │
│  Sandboxed evaluation                       │
│  Resource isolation                         │
│  Native execution primitives                │
│  Generated-code execution                   │
└──────────────────────┬──────────────────────┘
                       │
                       ▼
                  Environment
```

Python is used for research iteration and model integration.

Rust forms the primary trusted control plane.

C++ is reserved for components where native execution, interoperability, or performance requirements justify its use.

The use of multiple languages is an architectural decision, not a project objective.

---

## Trust Model

### Trusted

The intended trusted computing base includes:

```text
APS Runtime
Objective Guard
Standard Controller
Budget Controller
Policy Engine
Capability Enforcement
Security-critical Audit Logic
```

### Partially Trusted

```text
Model adapters
Evaluators
Native execution services
```

These components receive constrained interfaces and authority.

### Untrusted

```text
LLM output
Generated code
Task input
External content
Retrieved content
Tool output
Benchmark content
```

Untrusted data must not directly mutate privileged runtime state.

---

## Security Invariants

APS defines several initial runtime invariants.

### APS-INV-001 — Objective Integrity

Once execution begins:

$$
J_{t+1} = J_t
$$

The adaptive mechanism cannot modify the objective.

### APS-INV-002 — Valid Outcome Score

$$
0 \leq Q_t \leq 1
$$

Invalid evaluator results must not enter the standard controller.

### APS-INV-003 — Bounded Performance Standard

$$
0 \leq \sigma_t \leq 1
$$

Invalid standard transitions must be rejected.

### APS-INV-004 — Controlled Standard Mutation

Only validated evaluation outcomes may trigger an authorized standard transition.

Model-generated content cannot directly assign \(\sigma_t\).

### APS-INV-005 — Budget Dominance

The performance standard cannot override execution limits.

```text
Q < σ
AND
budget exhausted
        │
        ▼
      STOP
```

A high standard cannot produce unlimited retries.

### APS-INV-006 — Adaptive Performance ≠ Adaptive Authority

The central security property of APS is:

$$
\boxed{
\text{Adaptive Performance}
\not\Rightarrow
\text{Adaptive Authority}
}
$$

Increasing the performance standard must not grant:

- additional permissions;
- additional tools;
- arbitrary filesystem access;
- unrestricted network access;
- larger resource ceilings;
- objective modification;
- security-policy modification.

### APS-INV-007 — Security Policy Dominance

Performance optimization cannot override an enforced security decision.

```text
Security constraint
        >
Performance optimization
```

---

## Runtime Authority

The model may propose actions.

The runtime authorizes them.

```text
              LLM
               │
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
```

The model therefore does not directly control:

```text
objective
performance standard
permissions
budget
runtime state
security policy
```

This boundary is intended to remain valid even when model output is adversarial.

---

## Threat Model

APS will explicitly test attacks against its execution architecture.

Initial threat categories include:

### Objective Manipulation

Untrusted content attempts to modify or replace the fixed execution objective.

### Standard Manipulation

The model attempts to directly lower, raise, reset, or otherwise manipulate \(\sigma\).

### Evaluator Manipulation

Generated output attempts to attack the evaluation mechanism rather than solve the task.

### Budget Escalation

The agent attempts to obtain additional computation because its current outcome remains below \(\sigma\).

### Capability Escalation

The model attempts to access tools or resources outside its granted capability set.

### Generated-Code Escape

Model-generated code attempts to escape its execution boundary.

### Audit Manipulation

A component attempts to modify or suppress execution history.

These categories will evolve with the implementation and adversarial testing.

---

## Auditability

APS runtime decisions should be inspectable and reproducible.

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

An event may contain:

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

This provides evidence for both experimental analysis and security investigation.

Future versions may investigate tamper-evident execution traces.

---

# Known Research Risks

APS begins with several known weaknesses rather than hiding them.

## Historical Performance Is Not General Capability

A score of:

$$
Q = 0.95
$$

on one task does not establish that the agent can achieve comparable performance on another task.

Therefore:

$$
C_t = \max(C_{t-1},Q_t)
$$

is **not** treated as a reliable general capability estimate.

APS-V0 will initially use controlled task distributions before investigating task-conditioned standards.

---

## Distribution Shift

Consider:

```text
Easy Tasks
    │
    ▼
High Q
    │
    ▼
σ increases
    │
    ▼
Hard Tasks
    │
    ▼
Q falls below historical σ
    │
    ▼
Excessive retries
```

This may expose a fundamental limitation of globally maintained performance standards.

Potential future work includes task-conditioned or difficulty-normalized standards.

---

## Monotonicity

APS-V0 does not decrease \(\sigma\).

That means:

$$
\sigma_{t+1} \geq \sigma_t
$$

for every update.

This may produce poor calibration over long or heterogeneous task sequences.

The behavior is intentional in V0 because it creates a simple, testable baseline.

---

## Evaluator Dependence

APS adapts according to \(Q\).

Therefore:

> A performance controller cannot be more reliable than the signal used to evaluate performance.

A vulnerable, noisy, or poorly specified evaluator can cause the system to adapt toward the wrong behavior.

Evaluator integrity is therefore both a research problem and a security boundary.

---

# Research Roadmap

APS development is intentionally incremental.

```text
APS-V0
│
├── Static vs adaptive standard
├── Deterministic evaluation
├── Fixed objective
├── Fixed execution budget
└── Quality–compute measurement
        │
        ▼
APS-V1
│
├── Windowed standards
├── Decay mechanisms
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
└── Adaptive test-time compute controller
```

This roadmap is provisional.

Later versions should be determined by experimental results rather than predetermined feature expansion.

---

# What Would Falsify APS?

APS is not designed around the assumption that adaptive standards must work.

Evidence against the proposed mechanism would include:

- no meaningful behavioral difference from static thresholds;
- negligible quality improvement;
- disproportionately higher computational cost;
- unstable standard dynamics;
- persistent miscalibration;
- excessive budget exhaustion;
- poor behavior under distribution shift;
- evaluator gaming;
- inability to generalize beyond narrow task distributions.

A negative result is still useful if the experiment identifies why the mechanism fails.

---

# APS-V0 Success Criteria

APS-V0 is successful if the system demonstrates that:

1. the objective remains fixed throughout execution;
2. the performance standard exists as explicit runtime state;
3. validated outcomes can update the adaptive standard;
4. the updated standard changes future acceptance or retry decisions;
5. static and adaptive agents can be compared under controlled conditions;
6. quality and computational cost can both be measured;
7. runtime decisions can be audited;
8. resource and security constraints dominate the adaptive mechanism.

APS-V0 does **not** require:

```text
Adaptive Agent > Static Agent
```

The first question is more fundamental:

> **Does the mechanism create a measurable and controllable behavioral difference?**

---

# Non-Goals

APS is not currently intended to implement:

- autonomous goal generation;
- unrestricted self-modification;
- model-weight modification;
- reinforcement-learning training;
- unconstrained long-term autonomy;
- self-assigned permissions;
- self-expanded resource budgets;
- arbitrary multi-agent coordination.

The research target remains narrow:

$$
\boxed{
\text{Outcome}
\rightarrow
\text{Evaluation}
\rightarrow
\text{Performance Standard}
\rightarrow
\text{Execution Decision}
}
$$

under externally fixed objective, authority, and resource constraints.

---

# Current Status

**Status: Pre-implementation / architecture validation**

Current work focuses on:

- formalizing the research hypothesis;
- defining experimental controls;
- specifying runtime invariants;
- defining trust boundaries;
- constructing the initial threat model;
- designing the trusted execution architecture.

No empirical claims about APS performance are currently made.

---

# Research Aim

APS ultimately investigates two connected questions.

### Agent Systems

> Can demonstrated performance be transformed into a useful control signal for dynamically allocating test-time computation?

### AI Security

> Can an AI system safely adapt its execution behavior while its objective, authority, security policy, and resource boundaries remain externally constrained?

Together, these define the central APS principle:

$$
\boxed{
\text{Adaptive Execution}
\quad+\quad
\text{Fixed Objective}
\quad+\quad
\text{Constrained Authority}
}
$$

The system may adapt **how well it requires itself to perform**.

It may not independently adapt **what it is trying to achieve or what it is allowed to do**.