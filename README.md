#Enterprise Native Desktop Monorepo Foundation

## 1. Role & Mission

You are a Principal Software Architect, Senior Rust Engineer, Senior macOS Engineer, Senior Windows Engineer, Senior Linux Engineer, Design System Architect, and DevOps Engineer.

Your mission is to analyze, design, scaffold, implement, and validate a long-term maintainable, enterprise-oriented native desktop monorepo.

The monorepo must support three genuinely native desktop applications:

- macOS using Swift and SwiftUI.
- Windows using Rust and native WinUI 3.
- Linux using Rust, GTK 4, and libadwaita.

All three applications must share one authoritative Rust Core.

The architecture must prioritize native user interfaces, shared Rust application logic, explicit ownership boundaries, maintainability, security, performance, reproducibility, and cross-platform consistency.

### 1.1 Technology Stack

**Shared Core**
- Rust stable.
- Cargo Workspace.
- Modular Rust crates.
- Explicit application contracts.
- Shared domain and application logic.
- Platform-independent application behavior.

**Interoperability**
- UniFFI for macOS Swift interoperability.
- Direct Rust crate dependencies for Windows.
- Direct Rust crate dependencies for Linux.
- No unnecessary cross-language FFI for Rust-native platforms.

**macOS**
- Swift.
- SwiftUI.
- AppKit where required.
- Xcode.
- UniFFI.
- Native macOS application lifecycle.

**Windows**
- Rust only.
- WinUI 3.
- windows-reactor from Microsoft windows-rs.
- windows-rs for native Windows API access.
- Windows App SDK.
- Native WinRT/Windows API integration.
- Native Windows application lifecycle.
- No C# application layer.
- No .NET application runtime dependency introduced by our own application code.
- No XAML-based application layer required by the selected Rust UI architecture.

**Linux**
- Rust.
- GTK 4.
- libadwaita.
- gtk-rs.
- First-class native Wayland support.
- X11 compatibility.
- XDG Desktop Portal integration where appropriate.
- Native Linux application lifecycle.

**Shared Design System**
- Platform-neutral design tokens.
- Shared semantic visual definitions.
- Platform-specific native theme mapping.
- Light and dark appearance.
- Native accessibility integration.

**Shared Resources**
- Configuration.
- Localization.
- Brand assets.
- Shared specifications.
- Contract definitions.

**Automation**
- GitHub Actions.
- Cargo tooling.
- Platform-specific build automation.
- Deterministic code generation.
- Architecture compliance checks.

**Documentation and Governance**
- Markdown.
- Architecture Decision Records.
- AGENTS.md.
- Enforceable development rules.
- Ownership matrix.
- Dependency boundaries.

Use compatible stable versions of all technologies.

Verify current availability, maintenance status, compatibility, target architecture support, and operating-system requirements before selecting and pinning versions.

Prefer supported stable components when available.

Explicitly identify experimental components.

### 1.2 Primary Objective

**One shared Rust Core, three genuinely native desktop applications, one shared design language, explicit ownership boundaries, and no duplicated authoritative implementations.**

The architecture must satisfy these permanent requirements:

1. Rust owns shared application behavior.
2. macOS uses genuine native SwiftUI.
3. Windows uses genuine native WinUI 3 from Rust.
4. Linux uses genuine native GTK 4 and libadwaita.
5. All three applications consume the same Rust Core.
6. Windows and Linux depend on Rust Core directly.
7. macOS communicates with Rust through UniFFI.
8. Each platform preserves native appearance and interaction.
9. Shared visual definitions have one authoritative source.
10. Platform-specific behavior remains isolated.
11. Future functionality can be added without redesigning the fundamental architecture.

### 1.3 Initial Implementation Scope

This is a **foundation-only implementation**.

Do not implement:

- Business-specific features.
- Authentication.
- Database access.
- REST API clients.
- Offline synchronization.
- Background services.
- Image processing.
- Enterprise application modules.
- Other application-specific services.

Future functionality will be introduced through separate architectural analysis, PRDs, and implementation plans.

The initial deliverable must be a minimal, functional monorepo demonstrating the same Rust function being invoked by all three native desktop applications.

---

## 2. Permanent Architectural Principles

### 2.1 Single Source of Truth

Every shared responsibility must have exactly one authoritative owner.

| Responsibility | Authoritative Owner |
|---|---|
| Shared application behavior | Rust Core |
| Domain and business rules | Rust Core |
| Shared validation | Rust Core |
| Shared algorithms | Rust Core |
| Shared application contracts | Rust Core |
| Shared error semantics | Rust Core |
| macOS presentation | Swift + SwiftUI |
| Windows presentation | Rust + windows-reactor |
| Linux presentation | Rust + GTK 4 |
| macOS platform integration | Swift / AppKit |
| Windows platform integration | Rust / windows-rs |
| Linux platform integration | Rust / GTK / Linux APIs |
| macOS cross-language interoperability | UniFFI |
| Windows Core integration | Direct Rust dependency |
| Linux Core integration | Direct Rust dependency |
| Shared visual definitions | Design System |
| Shared configuration | Shared Resources |
| Shared localization | Shared Resources |
| Shared brand assets | Shared Resources |
| Code generation | Shared Tooling |
| Architecture policies | docs/ |
| Agent governance | AGENTS.md |

Do not duplicate authoritative shared application behavior across platforms.

Platform-specific presentation logic is permitted.

Platform-specific operating-system behavior is permitted.

Native presentation layers must not become competing implementations of shared Rust functionality.

### 2.2 Folder Structure Defines Architecture

Repository structure is an architectural contract.

Folders, crates, modules, projects, and namespaces must clearly define:

- Ownership.
- Responsibility.
- Public interfaces.
- Private implementations.
- Dependency direction.
- Testing responsibilities.
- Extension policies.

Every coding agent must respect these boundaries.

Avoid circular dependencies, undocumented cross-layer access, and overlapping responsibility.

### 2.3 Rust-First Shared Architecture

Rust is the authoritative implementation language for shared application behavior.

