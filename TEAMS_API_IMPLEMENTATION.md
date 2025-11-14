# Teams/Edge API Implementation Guide

This document outlines the backend API endpoints needed to support the Teams/Edge management interface in the WebUI.

## Overview

The WebUI now includes a comprehensive Teams/Edge management page that requires backend API support. The frontend is fully implemented and uses mock data in development mode until the backend endpoints are available.

## Required API Endpoints

### 1. Get Teams Status
**Endpoint:** `GET /api/v1/teams/status`

**Response:**
```json
{
  "enabled": true,
  "config": {
    "sbc_fqdn": "sbc.example.com",
    "tenant_domain": "contoso.onmicrosoft.com",
    "mtls_cert_path": "/etc/rustalk/teams-cert.pem",
    "mtls_key_path": "/etc/rustalk/teams-key.pem",
    "sip_proxies": [
      "sip.pstnhub.microsoft.com",
      "sip2.pstnhub.microsoft.com",
      "sip3.pstnhub.microsoft.com"
    ],
    "options_ping_enabled": true,
    "options_ping_interval": 60
  },
  "health_status": [
    {
      "proxy": "sip.pstnhub.microsoft.com",
      "status": "healthy",
      "last_check": 1699456800000,
      "response_time_ms": 45
    }
  ],
  "total_calls": 1523,
  "active_calls": 12
}
```

### 2. Get Teams Configuration
**Endpoint:** `GET /api/v1/teams/config`

**Response:**
```json
{
  "sbc_fqdn": "sbc.example.com",
  "tenant_domain": "contoso.onmicrosoft.com",
  "mtls_cert_path": "/etc/rustalk/teams-cert.pem",
  "mtls_key_path": "/etc/rustalk/teams-key.pem",
  "sip_proxies": [
    "sip.pstnhub.microsoft.com",
    "sip2.pstnhub.microsoft.com",
    "sip3.pstnhub.microsoft.com"
  ],
  "options_ping_enabled": true,
  "options_ping_interval": 60
}
```

### 3. Update Teams Configuration
**Endpoint:** `PUT /api/v1/teams/config`

**Request Body:** Same as TeamsConfig response above

**Response:**
```json
{
  "success": true,
  "message": "Teams configuration updated successfully"
}
```

### 4. Test Teams Connection
**Endpoint:** `POST /api/v1/teams/test`

**Response:**
```json
{
  "success": true,
  "message": "Connection test completed",
  "health_status": [
    {
      "proxy": "sip.pstnhub.microsoft.com",
      "status": "healthy",
      "last_check": 1699456800000,
      "response_time_ms": 45,
      "error": null
    }
  ]
}
```

## Implementation Steps

### 1. Add Teams Handler Module

Create `rustalk-cloud/src/handlers/teams.rs`:

```rust
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use rustalk_edge::teams::TeamsConfig;
use rustalk_edge::health::TeamsHealth;

pub type TeamsState = Arc<RwLock<Option<TeamsConfig>>>;

pub async fn get_teams_status(
    State(state): State<TeamsState>,
) -> (StatusCode, Json<TeamsStatusResponse>) {
    // Implementation here
}

pub async fn get_teams_config(
    State(state): State<TeamsState>,
) -> (StatusCode, Json<TeamsConfig>) {
    // Implementation here
}

pub async fn update_teams_config(
    State(state): State<TeamsState>,
    Json(config): Json<TeamsConfig>,
) -> (StatusCode, Json<UpdateResponse>) {
    // Implementation here
}

pub async fn test_teams_connection(
    State(state): State<TeamsState>,
) -> (StatusCode, Json<TestResponse>) {
    // Implementation here
}
```

### 2. Register Routes in API Module

In `rustalk-cloud/src/api/mod.rs`:

```rust
// Add Teams state
let teams_state = Arc::new(RwLock::new(None));

// Add routes
.route("/api/v1/teams/status", get(handlers::teams::get_teams_status).with_state(teams_state.clone()))
.route("/api/v1/teams/config", get(handlers::teams::get_teams_config).with_state(teams_state.clone()))
.route("/api/v1/teams/config", put(handlers::teams::update_teams_config).with_state(teams_state.clone()))
.route("/api/v1/teams/test", post(handlers::teams::test_teams_connection).with_state(teams_state))
```

### 3. Integrate with rustalk-edge

The handlers should use the `TeamsGateway` from `rustalk-edge` crate:

```rust
use rustalk_edge::{TeamsGateway, TeamsConfig};
```

### 4. Add Health Monitoring

Implement periodic OPTIONS ping in the background:
- Use the `options_ping_interval` from config
- Track last check time and response times
- Update health status based on responses

### 5. Configuration Persistence

- Load Teams config from the main config file or database
- Validate configuration before applying
- Restart Teams gateway when config changes

## Testing

1. Start the RusTalk server with Teams configuration
2. Open the WebUI at http://localhost:8080
3. Navigate to Teams/SBC → Teams Edge
4. Verify all sections load and display data
5. Test configuration updates
6. Test connection testing

## Security Considerations

- Validate all configuration inputs
- Ensure mTLS certificate paths are secure
- Sanitize error messages to avoid information leakage
- Implement proper access control for Teams endpoints

## Future Enhancements

- Real-time health status updates via WebSocket
- Detailed call logs for Teams calls
- Certificate expiration warnings
- Automatic certificate renewal integration
- Call quality metrics and analytics
