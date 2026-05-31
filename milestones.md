# SchemaForge - Development Task List

## Milestone 0: Project Foundation

### Backend Setup

* [ ] Initialize Go project
* [ ] Setup Gin framework
* [ ] Setup PostgreSQL connection
* [ ] Setup Redis connection
* [ ] Setup configuration management
* [ ] Setup structured logging
* [ ] Setup dependency injection
* [ ] Setup Dockerfile
* [ ] Setup Docker Compose

### Frontend Setup

* [ ] Create React + TypeScript + Vite app
* [ ] Install Tailwind CSS
* [ ] Configure ESLint + Prettier
* [ ] Setup TanStack Query
* [ ] Setup routing
* [ ] Setup layout system
* [ ] Setup API client

### Infrastructure

* [ ] Docker Compose for local development
* [ ] PostgreSQL container
* [ ] Redis container
* [ ] NATS container

---

# Milestone 1: Metadata Layer

## Database Metadata Tables

### Entities

* [ ] Create `schema_entities` table
* [ ] Create entity repository
* [ ] Create entity service
* [ ] Create entity APIs

### Fields

* [ ] Create `schema_fields` table
* [ ] Create field repository
* [ ] Create field service
* [ ] Create field APIs

### Relationships

* [ ] Create `schema_relationships` table
* [ ] Create relationship repository
* [ ] Create relationship service
* [ ] Create relationship APIs

### Metadata APIs

* [ ] Get all entities
* [ ] Get entity by id
* [ ] Create entity
* [ ] Update entity
* [ ] Delete entity
* [ ] Create field
* [ ] Update field
* [ ] Delete field

### Deliverable

* [ ] Admin can create entities and fields through APIs

---

# Milestone 2: Schema Registry Engine

## Registry

### Backend

* [ ] Build schema registry service
* [ ] Load metadata on startup
* [ ] Build in-memory schema cache
* [ ] Build Redis schema cache
* [ ] Create schema refresh mechanism

### APIs

* [ ] Get schema endpoint
* [ ] Get entity schema endpoint
* [ ] Get field metadata endpoint

### Deliverable

* [ ] System can generate schema model dynamically

---

# Milestone 3: Dynamic CRUD Engine

## Generic CRUD

### Create

* [ ] Generic create service
* [ ] Dynamic insert builder

### Read

* [ ] Generic read service
* [ ] Dynamic select builder

### Update

* [ ] Generic update service
* [ ] Dynamic update builder

### Delete

* [ ] Generic delete service

### APIs

* [ ] POST /records
* [ ] GET /records
* [ ] PATCH /records
* [ ] DELETE /records

### Deliverable

* [ ] CRUD works for any entity

---

# Milestone 4: Dynamic Query Builder

## Query Features

### Filtering

* [ ] Equals
* [ ] Not equals
* [ ] Greater than
* [ ] Less than
* [ ] In
* [ ] Like

### Sorting

* [ ] Ascending
* [ ] Descending

### Pagination

* [ ] Offset pagination
* [ ] Limit support

### Relationships

* [ ] Foreign key joins
* [ ] Nested entity fetching

### Security

* [ ] Parameterized queries
* [ ] SQL injection protection

### Deliverable

* [ ] Dynamic query generation complete

---

# Milestone 5: Validation Engine

## Metadata Driven Validation

### Rules

* [ ] Required validation
* [ ] Type validation
* [ ] Length validation
* [ ] Regex validation
* [ ] Enum validation
* [ ] Foreign key validation

### Engine

* [ ] Validation service
* [ ] Validation middleware

### Deliverable

* [ ] No hardcoded validations

---

# Milestone 6: Authentication & RBAC

## Authentication

### User Management

* [ ] Users table
* [ ] Roles table
* [ ] JWT authentication

### Permissions

#### Entity Level

* [ ] View
* [ ] Create
* [ ] Update
* [ ] Delete

#### Field Level

* [ ] Read permissions
* [ ] Write permissions

#### Row Level

* [ ] Row access filtering

### Deliverable

* [ ] RBAC fully operational