Windows and Linux must consume Rust Core directly.

Do not introduce additional programming languages or interoperability layers where Rust already provides a suitable implementation.

macOS remains an intentional exception because SwiftUI requires Swift-based native presentation.

### 2.4 Native-First Presentation

Every desktop platform must use its genuine native rendering system.

Do not replace native presentation with:

- Electron.
- Tauri.
- Browser-based application shells.
- WebView-based UI.
- Cross-platform UI emulation.

Platform-native appearance, accessibility, window management, and interaction must be preserved.

### 2.5 Contract-First Development

All shared functionality must be exposed through explicit, testable Rust contracts.

Do not expose internal implementation details unnecessarily.

Generated bindings must never be edited manually.

### 2.6 Minimalism and Future Extensibility

Build only what is necessary for a functional foundation.

Do not create speculative services, unused crates, unnecessary abstractions, or placeholder implementations.

Prefer a small working implementation over a large incomplete scaffold.

---

## 3. Architecture Overview

Establish the following logical architecture:

```text
                    DESKTOP MONOREPO
                           |
            +--------------+--------------+
            |              |              |
          macOS          Windows         Linux
            |              |              |
         SwiftUI         WinUI 3         GTK 4
            |              |          libadwaita
          Swift            Rust           Rust
            |              |              |
       Presentation    Presentation   Presentation
            |              |              |
         UniFFI        Direct Rust    Direct Rust
            |              |              |
            +--------------+--------------+
                           |
                     RUST CORE
                           |
                 Shared Application
                       Logic
```

### 3.1 Shared Core Dependencies

The Rust Core must not depend on:

- SwiftUI.
- AppKit.
- WinUI 3.
- windows-reactor.
- Windows App SDK.
- GTK.
- libadwaita.
- Platform-specific presentation frameworks.

Native applications may depend on Rust Core.

Rust Core must never depend on native applications.

### 3.2 Platform Independence

The Core must be independently buildable and testable without requiring the native desktop UI projects.

Platform integration dependencies must remain inside their corresponding application or adapter layers.

---

## 4. Repository Structure

Create the following architectural baseline:

```text
desktop-monorepo/
│
├── apps/
│   │
│   ├── macos/
│   │   ├── App/
│   │   ├── Presentation/
│   │   ├── DesignSystem/
│   │   ├── Platform/
│   │   ├── Interop/
│   │   └── Resources/
│   │
│   ├── windows/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── app/
│   │   │   ├── presentation/
│   │   │   ├── designsystem/
│   │   │   └── platform/
│   │   └── resources/
│   │
│   └── linux/
│       ├── Cargo.toml
│       ├── src/
│       │   ├── main.rs
│       │   ├── app/
│       │   ├── presentation/
│       │   ├── designsystem/
│       │   └── platform/
│       └── resources/
│
├── crates/
│   ├── core/
│   └── macos-ffi/
│
├── design-system/
│   ├── tokens/
│   │   ├── colors.json
│   │   ├── typography.json
│   │   ├── spacing.json
│   │   ├── radius.json
│   │   └── motion.json
│   ├── schemas/
│   └── README.md
│
├── shared/
│   ├── config/
│   ├── localization/
│   ├── assets/
│   └── specifications/
│
├── tooling/
│   ├── codegen/
│   └── validation/
│
├── bindings/
│   └── swift/
│
├── tests/
│   ├── fixtures/
│   └── contracts/
│
├── scripts/
│   ├── build-rust.sh
│   ├── build-macos.sh
│   ├── build-windows.ps1
│   ├── build-linux.sh
│   ├── generate-bindings.sh
│   ├── generate-design-tokens.sh
│   ├── validate.sh
│   └── test.sh
│
├── docs/
│   ├── architecture/
│   │   ├── overview.md
│   │   ├── boundaries.md
│   │   ├── dependency-rules.md
│   │   ├── repository-structure.md
│   │   ├── ownership-matrix.md
│   │   └── interoperability.md
│   │
│   ├── rules/
│   │   ├── development-rules.md
│   │   ├── service-rules.md
│   │   ├── rust-rules.md
│   │   ├── macos-rules.md
│   │   ├── windows-rules.md
│   │   ├── linux-rules.md
│   │   ├── design-system-rules.md
│   │   ├── shared-resources-rules.md
│   │   └── anti-duplication.md
│   │
│   ├── design-system/
│   │   ├── principles.md
│   │   ├── token-conventions.md
│   │   ├── platform-mapping.md
│   │   ├── component-guidelines.md
│   │   └── accessibility.md
│   │
│   ├── contracts/
│   │   ├── rust-api-conventions.md
│   │   ├── ffi-conventions.md
│   │   └── compatibility.md
│   │
│   ├── platforms/
│   │   ├── macos.md
│   │   ├── windows.md
│   │   ├── windows-reactor-evaluation.md
│   │   └── linux-wayland.md
│   │
│   ├── quality/
│   │   ├── architecture-compliance.md
│   │   └── testing.md
│   │
│   ├── decisions/
│   │   └── ADR-0001-foundation.md
│   │
│   └── plans/
│       └── README.md
│
├── .github/
│   └── workflows/
│
├── AGENTS.md
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── .editorconfig
├── .gitignore
└── README.md
```

### Repository Rules

This structure is an architectural baseline.

Do not create empty modules solely to reproduce the directory tree.

Every implemented module must have a real responsibility.

The Cargo Workspace must include:

- Shared Rust Core.
- macOS UniFFI bridge crate.
- Windows native Rust application.
- Linux native Rust application.

Use platform-specific Cargo dependencies where necessary.

Do not place Windows or Linux UI dependencies inside Rust Core.

Do not create a separate Windows FFI crate merely to connect Rust to Rust.

Do not create a separate Linux FFI crate merely to connect Rust to Rust.

The macOS FFI crate must remain thin and depend on Rust Core.

---

## 5. Shared Rust Core

Create a minimal Rust Cargo Workspace.

Implement one shared function:

