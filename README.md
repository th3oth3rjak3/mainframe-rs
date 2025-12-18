# Mainframe

> **Note:** This repository is currently named **`mainframe-rs`** and will be renamed to **`mainframe`** once legacy code is migrated from an older repository.

## Overview

Mainframe is a self-hosted web application focused on personal productivity for individuals, families, and small trusted groups. It aims to consolidate everyday tools—such as recipes, calendars, shared lists, and passwords—into a single, ad-free platform that prioritizes simplicity, control, and ease of use.

The primary goal of Mainframe is to handle common day-to-day coordination and organization tasks without relying on third-party SaaS platforms, external services, or advertising-driven products.

## Project Philosophy

- **Self-hostable first** – Designed to be easy to run privately, with minimal external dependencies.
- **No ads, no tracking** – Mainframe is intended for personal use, not monetization through user data.
- **Simple over exhaustive** – Features are intentionally scoped to avoid unnecessary complexity.
- **Modular access** – Users can opt into only the applications they want to use.
- **No email dependency** – The platform does not send emails and does not require an email server.

## Current Status

🚧 **Early Development / Design Phase**

Mainframe is under active development and is not yet feature-complete or production-ready.

### What Exists Today

- Backend web server with basic HTTP routing
- Cookie-based authentication (early implementation)
- Backend-hosted frontend
- Basic landing page and layout shell

### What Does _Not_ Exist Yet

- No fully implemented applications (recipes, calendars, lists, passwords)
- No finalized data models
- No permissions or group management UI
- No TOS or Privacy Policy (planned)

Breaking changes should be expected at this stage.

## Planned Applications & Roadmap

### Recipes

A recipe management application supporting both personal and shared use.

Planned capabilities:

- Private recipes
- Public recipes
- Group-shared recipes
- Optional collaborator access for editing recipes

**Ownership note:**

- Currently, recipes are owned by the creating user and are removed if that user account is deleted.
- A future Terms of Service may allow public recipes to be anonymized and retained for platform stability.

### Calendar

A simple, interactive calendar focused on everyday scheduling.

Planned capabilities:

- Personal calendars
- Shared and group calendars
- Add events to specific days with optional time ranges
- In-app notifications ("remind me")

**Explicitly out of scope:**

- Email notifications
- Location-based features
- External calendar integrations (for now)

### Password Manager

A lightweight password vault intended for personal and family use.

Planned capabilities:

- Private password vault per user
- (Under consideration) Shared/group vaults for common credentials

Security notes:

- User login passwords are hashed using **Argon2id**
- Vault entries are encrypted using **SHA-256**
- Users must provide their application password to decrypt private vaults

**Important:**

- Group/shared vault handling is still under design
- This is **not** intended to replace full-featured password managers like Bitwarden

### Lists

A generic list system designed to be flexible and reusable.

Planned capabilities:

- Arbitrary list types (todos, shopping lists, notes, etc.)
- Shared lists
- Granular permissions:
  - View-only
  - Edit
  - Full control

- Optional tagging system for organization and filtering

## Modularity & Access Control

- Users can opt into specific applications (recipes, calendar, lists, passwords)
- Access is granted per application, not globally
- Groups are shared across applications
- Permissions follow a least-privilege-by-default model
- User provisioning is managed by the host administrator

## Hosting & Deployment

Mainframe is designed to be:

- Hosted by the author for personal and family use (behind Cloudflare)
- Easily self-hosted by technically inclined users

The goal is to make private deployment straightforward without requiring complex infrastructure.

## Known Gaps & TODOs

- Privacy Policy
- Terms of Service
- Finalized cryptographic design for shared password vaults
- Admin and user management UI
- Application-specific UIs

## Milestone Checklist

### Backend

- [x] Basic HTTP server
- [x] Cookie-based authentication (early)
- [ ] Database schema design

### Frontend

- [x] Landing page scaffold
- [ ] Basic routing for apps

### Recipe Application

- [ ] Private recipe CRUD
- [ ] Public recipes
- [ ] Group sharing
- [ ] Optional collaborator editing

### Calendar Application

- [ ] Event creation and editing
- [ ] Shared/group calendars
- [ ] In-app notifications

### Password Manager

- [ ] Private vault per user
- [ ] Encryption implementation
- [ ] Shared/group vault handling (under design)

### Lists Application

- [ ] Generic list creation
- [ ] Shared lists with permissions
- [ ] Tagging/filter system

## License & Project Direction

This project is licensed under the [MIT License](./LICENSE).

Mainframe is currently a personal project with long-term maintenance goals. Contributions may be considered in the future once the core architecture stabilizes.

---

⚠️ **Disclaimer:** Mainframe is under active development. Use at your own risk, especially for sensitive data.
