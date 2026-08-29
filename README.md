# PS — Adaptive Performance Standard

> A secure AI execution runtime for investigating adaptive performance standards under fixed objectives and constrained authority.

PS is a research-oriented AI systems project investigating whether an AI agent can use evidence from its own demonstrated performance to adapt the quality threshold governing future execution decisions—without modifying its objective, permissions, or security constraints.

The project studies a distinction between:

- **what an agent is required to accomplish**, and
- **what level of execution quality the agent considers sufficient**.

PS treats the first as externally defined and immutable during an execution, while allowing the second to adapt from measured outcomes.

The initial research question is:

> **Can demonstrated performance provide a useful control signal for adapting an AI agent's acceptance threshold and test-time computation while its objective and authority remain fixed?**

PS is being developed as both an experimental platform for this question and a secure runtime architecture for studying bounded adaptation in AI agents.

---

## 1. Motivation

A common agent execution loop can be represented as:

```text
Task
  ↓
Plan
  ↓
Act
  ↓
Observe
  ↓
Evaluate
  ↓
Accept / Retry
```

The agent may evaluate its output repeatedly, but the criterion determining when an output is "good enough" is often externally specified or fixed for the duration of the system.

Consider an agent whose acceptance threshold is:

```text
σ = 0.70
```

Suppose the same system repeatedly demonstrates outcomes in the range:

```text
Q ≈ 0.85–0.90
```

where `Q` is a measurable outcome-quality function.

A later outcome scoring `0.71` still satisfies the original threshold, despite evidence that the system has previously achieved substantially stronger outcomes.

This raises a systems question:

> Should demonstrated performance influence the level of performance an agent subsequently requires before terminating computation?

PS investigates this question by introducing an explicit **performance standard** into the agent execution loop.

---

## 2. Objective vs. Performance Standard

The central architectural distinction in PS is between an agent's **objective** and its **performance standard**.

Let

\[
J
\]

denote the externally specified objective.

For a given execution, PS requires:

\[
J_{t+1}=J_t
\]

The adaptive mechanism is not permitted to modify `J`.

Let

\[
\sigma_t \in [0,1]
\]

denote the performance standard at time `t`.

`σ_t` represents the minimum measured quality the runtime should attempt to obtain before accepting an outcome, subject to resource and policy constraints.

Unlike the objective:

\[
\sigma_{t+1}
\]

may differ from:

\[
\sigma_t
\]

based on observed performance.

Therefore:

```text
Objective J
    │
    └── What must be accomplished?

Performance Standard σ
    │
    └── What measured quality is sufficient to stop?
```

These quantities are intentionally independent.

A changing performance standard MUST NOT imply a changing objective.

---

## 3. Research Hypothesis

PS investigates the following hypothesis:

> An agent's demonstrated performance can be used to adapt its future acceptance threshold, producing measurable changes in stopping behavior, outcome quality, and computational expenditure without modifying the underlying objective.

The corresponding feedback loop is:

\[
(X_t,\sigma_t)
\rightarrow
\pi_t
\rightarrow
A_t
\rightarrow
O_t
\rightarrow
Q(O_t)
\rightarrow
(X_{t+1},\sigma_{t+1})
\]

where:

| Symbol | Meaning |
|---|---|
| \(X_t\) | explicit execution state |
| \(J\) | fixed externally specified objective |
| \(\sigma_t\) | current performance standard |
| \(\pi_t\) | execution/decision policy |
| \(A_t\) | selected action |
| \(O_t\) | observed outcome |
| \(Q(O_t)\) | measured outcome quality |

The hypothesis is deliberately falsifiable.

PS does **not** assume that an adaptive standard will outperform a static standard.

The adaptive mechanism may improve outcome quality, waste computation, become poorly calibrated, fail under distribution shift, or produce no meaningful advantage.

Each is an experimentally meaningful result.

---

## 4. Initial Standard Update Rule

The first experimental controller uses:

\[
\boxed{
\sigma_{t+1}
=
\sigma_t
+
\alpha\max(0,Q_t-\sigma_t)
}
\]

where:

- `Q_t` is measured performance,
- `σ_t` is the current standard,
- `α ∈ [0,1]` is the adaptation rate.

If:

\[
Q_t>\sigma_t
\]

then:

\[
\sigma_{t+1}>\sigma_t
\]

and the standard partially moves toward demonstrated performance.

If:

\[
Q_t\leq\sigma_t
\]

then:

\[
\sigma_{t+1}=\sigma_t
\]

for the initial controller.

This rule is a **baseline mechanism**, not a claim of optimality.

One purpose of PS is to determine where this simple formulation fails.

Potential later controllers may investigate:

- windowed performance estimates,
- decaying standards,
- task-conditioned standards,
- uncertainty-aware standards,
- difficulty-normalized standards,
- distribution-shift-aware controllers.

These mechanisms are outside the initial experiment.

---

## 5. Behavioral Mechanism

Tracking `σ` without allowing it to affect execution would not constitute adaptation.

PS therefore uses the performance standard as an explicit stopping-control signal.

```text
Generate candidate
       │
       ▼
Evaluate outcome
       │
       ▼
      Q ≥ σ?
      /    \
    yes     no
     │       │
   accept    │
             ▼
       budget available?
          /       \
        yes        no
         │          │
       retry       stop
```

A higher standard can therefore cause the runtime to allocate additional test-time computation by rejecting outcomes that would previously have been accepted.

This exposes a quality–compute tradeoff:

\[
\text{Outcome Quality}
=
f(\sigma,\text{Execution Budget},\text{Task})
\]

PS studies that tradeoff rather than assuming that higher standards are intrinsically better.

---

## 6. Research Questions

The initial system is designed to answer five questions.

### RQ1 — Behavioral Effect

Does an adaptive performance standard produce a measurable change in agent stopping and retry behavior?

### RQ2 — Outcome Quality

Does adaptation change the quality distribution of accepted outcomes?

### RQ3 — Computational Cost

How does adaptation affect:

- model calls,
- attempts,
- token usage,
- tool invocations,
- latency,
- execution cost?

### RQ4 — Stability

How does `σ` evolve over time?

Does it:

- converge,
- increase gradually,
- become excessively aggressive,
- become poorly calibrated,
- or create retry saturation?

### RQ5 — Distribution Shift

What happens when previously demonstrated performance is not representative of the current task distribution?

For example:

```text
easy tasks
    ↓
high observed scores
    ↓
σ increases
    ↓
harder tasks
    ↓
historical σ may become unrealistic
    ↓
excessive retries / compute expenditure
```

This is expected to be an important limitation of globally maintained performance standards.

---

## 7. Experimental Design

The initial experiment compares:

```text
Static Standard Agent
          vs.
Adaptive Standard Agent
```

Both systems MUST use the same:

- underlying model,
- task sequence,
- fixed objective,
- evaluator,
- tools,
- per-attempt limits,
- maximum execution budget,
- generation configuration.

The experimental variable is the standard controller.

### Static

\[
\sigma_t=\sigma_0
\]

### Adaptive

\[
\sigma_{t+1}
=
\sigma_t+\alpha\max(0,Q_t-\sigma_t)
\]

This isolation is necessary to attribute behavioral differences to the adaptive-standard mechanism rather than unrelated agent modifications.

---

## 8. Evaluation

Where possible, V0 uses deterministic or mechanically verifiable evaluation.

For example, in a code-generation environment:

\[
Q(O_t)
=
\frac{\text{hidden tests passed}}
{\text{total hidden tests}}
\]

with:

\[
Q(O_t)\in[0,1]
\]

This is preferred over subjective model-based evaluation for the initial experiment because it improves:

- reproducibility,
- interpretability,
- experimental control,
- comparability between runs.

Primary measurements include:

| Metric | Purpose |
|---|---|
| Mean outcome score | overall performance |
| Accepted outcome score | quality actually accepted |
| Best observed score | maximum demonstrated task performance |
| Attempts per task | retry behavior |
| Model calls | inference expenditure |
| Token usage | compute proxy |
| Execution time | system cost |
| Standard trajectory | adaptation dynamics |
| Budget exhaustion rate | pathological retry behavior |

The primary analysis is expected to examine the **quality–compute frontier**, rather than quality alone.

---

# Secure Runtime Architecture

PS is not intended to rely on prompt instructions as its security boundary.

The system is designed around a separate trusted runtime responsible for enforcing objectives, authority, resource limits, state transitions, and adaptive-standard constraints.

The target architecture separates three concerns:

```text
┌───────────────────────────────────────────┐
│              RESEARCH PLANE               │
│                  Python                   │
│                                           │
│ experiments · benchmarks · analysis       │
│ model adapters · research policies        │
└─────────────────────┬─────────────────────┘
                      │
              typed interface
                      │
                      ▼
┌───────────────────────────────────────────┐
│              CONTROL PLANE                │
│                   Rust                    │
│                                           │
│ objective integrity                       │
│ execution state machine                   │
│ standard controller                       │
│ policy enforcement                        │
│ capability enforcement                    │
│ budget management                         │
│ audit/event generation                    │
└─────────────────────┬─────────────────────┘
                      │
              constrained execution
                      │
                      ▼
┌───────────────────────────────────────────┐
│             EXECUTION PLANE               │
│              Rust / C++                   │
│                                           │
│ evaluator execution                       │
│ process/resource isolation                │
│ native execution primitives               │
│ sandbox integration                       │
└─────────────────────┬─────────────────────┘
                      │
                      ▼
                 Environment
```

Language choice is architectural rather than cosmetic.

Python is used where research iteration and ML ecosystem integration are valuable.

Rust is intended for security-sensitive control logic because PS requires strong state, authority, and memory-safety guarantees.

C++ is reserved for native or performance-sensitive components where its ecosystem or systems-level interoperability provides a concrete advantage. Security-sensitive functionality is not moved into C++ merely for implementation complexity.

---

## 9. Security Model

PS begins from a simple assumption:

> **Model output is untrusted data, not runtime authority.**

An LLM may propose an action.

It does not receive direct authority to perform arbitrary state transitions or system operations.

Conceptually:

```text
LLM output
    │
    ▼
Structured parser
    │
    ▼
Schema validation
    │
    ▼
Policy enforcement
    │
    ▼
Capability check
    │
    ▼
Budget check
    │
    ▼
Execution boundary
```

The runtime, rather than the model, determines whether an operation is permitted.

---

## 10. Trust Boundaries

The intended trust model is:

### Trusted Computing Base

- runtime state machine,
- objective integrity mechanism,
- performance-standard controller,
- policy engine,
- capability enforcement,
- budget controller,
- security-critical audit logic.

### Partially Trusted

- evaluators,
- model adapters,
- native execution services.

These components receive narrowly scoped authority.

### Untrusted

- model-generated content,
- generated code,
- external task content,
- retrieved content,
- tool output,
- benchmark input.

Untrusted content must not directly control privileged runtime state.

---

## 11. Security Invariants

PS is designed around explicit invariants.

### Invariant 1 — Objective Integrity

During an execution:

\[
\boxed{J_{t+1}=J_t}
\]

The adaptive mechanism cannot rewrite the execution objective.

### Invariant 2 — Bounded Standard

\[
\boxed{0\leq\sigma_t\leq1}
\]

Invalid standard transitions must be rejected by the runtime.

### Invariant 3 — Adaptive Performance Does Not Grant Authority

\[
\boxed{
\text{Adaptive Performance}
\not\Rightarrow
\text{Adaptive Authority}
}
\]

Increasing `σ` cannot grant:

- additional permissions,
- new tools,
- arbitrary filesystem access,
- unrestricted network access,
- larger execution budgets,
- objective-modification authority.

### Invariant 4 — Budget Dominates Standard