```rust
pub fn hello(name: String) -> String {
    format!("Hello, {name}!")
}
```

This must be the only authoritative implementation of the demonstration behavior.

The same function must be called by:

1. macOS SwiftUI through UniFFI.
2. Windows WinUI 3 through direct Rust dependency.
3. Linux GTK 4 through direct Rust dependency.

Do not duplicate the greeting algorithm in Swift, Windows presentation code, or Linux presentation code.

### Core Design Rules

- Use safe Rust by default.
- Prefer clear ownership.
- Keep Core independent of operating systems.
- Avoid global mutable state.
- Use explicit public interfaces.
- Keep dependencies minimal.
- Provide unit tests.
- Avoid unnecessary allocations.
- Avoid unnecessary asynchronous runtimes.
- Do not create speculative domain models.

The Core must compile and run its unit tests independently from the native application projects.

### Future Core Responsibilities

Document ownership conventions for future:

- Domain entities.
- Business rules.
- Application use cases.
- Shared validation.
- Error semantics.
- API orchestration.
- Persistence orchestration.
- Synchronization algorithms.
- Processing algorithms.

These are ownership definitions, not instructions to implement the features now.

---

## 6. macOS Native Architecture

Use SwiftUI as the primary presentation framework.

Use AppKit only when necessary to access macOS-native functionality not adequately exposed through SwiftUI.

The application must be genuinely native.

Do not use Mac Catalyst as a substitute.

### Architecture

```text
SwiftUI View
     |
Presentation State
     |
Swift ViewModel
     |
Rust Adapter
     |
UniFFI
     |
macos-ffi
     |
Rust Core
```

### macOS Responsibilities

Swift owns:

- Application lifecycle.
- Windows.
- Navigation.
- Presentation state.
- Menu commands.
- Keyboard shortcuts.
- Native dialogs.
- Accessibility.
- Platform permissions.
- macOS-specific integrations.
- Native animations.

Rust owns shared application behavior.

### Interoperability

Use UniFFI to generate Swift bindings.

Rules:

1. Rust Core owns the implementation.
2. The macOS FFI crate exposes appropriate contracts.
3. UniFFI generates Swift bindings.
4. Generated Swift bindings must not be edited manually.
5. Swift presentation must use generated interfaces.
6. Memory and resource ownership must be explicit.
7. Error mapping must be documented.
8. Blocking operations must not run on the main UI thread.

### Foundation UI

Create a minimal native SwiftUI window with:

- Application title.
- Name input.
- Action button.
- Output label.
- Shared semantic theme.
- Light and dark support.

The output must originate from Rust Core.

Follow Apple Human Interface Guidelines.

Preserve native window management, menu bar behavior, keyboard focus, and accessibility.

---

## 7. Windows Native Architecture — Rust Only

Use Rust as the exclusive application implementation language for Windows.

Use native WinUI 3 as the presentation framework.

Use `windows-reactor` from Microsoft `windows-rs` as the initial Rust UI integration approach.

Use `windows-rs` to access native Windows APIs where appropriate.

### 7.1 Mandatory Windows Stack

- Rust.
- WinUI 3.
- windows-reactor.
- windows-rs.
- Windows App SDK.
- Cargo.

Do not introduce:

- C# application code.
- .NET application services.
- C# ViewModels.
- C# interoperability adapters.
- XAML code-behind as the application architecture.
- WPF.
- WinForms.
- Electron.
- Tauri.
- WebView-based application UI.

Native Windows App SDK and WinUI runtime dependencies required by windows-reactor are permitted.

The restriction on .NET applies to the application's own implementation; it does not prohibit required native Windows runtime components.

### 7.2 Windows Architecture

```text
Native WinUI 3 Controls
          |
      windows-reactor
          |
    Rust Presentation
          |
   Rust Application UI
          |
   Direct Cargo Import
          |
       Rust Core
```

There must be no custom cross-language FFI layer between Windows presentation and Rust Core.

The Windows application must consume Core through an ordinary Rust crate dependency.

### 7.3 Windows Ownership

Windows presentation owns:

- Native windows.
- UI controls.
- Navigation.
- Presentation state.
- Typed UI messages.
- Event handling.
- Keyboard shortcuts.
- Accessibility.
- Fluent Design integration.
- System appearance.
- Windows-specific interactions.

Windows platform adapters own:

- Native Windows APIs.
- Windows permissions.
- Window lifecycle.
- OS integrations.
- Platform-specific resource handling.

Rust Core owns shared application behavior.

### 7.4 windows-reactor Development Rules

Use the supported declarative component model provided by the selected windows-reactor version.

Prefer:

- Typed components.
- Typed messages.
- Explicit state.
- Native WinUI controls.
- Clear event handling.
- Small presentation components.
- Platform-specific adapters.

Do not introduce a separate C#-style MVVM framework merely to imitate the previous architecture.

Use the state and update model appropriate to windows-reactor.

Avoid duplicating state ownership unnecessarily.

Separate presentation behavior from shared application behavior.

### 7.5 Experimental Technology Validation

windows-reactor must be treated as an experimental dependency until evidence demonstrates adequate stability for the project's requirements.

Before committing to the complete Windows foundation, implement a small proof of concept.

Validate:

1. Rust application startup.
2. Native WinUI 3 window creation.
3. Native text input.
4. Native button.
5. Native label.
6. Typed UI events.
7. Component state updates.
8. Direct Rust Core invocation.
9. Light appearance.
10. Dark appearance.
11. Native resource loading.
12. Application shutdown.
13. Windows App SDK runtime compatibility.
14. x64 compilation.
15. ARM64 feasibility.
16. Windows native dependency setup.
17. Supported Rust compiler compatibility.

Document the actual results in:

`docs/platforms/windows-reactor-evaluation.md`

### 7.6 Go / No-Go Decision

Proceed with windows-reactor only when the minimum foundation capabilities work in the selected toolchain.

If a required capability is unavailable:

1. Identify the exact limitation.
2. Verify whether windows-rs exposes the necessary native API.
3. Determine whether a thin Rust-native adapter is sufficient.
4. Document the technical impact.
5. Record the decision in an ADR.

Do not silently switch to C#.

Do not silently introduce .NET.

Do not replace WinUI 3 with another framework without explicit architectural approval.

If the Rust-only WinUI approach cannot be validated, report Windows as blocked or unverified rather than presenting a different architecture as complete.

### 7.7 Windows App SDK Compatibility

Verify the compatibility between:

- windows-reactor.
- windows-rs.
- Windows App SDK.
- Required Windows runtime components.
- Selected Rust toolchain.
- Supported Windows versions.
- Target CPU architectures.

Do not assume the latest Windows App SDK is automatically compatible with the latest windows-reactor release.

Use a verified compatible version set.

Document the compatibility matrix.

Account for metadata and runtime version requirements imposed by windows-reactor.

### 7.8 Foundation UI

Create a native WinUI 3 window containing:

- Shared application title.
- Native name input.
- Native button.
- Result label.
- Shared semantic theme.
- Light and dark support.

When activated, the action must call the shared Rust Core function directly.

Follow Windows Fluent Design conventions.

Preserve native keyboard handling, scaling, focus behavior, and accessibility.

---

## 8. Linux Native Architecture

Use Rust, GTK 4, and libadwaita.

The application must run natively under Wayland.

Support compatible X11 desktop environments.

Do not use a WebView-based application shell.

### 8.1 Linux Architecture

```text
GTK 4 / libadwaita
         |
Rust Presentation
         |
Rust Application State
         |
Direct Cargo Import
         |
      Rust Core
```

No custom FFI is required between the Linux application and Rust Core.

### 8.2 Linux Ownership

Linux presentation owns:

- GTK application lifecycle.
- GTK widgets.
- libadwaita application windows.
- Navigation.
- Presentation state.
- Keyboard shortcuts.
- Accessibility.
- Theme integration.
- Native user interaction.

Linux platform adapters own:

- Linux-specific APIs.
- Desktop portal integration.
- Native filesystem integration.
- Desktop notifications.
- Window-system-specific integrations.
- Application activation.

Rust Core owns shared application behavior.

### 8.3 Wayland Requirements

Treat Wayland as a first-class deployment target.

The application must:

1. Use native GTK Wayland support.
2. Run without requiring XWayland in normal Wayland sessions.
3. Preserve compatible X11 support.
4. Avoid X11-specific assumptions.
5. Respect compositor-controlled window placement.
6. Respect native scaling.
7. Support HiDPI displays.
8. Use GTK-compatible clipboard abstractions.
9. Use GTK-compatible input handling.
10. Avoid unsupported global window-coordinate assumptions.

Do not force a Wayland-only backend configuration that unnecessarily disables X11 compatibility.

### 8.4 Desktop Environment Compatibility

Target compatible Linux desktop environments, including:

- GNOME.
- KDE Plasma.
- Other GTK-compatible desktop environments.

libadwaita is the selected visual language for the Linux application.

Do not assume that every Linux desktop environment provides identical visual decorations or desktop services.

### 8.5 XDG Desktop Portals

Establish conventions for future use of desktop portals for capabilities such as:

- File selection.
- Opening URIs.
- Notifications.
- Screen sharing.
- Desktop permissions.

Do not implement these services during the foundation phase.

### 8.6 Foundation UI

Create a native GTK 4/libadwaita application window containing:

- Shared application title.
- Name input.
- Native button.
- Output label.
- Shared semantic theme.
- Light and dark support.

The output must originate from the shared Rust Core.

Do not duplicate the implementation inside Linux presentation.

---

## 9. Shared Rust Contract Architecture

Establish one authoritative Rust application API.

Use the same conceptual contracts across all three platforms.

### 9.1 Contract Ownership

Rust Core owns:

- Shared public interfaces.
- Shared DTOs.
- Shared error semantics.
- Business validation.
- Shared algorithms.
- Behavioral expectations.

### 9.2 Native Consumption

macOS consumes appropriately exposed contracts through UniFFI.

Windows consumes Rust Core public APIs directly.

Linux consumes Rust Core public APIs directly.

### 9.3 Contract Rules

Define conventions for:

- Public types.
- Request and response types.
- Typed errors.
- Nullability.
- Compatibility.
- Versioning.
- Async operations.
- Cancellation.
- Resource ownership.
- Thread safety.
- Native platform capability interfaces.

Do not create speculative complex DTOs during the foundation phase.

### 9.4 FFI Boundary Isolation

UniFFI must remain isolated inside the macOS interoperability boundary.

Do not introduce UniFFI dependencies into the shared domain implementation unless technically required and architecturally justified.

Do not force Windows and Linux to depend on the macOS FFI crate.

Generated Swift bindings must never become the authoritative contract source.

---

## 10. Shared Design System

Implement the permanent principle:

**Shared Design Language, Native UI Implementation.**

The Design System owns canonical visual definitions.

Each platform renders them through its native UI framework.

### 10.1 Canonical Design Tokens

Use one platform-neutral, version-controlled token source.

Define:

- Brand colors.
- Primary colors.
- Secondary colors.
- Background.
- Surface.
- Text.
- Borders.
- Success.
- Warning.
- Error.
- Typography roles.
- Font weights.
- Spacing.
- Corner radius.
- Motion semantics.

Distinguish primitive tokens from semantic aliases.

Support light and dark appearance.

### 10.2 Token Generation

Establish deterministic token generation:

```text
              CANONICAL DESIGN TOKENS
                         |
                   JSON Sources
                         |
                  Token Generator
                         |
          +--------------+--------------+
          |              |              |
          v              v              v
     Swift Tokens    Rust Tokens    GTK Resources
          |              |              |
          v              v              v
     SwiftUI Theme   WinUI 3 Theme   GTK Theme
          |              |              |
         macOS         Windows         Linux
```

The Windows design system must generate or reproducibly map design values into Rust-compatible token definitions.

Do not assume XAML generation is required by windows-reactor.

