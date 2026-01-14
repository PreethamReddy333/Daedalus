# Jira MCP Server
A Model Context Protocol (MCP) applet that integrates with the **Jira Cloud REST API v3** to create and manage tickets.

## Core tools
- `create_ticket`: Create a new Jira ticket.
- `create_case_ticket`: Create a ticket specifically for a surveillance case.
- `close_ticket`: Close a Jira ticket.
- `get_ticket`: Get ticket details by key.
- `add_comment`: Add a comment to a ticket.
- `update_ticket_status`: Update ticket status.

## Testing 

### Deployment
```
deploy -f <path to>/jira_mcp.wasm -p <path to>/jira_mcp.widl -c <path to>/config.yaml
```

#### `config.yaml`
```yaml
jira_url: "https://your-domain.atlassian.net"
jira_email: "your-email@example.com"
jira_api_token: "YOUR_API_TOKEN"
project_key: "PROJ"
default_issue_type: "Task"
```

### Prompt examples
- Create a Jira ticket for case 123
- Get details for ticket WEIL-1
- Close ticket WEIL-1
- Update status of WEIL-1 to In Progress
