# APS Architecture

**Project:** Adaptive Performance Standard Agent  
**Document:** System Architecture  
**Status:** Draft  
**Version:** 0.1

---

## 1. Purpose

This document defines the initial system architecture of APS.

APS is a research-engineering system investigating adaptive performance standards for AI agents under fixed objectives and constrained authority.

The architecture has two simultaneous goals:

1. support controlled experiments on adaptive performance standards;
2. enforce security boundaries independently of model behavior.

The system is therefore separated into distinct trust and execution domains rather than implemented as a single agent process.

The primary architectural principle is:

> **The model may influence execution decisions through constrained outputs, but it does not own the runtime state that defines its objective, authority, security policy, or resource limits.**

---

# 2. System Context

At the highest level, APS contains three logical planes:

```text
┌─────────────────────────────────────────────┐
│               RESEARCH PLANE                │
│                   Python                    │
│                                             │
│ Experiments                                 │
│ Benchmarks                                  │
│ Model interaction                           │
│ Analysis                                    │
└─────────────────────┬───────────────────────┘
                      │
                 requests
                      │
                      ▼
┌─────────────────────────────────────────────┐
│                CONTROL PLANE                │
│                    Rust                     │
│                                             │
│ Objective integrity                         │
│ Performance standard                        │
│ Execution state                             │
│ Budget enforcement                          │
│ Policy / capability enforcement             │
│ Audit                                       │
└─────────────────────┬───────────────────────┘
                      │
                 authorizes
                      │
                      ▼
┌─────────────────────────────────────────────┐
│               EXECUTION PLANE               │
│                 Rust / C++                  │
│                                             │
│ Isolated execution                          │
│ Evaluator execution                         │
│ Resource enforcement                        │
│ Generated-code execution                    │
└─────────────────────┬───────────────────────┘
                      │
                      ▼
                 Environment
```

These planes represent responsibility and trust boundaries.

They do not require every component to execute as a separate process in the first implementation.

---

# 3. Architectural Principles

APS begins with several architectural rules.

## AP-001 — Model Output Is Data

LLM output is treated as untrusted input.

```text
LLM
 │
 ▼
Proposal
 │
 ▼
Validation
 │
 ▼
Authorization
 │
 ▼
Execution
```

The model does not directly execute privileged runtime operations.

---

## AP-002 — Research Code Is Not the Security Authority

The Python research plane may configure experiments and request operations.

It must not be the authoritative owner of protected execution state.

```text
Python
   │
   │ request
   ▼
Rust Runtime
   │
   │ authorize / reject
   ▼
Execution
```

---

## AP-003 — Adaptive State Is Separate From Protected State

APS distinguishes between state that may adapt and state that must remain protected.

### Adaptive State

Examples:

```text
performance standard
retry behavior
execution strategy
```

### Protected State

Examples:

```text
objective
maximum execution budget
capabilities
security policy
authority
```

Adaptive state must not grant mutation authority over protected state.

---

## AP-004 — Security Constraints Dominate Optimization

If an adaptive decision conflicts with a security or resource constraint, the constraint wins.

```text
Performance Controller
         │
         ▼
      "Retry"
         │
         ▼
   Budget Controller
      /        \
 permitted    denied
    │            │
    ▼            ▼
  Retry       Terminate
```

---

## AP-005 — Privileged State Has a Single Authoritative Owner

Protected runtime state must not have multiple independent sources of truth.

The Rust runtime is the authoritative owner of:

```text
objective
performance standard
execution state
execution budget
capabilities
runtime policy
```

Other components may receive representations of this state but cannot independently redefine the authoritative value.

---

# 4. Research Plane

The research plane is primarily implemented in Python.

Its purpose is rapid experimentation and integration with the AI/ML ecosystem.

Responsibilities include:

- experiment definition;
- benchmark selection;
- model adapters;
- model API interaction;
- experimental configuration;
- result collection;
- statistical analysis;
- visualization.

Example structure:

```text
research/
└── aps/
    ├── agents/
    ├── models/
    ├── evaluators/
    ├── experiments/
    └── analysis/
```

The research plane may request that the runtime initialize or execute an experiment.

It does not directly mutate privileged runtime state.

---

# 5. Control Plane

The control plane is the trusted core of APS.

It is primarily implemented in Rust.