Use the supported native styling and theming APIs of the selected windows-reactor version.

For Linux, use GTK-compatible CSS and Rust-side token definitions where appropriate.

For macOS, use Swift-compatible token definitions.

### 10.3 Design System Ownership

Canonical tokens must not be independently maintained inside platform repositories.

Do not duplicate shared brand constants.

Do not manually edit generated token definitions.

### 10.4 Native UI Rules

**macOS**
- Follow Apple Human Interface Guidelines.
- Preserve native SwiftUI controls.
- Preserve native macOS window behavior.
- Respect accessibility and text scaling.

**Windows**
- Follow Fluent Design.
- Preserve native WinUI 3 controls.
- Respect system theme and accessibility.
- Support high-contrast requirements where technically possible.

**Linux**
- Follow GTK/libadwaita conventions.
- Respect native GTK controls.
- Preserve Wayland-compatible interaction.
- Respect accessibility and desktop appearance settings.

### 10.5 Cross-Platform Consistency

Cross-platform consistency means:

- Consistent brand identity.
- Consistent terminology.
- Consistent visual semantics.
- Consistent business behavior.
- Consistent contracts.
- Consistent resource ownership.

It does not require pixel-identical controls or navigation patterns.

### 10.6 Foundation Validation

The Hello World screen must demonstrate:

1. Shared semantic colors.
2. Shared typography definitions.
3. Shared spacing definitions.
4. Light appearance.
5. Dark appearance.
6. Native controls.
7. Native accessibility behavior.

Do not create a large reusable component library during this phase.

---

## 11. Shared Configuration

Establish a canonical configuration convention.

Distinguish:

- Build-time configuration.
- Runtime configuration.
- Environment definitions.
- Public application settings.
- Platform-specific settings.
- Future feature flags.

Use a shared schema for values that must remain consistent across platforms.

Platform-specific settings may remain native.

Never store secrets, signing credentials, private keys, or sensitive configuration in shared public resources.

Do not implement a remote configuration service during foundation setup.

Document how future configurations will be validated, versioned, and consumed.

---

## 12. Shared Localization

Establish a canonical source for shared localization content.

Define:

- Stable localization keys.
- Locale naming conventions.
- Fallback behavior.
- Placeholder validation.
- Missing translation detection.
- Shared terminology.
- Platform resource generation.

### Platform Mapping

macOS uses native Apple localization resources.

Windows uses a Rust-compatible localization adapter integrated with native presentation.

Linux uses GTK-compatible localization resources.

Preserve native locale formatting and pluralization conventions.

Do not independently maintain duplicate translation catalogs.

Implement only the minimal localization example necessary for the foundation.

Do not create a full translation management platform.

---

## 13. Shared Assets

Establish conventions for canonical brand assets.

Define:

- Asset naming.
- Canonical source formats.
- Platform-specific exports.
- Light and dark variants.
- Resolution requirements.
- Optimization.
- Versioning.
- Ownership.

### Platform Packaging

macOS assets must support native application bundles.

Windows assets must support native WinUI application resources.

Linux assets must follow appropriate desktop application icon conventions.

Avoid manually maintained duplicate brand assets.

Native operating-system icons may differ according to platform conventions.

Do not introduce unnecessary artwork during foundation setup.

---

## 14. Native Application Lifecycle

Document platform-specific lifecycle conventions.

### macOS

Respect:

- Application launch.
- Native windows.
- Menu bar behavior.
- Application activation.
- Termination.
- System appearance.

### Windows

Respect:

- Native Windows application startup.
- WinUI window activation.
- windows-reactor component lifecycle.
- Native Windows event handling.
- Process termination.
- Windows App SDK runtime initialization.

### Linux

Respect:

- GTK application lifecycle.
- Desktop activation.
- Wayland compositor behavior.
- X11 compatibility.
- Clean application shutdown.

### Shared Core

Rust Core must not assume:

- A single permanent window.
- A particular operating system.
- A particular UI framework.
- A particular desktop environment.
- One fixed application entry point.

Do not create complex lifecycle management infrastructure during the initial foundation.

---

## 15. Performance and Resource Management

Establish performance-oriented architectural principles.

### Shared Rust Core

- Prefer safe Rust.
- Minimize unnecessary allocations.
- Avoid unnecessary cloning.
- Use explicit ownership.
- Keep APIs predictable.
- Avoid unnecessary dependencies.
- Use safe concurrency patterns.

### Native Presentation

Keep UI operations responsive.

Avoid blocking UI threads.

Respect native event loops.

Release resources correctly.

Avoid unnecessary redraws and state duplication.

### macOS

Respect SwiftUI and AppKit main-thread requirements.

### Windows

Respect WinUI and windows-reactor UI-thread requirements.

Do not perform blocking shared operations in UI event handlers when those operations may become expensive.

### Linux

Respect GTK main-context requirements.

Do not invoke GTK UI operations from arbitrary worker threads.

### Future Concurrency

Document conventions for asynchronous operations and cancellation without introducing unnecessary runtimes or task frameworks during the Hello World phase.

---

## 16. Security and Platform Boundaries

Establish basic security principles.

### Rust Core

- Prefer safe Rust.
- Limit unsafe code.
- Document unsafe invariants.
- Validate boundary inputs.
- Avoid exposing private internals.
- Use structured error semantics.

### macOS

- Respect platform permissions.
- Document future code-signing requirements.
- Keep signing credentials outside source control.
- Document sandboxing and notarization expectations.

### Windows

- Respect Windows permissions.
- Avoid unnecessary elevation.
- Use trusted native dependencies.
- Follow Windows runtime deployment requirements.
- Avoid unnecessary dynamic library loading.

### Linux

- Respect filesystem permissions.
- Avoid unnecessary privileges.
- Prefer portal-based integration where appropriate.
- Document future Flatpak sandbox requirements.

Do not implement authentication, encryption services, or complex security infrastructure during the foundation phase.

---

## 17. Shared Tooling and Code Generation

Establish deterministic tooling for:

- Rust builds.
- Rust tests.
- UniFFI Swift bindings.
- Design token generation.
- Shared resources.
- Schema validation.
- Architecture compliance.
- Platform build automation.

### Generated Code Rules

Generated code must:

1. Have clear ownership.
2. Be reproducible.
3. Never be manually edited.
4. Be independently validated.
5. Remain separate from authoritative source files.

Do not generate unnecessary Windows or Linux FFI bindings.

Windows and Linux must use direct Rust dependencies.

Pin tool versions where appropriate.

Do not create a complex custom code-generation framework when lightweight established tooling is sufficient.

---

## 18. Architecture Compliance

Implement lightweight automated checks for architectural consistency.

Checks should cover, where technically practical:

1. Rust formatting.
2. Rust linting.
3. Rust unit tests.
4. Cargo Workspace dependency direction.
5. Prohibited Core dependencies.
6. UniFFI generation consistency.
7. Windows direct Core integration.
8. Linux direct Core integration.
9. Absence of unnecessary Windows FFI.
10. Absence of unnecessary Linux FFI.
11. Design token schema validation.
12. Generated token consistency.
13. Shared configuration validation.
14. Localization consistency.
15. Generated file policies.
16. Required documentation.
17. macOS build integration.
18. Windows build integration.
19. Linux build integration.
20. Platform isolation.

### Prohibited Dependency Rules

The Rust Core must not directly depend on:

- windows-reactor.
- WinUI 3.
- GTK 4.
- libadwaita.
- SwiftUI.
- AppKit.

Windows must not depend on macOS FFI.

Linux must not depend on macOS FFI.

Platform-specific UI crates must not become transitive Core dependencies.

Document checks in:

`docs/quality/architecture-compliance.md`

Distinguish automated checks from rules requiring human architectural review.

Do not create an oversized governance framework.

---

## 19. Agent Governance

Create a root `AGENTS.md`.

It must be concise, authoritative, and suitable for future coding agents.

### Before Implementation

Every agent must:

1. Inspect the repository.
2. Read relevant documentation.
3. Identify the responsible architectural layer.
4. Check existing modules.
5. Inspect shared contracts.
6. Check for duplicate responsibilities.
7. Identify permitted dependencies.
8. Determine testing requirements.
9. Review relevant ADRs.

### During Implementation

Every agent must:

1. Follow ownership boundaries.
2. Respect dependency direction.
3. Reuse shared Rust functionality.
4. Keep native UI code platform-specific.
5. Use shared design tokens.
6. Use shared resources.
7. Keep generated code isolated.
8. Avoid speculative abstractions.
9. Preserve native platform behavior.
10. Update documentation when architecture changes.

### After Implementation

Every agent must:

1. Run relevant tests.
2. Run architecture checks.
3. Validate generated artifacts.
4. Verify affected platform builds where possible.
5. Document actual results.
6. Report limitations accurately.
7. Document architectural changes.

### Multi-Agent Collaboration

Agents working on different platforms must not independently redefine:

- Shared Rust contracts.
- Shared domain behavior.
- Design tokens.
- Shared configuration schemas.
- Shared localization keys.
- Shared resource ownership.

Shared changes must be evaluated against every affected platform.

Platform-specific agents may implement native adapters without changing shared Core behavior.

Material architectural changes require ADR documentation.

`AGENTS.md` must link to detailed documentation instead of duplicating its entire contents.

---

## 20. Service and Module Rules

Define development conventions for future services without implementing application services yet.

Future services must have:

- One clear responsibility.
- One authoritative owner.
- Explicit public interfaces.
- Defined dependency boundaries.
- Structured errors.
- Testable behavior.
- Documented lifecycle expectations.

Do not allow services to access another service's private implementation directly.

Avoid circular dependencies.

Avoid overlapping service ownership.

Before creating a new crate, determine whether an existing crate or module can appropriately own the functionality.

Do not create service scaffolding merely for anticipated features.

---

## 21. Testing Strategy

Implement foundation-level tests.

### Rust Core

- Unit tests.
- Formatting checks.
- Lint checks.
- Public API tests.
- Basic contract tests.

### macOS

- Swift binding generation.
- Swift binding integration.
- Rust Hello World invocation.
- Basic SwiftUI smoke test where feasible.
- Shared token integration.

### Windows

- windows-reactor startup validation.
- Native WinUI window creation.
- Typed UI event handling.
- Rust Core invocation.
- Shared token integration.
- Windows runtime compatibility.
- Basic native UI smoke test where feasible.

### Linux

- Cargo integration.
- Rust Core invocation.
- GTK application startup.
- Shared token integration.
- Wayland validation where available.
- X11 compatibility validation where available.

### Shared Resources

- Token schema validation.
- Token generation consistency.
- Localization validation.
- Configuration schema validation where implemented.

### Architecture

- Dependency boundary checks.
- Generated-code consistency checks.
- Documentation checks.
- Platform isolation checks.

Do not introduce complex end-to-end test infrastructure for features that do not yet exist.

---

## 22. Build System and Target Architectures

Provide reproducible commands for:

- Building Rust Core.
- Testing Rust Core.
- Generating UniFFI bindings.
- Building Rust libraries for macOS.
- Building macOS SwiftUI.
- Building Windows Rust/WinUI 3.
- Building Linux Rust/GTK.
- Generating design tokens.
- Validating shared resources.
- Running foundation tests.

### macOS

Plan for:

- Apple Silicon ARM64.
- Intel x86_64 where supported.

Use Xcode and compatible native Rust targets.

Document the minimum supported macOS version.

### Windows

Plan for:

- x86_64.
- ARM64 where supported.

Use a verified Windows Rust target and compatible Windows App SDK runtime.

Document windows-reactor target limitations.

Do not assume ARM64 support without verification.

### Linux

Plan for:

- x86_64.
- aarch64 where supported.

Document minimum GTK 4 and libadwaita requirements.

Document required native system libraries.

### Build Host Rules

Use macOS for native SwiftUI builds.

Use Windows for native WinUI builds.

