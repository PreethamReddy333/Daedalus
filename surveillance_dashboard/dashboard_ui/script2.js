/**
 * Surveillance Dashboard - JavaScript
 * Connects to WeilChain via WeilWallet and loads data from surveillance_dashboard contract
 */

// Contract configuration
const CONFIG = {
    contractAddress: 'aaaaaa425yj6nyqxta4t7rctn6av6mm7hjvmf2vm2sxemiomdw4e4ggaga',
};

// State
let wallet = null;
let connected = false;
let elements = {};

// Initialize after DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    // DOM Elements
    elements = {
        connectBtn: document.getElementById('connectWallet'),
        connectionStatus: document.getElementById('connectionStatus'),
        alertsToday: document.getElementById('alertsToday'),
        openCases: document.getElementById('openCases'),
        highRiskEntities: document.getElementById('highRiskEntities'),
        complianceScore: document.getElementById('complianceScore'),
        alertsList: document.getElementById('alertsList'),
        casesList: document.getElementById('casesList'),
        riskGrid: document.getElementById('riskGrid'),
        workflowList: document.getElementById('workflowList'),
        severityFilter: document.getElementById('severityFilter'),
        caseStatusFilter: document.getElementById('caseStatusFilter'),
    };

    // Event Listeners
    if (elements.connectBtn) {
        elements.connectBtn.addEventListener('click', connectWallet);
        console.log('Connect button found and listener attached');
    } else {
        console.error('Connect button not found!');
    }

    if (elements.severityFilter) {
        elements.severityFilter.addEventListener('change', loadAlerts);
    }
    if (elements.caseStatusFilter) {
        elements.caseStatusFilter.addEventListener('change', loadCases);
    }

    // Load demo data for preview
    loadDemoData();
});

// ===== Wallet Connection =====
async function connectWallet() {
    console.log('Connect wallet clicked');

    if (typeof window.WeilWallet === 'undefined') {
        alert('WeilWallet extension not found. Please install it from the Chrome Web Store.');
        return;
    }

    try {
        // Use correct method name from WeilWallet docs
        const accounts = await window.WeilWallet.request({
            method: 'weil_requestAccounts',
        });
        console.log('Accounts received:', accounts);

        if (accounts && accounts.length > 0) {
            wallet = window.WeilWallet;
            connected = true;
            updateConnectionStatus(true);
            loadAllData();
        }
    } catch (error) {
        console.error('Failed to connect wallet:', error);
        alert('Failed to connect to wallet: ' + error.message);
    }
}

function updateConnectionStatus(isConnected) {
    console.log('Updating connection status:', isConnected);
    const statusEl = elements.connectionStatus;
    if (!statusEl) {
        console.error('Status element not found!');
        return;
    }

    const dot = statusEl.querySelector('.dot');
    const text = statusEl.querySelector('span:last-child');

    if (isConnected) {
        if (dot) dot.className = 'dot online';
        if (text) text.textContent = 'Connected';
        if (elements.connectBtn) {
            elements.connectBtn.textContent = '✓ Connected';
            elements.connectBtn.disabled = true;
        }
    } else {
        if (dot) dot.className = 'dot offline';
        if (text) text.textContent = 'Disconnected';
    }
}

// ===== Data Loading =====
async function loadAllData() {
    await Promise.all([
        loadStats(),
        loadAlerts(),
        loadCases(),
        loadRiskEntities(),
        loadWorkflows(),
    ]);
}

async function callContract(method, args = {}) {
    if (!wallet) return null;

    try {
        const result = await wallet.request({
            method: 'weil_call',
            params: {
                to: CONFIG.contractAddress,
                method: method,
                args: JSON.stringify(args),
            },
        });
        return JSON.parse(result);
    } catch (error) {
        console.error(`Error calling ${method}:`, error);
        return null;
    }
}

async function loadStats() {
    const stats = await callContract('get_stats');
    if (stats && stats.Ok) {
        const data = stats.Ok;
        elements.alertsToday.textContent = data.total_alerts_today || 0;
        elements.openCases.textContent = data.open_cases || 0;
        elements.highRiskEntities.textContent = data.high_risk_entities || 0;
        elements.complianceScore.textContent = (data.compliance_score || 0) + '%';
    }
}

async function loadAlerts() {
    const severity = elements.severityFilter ? elements.severityFilter.value : 'ALL';
    const alerts = await callContract('get_live_alerts', {
        severity_filter: severity === 'ALL' ? null : severity,
        limit: 20,
    });

    if (alerts && alerts.Ok) {
        renderAlerts(alerts.Ok);
    }
}

async function loadCases() {
    const status = elements.caseStatusFilter ? elements.caseStatusFilter.value : 'ALL';
    const cases = await callContract('get_cases_by_status', {
        status: status === 'ALL' ? null : status,
        limit: 20,
    });

    if (cases && cases.Ok) {
        renderCases(cases.Ok);
    }
}

async function loadRiskEntities() {
    const entities = await callContract('get_high_risk_entities', {
        min_risk_score: 70,
        limit: 10,
    });

    if (entities && entities.Ok) {
        renderRiskEntities(entities.Ok);
    }
}

async function loadWorkflows() {
    const workflows = await callContract('get_workflow_history', {
        workflow_type: null,
        limit: 10,
    });

    if (workflows && workflows.Ok) {
        renderWorkflows(workflows.Ok);
    }
}