The choice of Rust is motivated by:

- memory safety;
- explicit ownership;
- strong type guarantees;
- predictable native performance;
- suitability for security-sensitive systems components.

The control plane is responsible for maintaining APS runtime invariants.

Major subsystems include:

```text
runtime/
└── src/
    ├── core/
    ├── controller/
    ├── execution/
    ├── policy/
    ├── audit/
    └── error.rs
```

---

# 6. Core Runtime

The runtime core defines validated representations of security-sensitive state.

Examples include:

```text
Objective
Score
PerformanceStandard
ExecutionBudget
```

The core should prefer making invalid states difficult or impossible to represent.

For example, instead of passing arbitrary floating-point values throughout the runtime:

```text
score: f64
```

APS should use a validated type:

```text
Score
```

whose construction guarantees:

```text
0 ≤ Score ≤ 1
```

This moves validation from convention into the type system.

---

# 7. Objective Ownership

The execution objective is externally specified before an execution begins.

Conceptually:

```text
External Objective
       │
       ▼
Runtime Validation
       │
       ▼
Objective Binding
       │
       ▼
ExecutionContext
```

After binding, the objective becomes part of the protected execution context.

The runtime must not expose an operation equivalent to:

```text
set_objective(...)
```

during normal execution.

The intended invariant is:

```text
J(t+1) = J(t)
```

for the lifetime of an execution.

Future implementations may associate the objective with a canonical representation and cryptographic identity for auditability.

---

# 8. Performance Standard Ownership

The performance standard is adaptive runtime state.

It is owned by the Rust control plane.

The research plane may configure:

```text
initial standard
controller type
adaptation parameters
```

before execution.

Once execution begins, transitions are performed by the runtime controller.

For APS-V0:

```text
sigma(t+1)
=
sigma(t)
+
alpha * max(0, Q(t) - sigma(t))
```

Only validated evaluation results may enter this transition.

The model cannot directly execute operations such as:

```text
set_standard(0.1)
```

---

# 9. Standard Controller

The controller determines how the performance standard evolves.

The initial architecture supports at least two policies:

```text
StandardController
        │
        ├── StaticStandard
        │
        └── AdaptiveStandard
```

The static controller maintains:

```text
sigma(t+1) = sigma(t)
```

The adaptive controller implements the APS-V0 update rule.

Both controllers must expose the same runtime interface so that experiments can replace one with the other without modifying unrelated system behavior.

This is necessary for controlled comparison.

---

# 10. Execution State Machine

Agent execution must be represented as explicit state transitions rather than arbitrary control flow distributed throughout the codebase.

A preliminary state machine is:

```text
              ┌─────────────┐
              │ Initialized │
              └──────┬──────┘
                     │
                     ▼
              ┌─────────────┐
              │  Executing  │
              └──────┬──────┘
                     │
                     ▼
              ┌─────────────┐
              │ Evaluating  │
              └──────┬──────┘
                     │
                evaluation
                     │
              ┌──────▼──────┐
              │  Deciding   │
              └──────┬──────┘
                     │
             ┌───────┼────────┐
             │       │        │
             ▼       ▼        ▼
           Retry   Accept   Terminate
             │       │        │
             │       ▼        ▼
             │   Completed  Exhausted
             │
             └────────────► Executing
```

Exact states may change during implementation.

The important requirement is that transitions are explicit and validated.

For example:

```text
Completed → Executing
```

should not be a legal transition for the same execution.

---

# 11. Execution Budget

APS must place a hard upper bound on execution.

Possible budget dimensions include:

```text
maximum attempts
maximum model calls
maximum tool calls
maximum tokens
maximum execution time
```

APS-V0 does not necessarily need every dimension immediately.

However, at least one hard retry/attempt ceiling must exist before adaptive retry behavior is enabled.

The critical rule is:

```text
Performance Standard < Resource Boundary
```

in authority.

If:

```text
Q < sigma
```

but no execution budget remains:

```text
Terminate
```

The standard cannot authorize additional resources.

---

# 12. Policy and Capability Layer

The policy layer determines whether a requested operation is permitted.

The long-term authorization path is:

```text
Proposed Action
      │
      ▼
Schema Validation
      │
      ▼
Policy Check
      │
      ▼
Capability Check
      │
      ▼
Budget Check
      │
      ▼
Authorized Execution
```