Use Linux for native GTK builds.

Do not claim unsupported cross-compilation workflows are functional without actual verification.

Keep build artifacts separate from authoritative source files.

---

## 23. CI/CD Foundation

Create GitHub Actions workflows for:

- Rust formatting.
- Rust linting.
- Rust tests.
- UniFFI generation validation.
- Design token validation.
- Shared resource validation.
- Architecture compliance.
- macOS build verification.
- Windows build verification.
- Linux build verification.

### Runner Requirements

Use macOS runners for SwiftUI and Xcode.

Use Windows runners for Rust-based WinUI 3.

Use Linux runners for GTK 4 and libadwaita.

### Windows CI Requirements

The Windows CI workflow must validate:

- Rust toolchain.
- windows-reactor dependency resolution.
- Windows App SDK compatibility.
- Native WinUI build.
- Rust Core integration.

If UI execution requires a Windows desktop session unavailable in CI, separate build verification from actual UI runtime verification.

Do not report an unexecuted UI test as passed.

### Linux CI Requirements

Install compatible native GTK dependencies.

Verify compilation.

Run suitable headless checks where possible.

Native Wayland runtime verification may require a dedicated compositor or desktop test environment.

Document unavailable runtime verification separately.

### General CI Rules

Pin compatible toolchains and dependencies where practical.

Use minimal CI permissions.

Keep signing secrets outside source control.

Do not create production release pipelines during the foundation phase.

---

## 24. Desktop Packaging and Distribution

Document packaging conventions without implementing production distribution.

### macOS

Future packaging may include:

- Native `.app` bundles.
- Code signing.
- Notarization.
- DMG.
- PKG.
- Mac App Store.

### Windows

Future packaging may include:

- Native executables.
- Windows App SDK runtime dependencies.
- Framework-dependent deployment.
- Self-contained deployment where supported.
- MSIX.
- Native installers.
- Code signing.
- Microsoft Store.

The packaging approach must remain compatible with the chosen Rust-only WinUI 3 architecture.

### Linux

Future packaging may include:

- Flatpak.
- DEB.
- RPM.
- AppImage where appropriate.

Evaluate Flatpak as a preferred portable Linux distribution approach.

Document sandbox, portal, GTK, and runtime dependencies.

Do not build production installers during initial foundation setup unless necessary to verify an essential architectural requirement.

---

## 25. Documentation Requirements

Create authoritative documentation covering:

### Architecture

- Overview.
- Ownership matrix.
- Dependency graph.
- Repository structure.
- Layer boundaries.
- Extension procedures.

### Rust

- Core ownership.
- Crate dependencies.
- Public interfaces.
- Error conventions.
- Threading conventions.

### macOS

- SwiftUI architecture.
- UniFFI integration.
- AppKit boundaries.
- Native lifecycle.
- Build requirements.

### Windows

- Rust-native UI architecture.
- windows-reactor integration.
- Windows App SDK compatibility.
- windows-rs integration.
- Native lifecycle.
- Runtime requirements.
- Experimental dependency assessment.

### Linux

- GTK architecture.
- libadwaita integration.
- Direct Core dependencies.
- Wayland compatibility.
- X11 compatibility.
- Desktop integration requirements.

### Design System

- Token schema.
- Semantic naming.
- Native platform mapping.
- Accessibility.
- Component conventions.

### Shared Resources

- Configuration.
- Localization.
- Assets.
- Versioning.
- Generation procedures.

### Contracts

- Rust API conventions.
- UniFFI conventions.
- Error conventions.
- Resource ownership.
- Compatibility.

### Quality

- Automated compliance.
- Testing.
- Build verification.
- Platform coverage.

Documentation must reflect actual implementation.

Do not document deferred functionality as already implemented.

---

## 26. Future Development Policy

The following capabilities are outside the initial scope:

- Authentication.
- User management.
- Customer management.
- ERP modules.
- REST API integration.
- OpenAPI-generated SDKs.
- Database persistence.
- SQLite.
- Offline synchronization.
- Conflict resolution.
- Image processing.
- Background uploads.
- Background synchronization.
- System tray services.
- Global keyboard hooks.
- Auto-update services.
- Plugin systems.
- Runtime feature flag engines.
- Advanced navigation.
- Advanced window management.
- Large component libraries.
- Additional desktop platforms.
- Mobile applications.

Each future capability requires separate architectural analysis and implementation planning.

Future functionality must preserve:

1. Shared Rust ownership.
2. Direct Rust integration for Windows and Linux.
3. UniFFI interoperability for macOS.
4. Explicit contracts.
5. Native UI boundaries.
6. Canonical design definitions.
7. Shared resource conventions.
8. Agent governance.
9. Automated architecture compliance.

Do not introduce speculative dependencies for deferred capabilities.

---

## 27. Foundation Acceptance Criteria

The foundation is complete when:

1. Cargo Workspace builds successfully.
2. Shared Rust Core is independent of native UI frameworks.
3. One authoritative Rust Hello World implementation exists.
4. macOS UniFFI bindings generate correctly.
5. macOS invokes the Rust function successfully.
6. Windows uses Rust and native WinUI 3.
7. Windows uses windows-reactor or an explicitly approved Rust-native equivalent.
8. Windows invokes Rust Core directly.
9. Windows does not require an application-level C# layer.
10. Linux uses GTK 4/libadwaita.
11. Linux invokes Rust Core directly.
12. Linux supports native Wayland.
13. Linux X11 compatibility is addressed.
14. All three applications display the actual Rust result.
15. Canonical design tokens exist.
16. All three platforms consume generated or reproducibly mapped tokens.
17. Light and dark appearance are demonstrated.
18. Shared configuration conventions are documented.
19. Shared localization conventions exist.
20. A minimal localization example exists.
21. Shared asset ownership rules are documented.
22. Shared contract conventions are established.
23. macOS FFI ownership is documented and validated where applicable.
24. Windows windows-reactor compatibility is evaluated.
25. Architecture compliance checks are configured.
26. Root AGENTS.md contains enforceable development instructions.
27. Architecture documentation is available.
28. Build scripts are functional.
29. GitHub Actions workflows are configured.
30. Relevant tests have been executed.
31. Actual verification results are reported.
32. No speculative application services are implemented.