// ===== Rendering Functions =====
function renderAlerts(alerts) {
    if (!elements.alertsList) return;

    if (!alerts || alerts.length === 0) {
        elements.alertsList.innerHTML = '<div class="empty-state">No alerts found</div>';
        return;
    }

    elements.alertsList.innerHTML = alerts.map(alert => `
        <div class="alert-item">
            <div class="alert-severity ${alert.severity}"></div>
            <div class="alert-content">
                <div class="alert-title">${alert.alert_type}: ${alert.description}</div>
                <div class="alert-meta">
                    <span>📈 ${alert.symbol}</span>
                    <span>👤 ${alert.entity_id}</span>
                    <span>Score: ${alert.risk_score}</span>
                </div>
            </div>
        </div>
    `).join('');
}

function renderCases(cases) {
    if (!elements.casesList) return;

    if (!cases || cases.length === 0) {
        elements.casesList.innerHTML = '<div class="empty-state">No cases found</div>';
        return;
    }

    elements.casesList.innerHTML = cases.map(c => `
        <div class="case-item">
            <div class="case-info">
                <div class="case-id">${c.case_id}</div>
                <div class="case-subject">${c.subject_entity} - ${c.symbol}</div>
            </div>
            <span class="case-status ${c.status}">${c.status}</span>
        </div>
    `).join('');
}

function renderRiskEntities(entities) {
    if (!elements.riskGrid) return;

    if (!entities || entities.length === 0) {
        elements.riskGrid.innerHTML = '<div class="empty-state">No high-risk entities</div>';
        return;
    }

    elements.riskGrid.innerHTML = entities.map(entity => `
        <div class="risk-entity">
            <div class="risk-entity-name">${entity.entity_name}</div>
            <div class="risk-score-bar">
                <div class="risk-score-fill ${entity.risk_score >= 85 ? 'high' : 'medium'}" 
                     style="width: ${entity.risk_score}%"></div>
            </div>
            <div class="risk-meta">
                <span>Score: ${entity.risk_score}</span>
                <span>Alerts: ${entity.alert_count}</span>
            </div>
        </div>
    `).join('');
}

function renderWorkflows(workflows) {
    if (!elements.workflowList) return;

    if (!workflows || workflows.length === 0) {
        elements.workflowList.innerHTML = '<div class="empty-state">No workflows found</div>';
        return;
    }

    elements.workflowList.innerHTML = workflows.map(wf => {
        const progress = wf.total_steps > 0 ? (wf.steps_completed / wf.total_steps) * 100 : 0;
        const icon = wf.status === 'COMPLETED' ? '✅' : wf.status === 'FAILED' ? '❌' : '⏳';

        return `
            <div class="workflow-item">
                <div class="workflow-status-icon">${icon}</div>
                <div class="workflow-content">
                    <div class="workflow-type">${wf.workflow_type}</div>
                    <div class="workflow-progress">
                        <div class="progress-bar">
                            <div class="progress-fill" style="width: ${progress}%"></div>
                        </div>
                        <span class="progress-text">${wf.steps_completed}/${wf.total_steps}</span>
                    </div>
                </div>
            </div>
        `;
    }).join('');
}

// ===== Demo Mode =====
function loadDemoData() {
    // Demo stats
    if (elements.alertsToday) elements.alertsToday.textContent = '24';
    if (elements.openCases) elements.openCases.textContent = '7';
    if (elements.highRiskEntities) elements.highRiskEntities.textContent = '5';
    if (elements.complianceScore) elements.complianceScore.textContent = '92%';

    // Demo alerts
    const demoAlerts = [
        { severity: 'CRITICAL', alert_type: 'INSIDER_TRADING', description: 'Large trade before earnings', symbol: 'RELIANCE', entity_id: 'ENT-001', risk_score: 92 },
        { severity: 'HIGH', alert_type: 'WASH_TRADE', description: 'Circular trading pattern detected', symbol: 'TCS', entity_id: 'ENT-002', risk_score: 78 },
        { severity: 'MEDIUM', alert_type: 'SPOOFING', description: 'Large orders cancelled rapidly', symbol: 'INFY', entity_id: 'ENT-003', risk_score: 65 },
    ];
    renderAlerts(demoAlerts);

    // Demo cases
    const demoCases = [
        { case_id: 'CASE-2026-001', subject_entity: 'Rajesh Kumar', symbol: 'RELIANCE', status: 'INVESTIGATING' },
        { case_id: 'CASE-2026-002', subject_entity: 'Priya Sharma', symbol: 'TCS', status: 'OPEN' },
        { case_id: 'CASE-2026-003', subject_entity: 'Amit Patel', symbol: 'HDFC', status: 'CLOSED' },
    ];
    renderCases(demoCases);

    // Demo risk entities
    const demoRisk = [
        { entity_name: 'Rajesh Kumar', risk_score: 92, alert_count: 5 },
        { entity_name: 'Priya Sharma', risk_score: 78, alert_count: 3 },
        { entity_name: 'Amit Patel', risk_score: 71, alert_count: 2 },
    ];
    renderRiskEntities(demoRisk);

    // Demo workflows
    const demoWorkflows = [
        { workflow_type: 'INSIDER_DETECTION', status: 'COMPLETED', steps_completed: 5, total_steps: 5 },
        { workflow_type: 'KYC_ONBOARD', status: 'RUNNING', steps_completed: 3, total_steps: 6 },
        { workflow_type: 'MANIPULATION_CHECK', status: 'FAILED', steps_completed: 2, total_steps: 4 },
    ];
    renderWorkflows(demoWorkflows);
}