A capability represents explicit authority to perform a class of operation.

Examples may eventually include:

```text
ModelInference
ExecuteCode
ReadWorkspaceFile
WriteWorkspaceFile
NetworkRequest
```

Possessing one capability must not imply possession of another.

APS-V0 may implement only the capabilities required by the first experiment.

---

# 13. Execution Plane

The execution plane performs operations that may involve untrusted or generated content.

Potential responsibilities include:

- generated-code execution;
- evaluator execution;
- subprocess management;
- CPU limits;
- memory limits;
- filesystem isolation;
- environment isolation;
- wall-clock timeouts.

The execution plane must not have unrestricted authority to modify APS control-plane state.

Communication should occur through narrowly defined inputs and outputs.

Conceptually:

```text
APS Runtime
    │
    │ ExecutionRequest
    ▼
Sandbox
    │
    │ ExecutionResult
    ▼
APS Runtime
```

not:

```text
Sandbox
   │
   └── directly mutates runtime state
```

---

# 14. Role of C++

C++ is not considered a security mechanism by itself.

Rust remains the preferred language for security-sensitive control logic.

C++ should be introduced only when there is a concrete requirement such as:

- integration with a native library;
- performance-sensitive execution;
- operating-system or runtime interoperability;
- an existing C/C++ isolation primitive.

Therefore:

```text
Security-sensitive control logic → Rust

Research / ML integration        → Python

Native component where justified → C++
```

APS should not increase implementation complexity merely to become a multi-language system.

---

# 15. Evaluator Boundary

Evaluation is security-sensitive because the adaptive controller depends on evaluator output.

```text
Candidate
    │
    ▼
Evaluator
    │
    ▼
Raw Evaluation Result
    │
    ▼
Validation Boundary
    │
    ▼
Validated Score
    │
    ▼
Standard Controller
```

The runtime must not trust arbitrary evaluator output.

For APS-V0, a score must satisfy:

```text
0 ≤ Q ≤ 1
```

Invalid values must be rejected before they can influence the controller.

Future work must consider evaluator manipulation and reward hacking.

---

# 16. Audit System

Important runtime transitions should produce structured audit events.

Examples include:

```text
RunStarted
ObjectiveBound
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
event_id
run_id
task_id
timestamp
event_type
objective_id
attempt
score
sigma_before
sigma_after
remaining_budget
policy_version
```

The audit subsystem should observe state transitions.

It should not determine them.

This preserves the dependency direction:

```text
Runtime State
     │
     ▼
Audit Event
```

rather than:

```text
Audit System
     │
     ▼
Runtime State
```

---

# 17. Cross-Language Boundary

APS contains multiple implementation languages.

Communication between these components must use explicit contracts.

The intended dependency direction is:

```text
Python Research Plane
         │
         ▼
      Protocol
         │
         ▼
    Rust Runtime
         │
         ▼
Execution Interface
         │
         ▼
      Sandbox
```

The exact IPC mechanism is intentionally undecided in Architecture V0.1.

Possible implementations include:

- local process IPC;
- FFI;
- Unix/domain sockets where supported;
- gRPC or another typed RPC mechanism.

The mechanism should be selected after the first runtime interface is understood.

Prematurely selecting an IPC technology would constrain the architecture before its requirements are known.

---

# 18. Dependency Direction

APS follows a one-way dependency model.

```text
Research
   │
   ▼
Protocol
   │
   ▼
Runtime
   │
   ▼
Execution
```

The following dependencies are prohibited:

```text
Runtime → Research implementation

Execution → Research implementation

Model → direct privileged runtime state

Sandbox → direct runtime state mutation
```

This keeps the trusted core independent of the experimental environment.

---

# 19. State Ownership

Initial authoritative state ownership is defined as follows:

| State | Authoritative Owner |
|---|---|
| Objective | Rust Runtime |
| Performance Standard | Rust Runtime |
| Execution State | Rust Runtime |
| Maximum Budget | Rust Runtime |
| Remaining Budget | Rust Runtime |
| Capabilities | Rust Runtime |
| Runtime Security Policy | Rust Runtime |
| Experiment Definition | Research Plane |
| Benchmark Definition | Research Plane |
| Model Configuration | Research Plane |
| Statistical Results | Research Plane |
| Untrusted Process State | Execution Plane |