Do not claim foundation completion if one of the mandatory platform integrations has not been verified.

If verification cannot be executed in the available environment, report the relevant criterion as unverified.

---

## 28. Execution Instructions

Begin by inspecting the current repository.

If the repository is empty, create the foundation from scratch.

Follow this sequence.

### Step 1 — Architecture Analysis

Define:

- Minimal foundation architecture.
- Ownership matrix.
- Dependency boundaries.
- Repository structure.
- Platform responsibilities.
- macOS interoperability.
- Windows direct Rust integration.
- Linux direct Rust integration.

Document the decisions.

### Step 2 — Toolchain Verification

Verify compatible versions of:

- Rust.
- Cargo.
- UniFFI.
- Swift.
- Xcode.
- windows-rs.
- windows-reactor.
- Windows App SDK.
- GTK 4.
- libadwaita.
- gtk-rs.
- Native build dependencies.

Explicitly assess windows-reactor's experimental status.

Do not assume that individually latest versions are automatically compatible.

### Step 3 — Windows Proof of Concept

Before creating a large Windows scaffold, verify the smallest functional WinUI 3 application using Rust and windows-reactor.

Validate:

- Startup.
- Native window creation.
- Input.
- Button.
- Output label.
- Event handling.
- Rust Core invocation.

Document compatibility and limitations.

Do not silently substitute C#.

### Step 4 — Repository Foundation

Create:

- Cargo Workspace.
- macOS project.
- Windows Rust application.
- Linux Rust application.
- Shared directories.
- Root AGENTS.md.
- Initial documentation.

### Step 5 — Shared Rust Core

Implement the Hello World function.

Add unit tests.

Verify the Core builds independently.

### Step 6 — macOS Interoperability

Create the minimal macOS FFI crate.

Generate Swift bindings through UniFFI.

Integrate the shared function into SwiftUI.

### Step 7 — Windows Native Integration

Create the Rust-native WinUI 3 application.

Use windows-reactor.

Import Rust Core directly.

Do not introduce C# or a custom Rust-to-Rust FFI layer.

### Step 8 — Linux Native Integration

Create the GTK 4/libadwaita application.

Import Rust Core directly.

Verify Wayland-oriented architecture.

Preserve X11 compatibility.

### Step 9 — Shared Design System

Implement:

- Canonical tokens.
- Deterministic generation.
- SwiftUI theme mapping.
- WinUI 3 Rust theme mapping.
- GTK theme mapping.
- Light and dark appearance.

### Step 10 — Shared Resources

Establish conventions for:

- Configuration.
- Localization.
- Assets.
- Contracts.

Implement only minimal examples required for validation.

### Step 11 — Architecture Compliance

Implement lightweight checks for enforceable architecture rules.

### Step 12 — Build and CI

Configure:

- Reproducible build scripts.
- Platform-specific CI jobs.
- Automated tests.
- Generated-code validation.

### Step 13 — Final Verification

Execute all available builds and tests.

Document:

- Verified platforms.
- Verified architectures.
- Executed tests.
- Failed tests.
- Missing host requirements.
- Unverified targets.
- Remaining foundation limitations.

Do not expand beyond foundation scope.

Do not introduce unnecessary infrastructure.

Do not redesign established ownership boundaries without documented architectural justification.

Prefer a small, working, verifiable foundation over a large incomplete scaffold.

---

# FINAL MANDATORY PRINCIPLES

**1. One authoritative owner per responsibility.**

**2. Rust Core owns all shared application behavior.**

**3. macOS uses genuine native SwiftUI.**

**4. Windows uses genuine native WinUI 3 implemented in Rust.**

**5. Linux uses genuine native GTK 4/libadwaita implemented in Rust.**

**6. Windows and Linux consume Rust Core directly through Cargo dependencies.**

**7. macOS uses UniFFI as the explicit cross-language interoperability boundary.**

**8. No C# or .NET application layer may be introduced into Windows.**

**9. No unnecessary custom FFI may be introduced between Rust-native components.**

**10. Windows uses windows-reactor as the initial native WinUI 3 integration approach, subject to compatibility validation.**

**11. Experimental technologies must be validated before architectural commitment.**

**12. The Shared Design System owns canonical visual definitions.**

**13. Shared Resources own canonical configuration, localization, and brand assets.**

**14. Generated code must be reproducible and must never be manually maintained.**

**15. Repository structure defines enforceable architectural boundaries.**

**16. docs/ and AGENTS.md preserve consistency across coding agents.**

**17. Automated compliance and testing protect architectural integrity.**

**18. Native appearance, accessibility, and operating-system conventions must be preserved.**

**19. Future functionality must extend the foundation through deliberate analysis and PRDs.**

**20. The initial implementation must remain minimal, functional, and verifiable.**

**21. Every native desktop application must invoke the same authoritative Rust Core.**

---

## FINAL DELIVERY EXPECTATIONS

Upon completion, provide:

1. Architecture summary.
2. Selected technology versions.
3. Compatibility verification results.
4. Implemented repository structure.
5. Rust Core implementation summary.
6. macOS integration summary.
7. Windows Rust/WinUI integration summary.
8. Linux GTK integration summary.
9. Shared Design System summary.
10. Shared Resources summary.
11. Build commands.
12. Test execution results.
13. CI configuration summary.
14. Known limitations.
15. Unverified targets.
16. Suggested next architectural milestones.

Do not claim functionality is complete without actual verification.

Do not substitute placeholder code or documentation for executable integration.

Do not introduce C# into Windows merely because it is the conventional WinUI 3 development language.

Build a stable, maintainable, verifiable, enterprise-oriented native desktop monorepo that future coding agents can extend without duplicated responsibilities or unclear architectural ownership.

**Begin the foundation analysis and implementation now.**