---

# Milestone 7: Frontend Core

## Layout

* [ ] Sidebar
* [ ] Header
* [ ] Navigation
* [ ] Responsive layout

## Shared Components

* [ ] Button
* [ ] Modal
* [ ] Drawer
* [ ] Table
* [ ] Form components

### Deliverable

* [ ] Base UI framework complete

---

# Milestone 8: Dynamic Table Generator

## DynamicTable Component

### Features

* [ ] Dynamic columns
* [ ] Dynamic sorting
* [ ] Dynamic filtering
* [ ] Pagination
* [ ] Row actions

### Deliverable

* [ ] Table generated from metadata

---

# Milestone 9: Dynamic Form Generator

## DynamicForm Component

### Field Types

* [ ] Text
* [ ] Number
* [ ] Date
* [ ] Select
* [ ] Checkbox
* [ ] Radio
* [ ] Textarea

### Features

* [ ] Dynamic validation
* [ ] Dynamic field rendering
* [ ] Relationship fields

### Deliverable

* [ ] Forms generated from metadata

---

# Milestone 10: Entity Builder

## Admin UI

### Entity Management

* [ ] Create entity
* [ ] Edit entity
* [ ] Delete entity

### Field Management

* [ ] Create field
* [ ] Edit field
* [ ] Delete field

### Relationship Management

* [ ] Create relationship
* [ ] Edit relationship
* [ ] Delete relationship

### Deliverable

* [ ] Users can create modules without code

---

# Milestone 11: Event System (NATS)

## NATS Setup

* [ ] NATS integration
* [ ] Event publisher
* [ ] Event subscriber

### Events

* [ ] record.created
* [ ] record.updated
* [ ] record.deleted
* [ ] schema.changed

### Deliverable

* [ ] Event-driven architecture enabled

---

# Milestone 12: Workflow Engine

## Backend

### Workflow Storage

* [ ] workflows table
* [ ] workflow repository

### Execution

* [ ] Trigger engine
* [ ] Step executor
* [ ] Retry mechanism

### Workflow Actions

* [ ] Create record
* [ ] Update record
* [ ] Delete record
* [ ] Send webhook
* [ ] Publish event

### Deliverable

* [ ] Workflows execute dynamically

---

# Milestone 13: WebSocket Gateway

## Real-time Layer

### Backend

* [ ] WebSocket server
* [ ] Connection manager

### Events

* [ ] schema.changed
* [ ] workflow.completed
* [ ] record.updated

### Frontend

* [ ] Live updates
* [ ] Cache invalidation

### Deliverable

* [ ] Real-time synchronization

---

# Milestone 14: Workflow Builder

## Frontend

### React Flow

* [ ] Install React Flow
* [ ] Build workflow canvas
* [ ] Create workflow nodes
* [ ] Create workflow edges

### Features

* [ ] Drag and drop
* [ ] Save workflow
* [ ] Load workflow

### Deliverable

* [ ] Visual workflow editor

---

# Milestone 15: Search

## Search System

### OpenSearch

* [ ] Setup OpenSearch
* [ ] Index entities
* [ ] Search APIs

### Features

* [ ] Global search
* [ ] Entity search
* [ ] Filter search

### Deliverable

* [ ] Full platform search

---

# Milestone 16: AI Layer

## AI Integration

### Schema Discovery

* [ ] Schema API for AI
* [ ] Metadata retrieval

### AI Actions

* [ ] Create records
* [ ] Update records
* [ ] Query records
* [ ] Trigger workflows

### Deliverable

* [ ] AI can operate dynamically on platform

---

# MVP Completion Criteria

* [ ] Dynamic entities
* [ ] Dynamic fields
* [ ] Dynamic relationships
* [ ] Dynamic CRUD
* [ ] Dynamic validation
* [ ] RBAC
* [ ] Dynamic tables
* [ ] Dynamic forms
* [ ] Event system
* [ ] Workflow engine
* [ ] Workflow builder
* [ ] Real-time updates
* [ ] Search
* [ ] AI-ready architecture