State ownership should remain explicit as APS evolves.

---

# 20. Trust Boundaries

The initial trust model is:

```text
┌──────────────────────────────────────────────┐
│                 UNTRUSTED                    │
│                                              │
│ LLM output                                   │
│ generated code                               │
│ external content                             │
│ task input                                   │
│ tool output                                  │
└──────────────────────┬───────────────────────┘
                       │
                validation boundary
                       │
                       ▼
┌──────────────────────────────────────────────┐
│              TRUSTED CONTROL                 │
│                                              │
│ APS Rust Runtime                             │
│ objective state                              │
│ standard state                               │
│ budget                                       │
│ policy                                       │
│ capabilities                                 │
└──────────────────────┬───────────────────────┘
                       │
               constrained request
                       │
                       ▼
┌──────────────────────────────────────────────┐
│             ISOLATED EXECUTION               │
│                                              │
│ generated code                               │
│ evaluator processes                          │
│ native execution                             │
└──────────────────────────────────────────────┘
```

Trust is granted to components, not programming languages.

---

# 21. Failure Philosophy

APS should fail closed for security-sensitive operations.

If the runtime cannot determine whether an operation is authorized:

```text
DENY
```

If an evaluator produces an invalid score:

```text
REJECT RESULT
```

If the execution budget is exhausted:

```text
TERMINATE
```

If runtime state becomes invalid:

```text
ABORT EXECUTION
+
EMIT AUDIT EVENT
```

The system should not silently recover by weakening security constraints.

---

# 22. Initial Runtime Invariants

The architecture defines the following initial invariants.

### APS-INV-001

Objective state cannot change after execution binding.

```text
J(t+1) = J(t)
```

### APS-INV-002

Evaluation scores entering the controller satisfy:

```text
0 ≤ Q ≤ 1
```

### APS-INV-003

Performance standards satisfy:

```text
0 ≤ sigma ≤ 1
```

### APS-INV-004

Only authorized controller logic may mutate the performance standard.

### APS-INV-005

Performance requirements cannot override resource limits.

### APS-INV-006

Untrusted model output cannot directly mutate privileged runtime state.

### APS-INV-007

Adaptive performance does not imply adaptive authority.

### APS-INV-008

Illegal execution-state transitions are rejected.

These invariants should eventually correspond to automated tests.

---

# 23. Initial End-to-End Flow

The first complete APS execution should eventually follow:

```text
1. Research plane defines experiment
                 │
                 ▼
2. Runtime receives execution request
                 │
                 ▼
3. Runtime validates configuration
                 │
                 ▼
4. Objective is bound
                 │
                 ▼
5. Budget is established
                 │
                 ▼
6. Performance standard is initialized
                 │
                 ▼
7. Candidate is generated
                 │
                 ▼
8. Candidate is evaluated
                 │
                 ▼
9. Evaluation result is validated
                 │
                 ▼
10. Runtime compares Q with sigma
                 │
          ┌──────┴──────┐
          │             │
          ▼             ▼
       Q >= sigma     Q < sigma
          │             │
          ▼             ▼
       Accept       Check Budget
                        │
                   ┌────┴────┐
                   ▼         ▼
                 Retry    Terminate
                   │
                   ▼
              Next Attempt
```

After an authorized evaluation, the adaptive controller may update the performance standard according to its configured policy.

---

# 24. Architecture V0.1 Scope

Architecture V0.1 defines:

- system boundaries;
- trust boundaries;
- state ownership;
- dependency direction;
- initial runtime invariants;
- responsibility of each implementation language;
- high-level execution flow.

It intentionally does not yet define:

- final IPC technology;
- final sandbox implementation;
- deployment topology;
- database architecture;
- distributed execution;
- cloud infrastructure;
- production observability stack;
- final capability representation.

Those decisions require evidence from the first implementation.

---

# 25. Architectural Success Condition

The architecture is successful if APS can evolve experimentally without violating the following principle:

```text
        PERFORMANCE MAY ADAPT

                 │
                 ▼

        EXECUTION MAY CHANGE

                 │
                 ▼

     AUTHORITY MUST REMAIN BOUNDED
```

In other words:

> **APS may change how it executes in pursuit of an objective, but adaptation must not independently change the objective, authority, security policy, or maximum resource boundary governing that execution.**