A performance standard cannot override resource limits.

```text
Q < σ
+
budget exhausted
        ↓
STOP
```

not:

```text
unbounded retry
```

### Invariant 5 — Model Output Is Non-Authoritative

Generated model content cannot directly mutate privileged runtime state.

### Invariant 6 — Security Policy Dominates Optimization

If performance optimization conflicts with an enforced security policy, the security policy wins.

---

## 12. Objective Integrity

The fixed-objective requirement should be enforced structurally rather than solely through prompting.

An execution may conceptually bind:

```text
Objective specification
        ↓
Canonical representation
        ↓
Cryptographic digest
        ↓
Objective identity
        ↓
Execution context
```

Subsequent state transitions reference the bound objective.

The adaptive controller receives no authority to replace it.

This allows the experimental claim:

\[
J_{t+1}=J_t
\]

to correspond to a runtime invariant rather than an instruction given to the language model.

---

## 13. Auditing and Reproducibility

Security and experimental reproducibility both require observable state transitions.

PS therefore intends to expose structured execution events such as:

```text
RunStarted
ObjectiveBound
TaskReceived
CandidateGenerated
EvaluationStarted
EvaluationCompleted
CandidateRejected
RetryAuthorized
StandardUpdated
CandidateAccepted
BudgetExhausted
RunCompleted
```

Relevant event metadata may include:

```text
run_id
task_id
objective_id
policy_version
attempt
score
sigma_before
sigma_after
remaining_budget
timestamp
```

This should make it possible to reconstruct why the runtime accepted, rejected, retried, or terminated an execution.

Future versions may investigate tamper-evident or hash-chained audit traces.

---

## 14. Threat Model

The initial threat model considers failures such as:

### Objective Manipulation

A model or untrusted task attempts to alter the objective governing execution.

### Standard Manipulation

Untrusted content attempts to directly modify `σ` or influence it outside the authorized evaluation path.

### Budget Escalation

The agent attempts to obtain additional execution resources because its current performance remains below `σ`.

### Unauthorized Tool Use

Generated content attempts to invoke capabilities that were not granted to the execution.

### Generated-Code Escape

Model-generated code attempts to escape its execution environment or access unauthorized resources.

### Evaluation Manipulation

An outcome attempts to interfere with the evaluator rather than solve the assigned task.

### Audit Tampering

A component attempts to remove or alter evidence of previous state transitions.

These threats will be refined as implementation exposes concrete attack surfaces.

---

## 15. System Philosophy

PS intentionally separates adaptation from authority.

The system may adapt:

```text
how demanding its acceptance criterion is
```

but not independently adapt:

```text
what its objective is
what permissions it possesses
what security policies apply
what resource ceiling it receives
```

This distinction is fundamental to the project.

```text
               ADAPTIVE
                  │
          performance standard
                  │
                  ▼
Fixed ─────► Secure Runtime ─────► Execution
Objective          │
                   ▼
              Policy / Budget /
              Capability Bounds
```

The desired property is bounded adaptation:

\[
\boxed{
\text{adaptive execution policy}
\;\land\;
\text{fixed objective}
\;\land\;
\text{fixed authority boundary}
}
\]

---

## 16. Project Structure

The target repository structure is:

```text
PS/
│
├── runtime/                 # trusted Rust control plane
│
├── sandbox/                 # isolated/native execution components
│
├── research/                # Python experimental plane
│
├── proto/                   # cross-language contracts
│
├── benchmarks/              # controlled experimental tasks
│
├── configs/                 # experiment configuration
│
├── results/                 # generated experimental results
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
└── SECURITY.md
```

The repository will be developed incrementally. This structure represents architectural boundaries, not a requirement to implement every subsystem before experimentation begins.

---

## 17. Development Strategy

PS follows a vertical-slice development strategy.

The first useful system should establish:

```text
Task
  ↓
Experiment client
  ↓
Secure runtime
  ├── binds objective
  ├── maintains σ
  ├── enforces budget
  └── controls acceptance/retry
  ↓
Evaluator
  ↓
Measured Q
  ↓
Runtime transition
  ↓
Structured audit event
```

Only after this path is functional should additional complexity be introduced.

The project explicitly avoids building infrastructure without an experimental or security requirement.

---

## 18. Non-Goals

The initial system does not attempt to implement:

- autonomous goal generation,
- unrestricted self-modification,
- model weight updates,
- reinforcement-learning training,
- unconstrained long-term autonomy,
- multi-agent coordination,
- arbitrary tool ecosystems,
- self-assigned permissions,
- self-expanded execution budgets.

PS studies a much narrower mechanism:

\[
\boxed{
\text{Outcome}
\rightarrow
\text{Evaluation}
\rightarrow
\text{Adaptive Standard}
\rightarrow
\text{Changed Execution Decision}
}
\]

under fixed objective and authority constraints.

---

## 19. Limitations and Open Questions

Several limitations are known before implementation.

### Historical Performance Is Not General Capability

A high score on one task does not establish equivalent capability on another.

Therefore:

\[
\max(Q_1,\ldots,Q_t)
\]

must not automatically be interpreted as a general capability estimate.

Initial experiments will use controlled task distributions before investigating task-conditioned standards.

### Monotonic Standards May Become Miscalibrated

The initial update rule cannot decrease `σ`.

Distribution shift may therefore create unrealistic standards and excessive retry behavior.

This is an intentional experimental condition rather than a hidden assumption.

### Evaluation Quality Bounds the Controller

The standard controller can only be as meaningful as `Q`.

If the evaluator is noisy, manipulable, or poorly aligned with actual task quality, adaptation may optimize the wrong signal.

### Higher Quality May Not Justify Higher Cost

An adaptive system may achieve stronger outcomes while consuming disproportionately more computation.

PS therefore treats quality and compute jointly.

---

## 20. What Would Falsify the Hypothesis?

PS should not be constructed so that every outcome can be interpreted as success.

Evidence against the usefulness of the mechanism would include:

- no meaningful behavioral difference from a static controller,
- increased computation without meaningful quality improvement,
- persistent threshold miscalibration,
- unstable or pathological retry behavior,
- poor generalization across task distributions,
- sensitivity so high that practical parameterization becomes unreliable.

Negative results are part of the research objective.

---

## 21. V0 Success Criteria

V0 succeeds as an experimental system if it demonstrates that:

1. the objective remains fixed during execution;
2. the performance standard is explicit and inspectable;
3. measured outcomes can update the adaptive standard;
4. the standard affects subsequent stopping decisions;
5. static and adaptive controllers can be compared under controlled conditions;
6. computational expenditure can be measured;
7. runtime decisions are auditable;
8. security and resource constraints cannot be overridden by the adaptive mechanism.

V0 does **not** require the adaptive controller to outperform the static controller.

---

## 22. Research Direction

If the initial mechanism produces meaningful behavior, subsequent work may investigate:

```text
V0
Global adaptive standard
        ↓
V1
Windowed / decaying standards
        ↓
V2
Task-conditioned standards
        ↓
V3
Uncertainty-aware capability estimation
        ↓
V4
Adaptive test-time compute control
```

A parallel security track may investigate stronger isolation, evaluator integrity, capability-based tool access, adversarial task inputs, and tamper-evident execution traces.

These directions are hypotheses and planned experiments, not implemented features.

---

## 23. Current Status

**Status: Architecture and experimental-design phase.**

The project is intentionally documenting its research assumptions, security invariants, threat boundaries, and experimental controls before implementing the runtime.

No empirical performance claims are currently made.

---

## 24. Core Principle

PS is built around one constraint:

\[
\boxed{
\text{The system may adapt how well it expects to perform,
but not what it is trying to achieve or what it is allowed to do.}
}
\]

The purpose of the project is to determine whether that distinction can form a useful basis for adaptive, measurable, and securely constrained AI-agent execution.