

const CONTRACTS = {
    dashboard: 'aaaaaayrd3bsxvttmltgyzjr34uzesbbqqzris3dlvskwh3asoj7s4xnsu',
    trade_data: 'aaaaaa7ycnpwxv2xks67l4lkczhwgbyawt34u7esvsbfgbnq7putxnvmfm',
    entity_relationship: 'aaaaaa36wpz26hhxrmyvo2clmpqqkwqudoqkymfkgltfmtvoceftcfes5u',
    anomaly_detection: 'aaaaaa4s4a4xuzmfsxs5gbs3hcm7npikux3jah52oq6rvnty63jw7qpoam',
    regulatory_reports: 'aaaaaa77wlohisawkkbxkgc77cb24slezi4a5yx6obqiyjcmwofwx3njfm',
    slack_notifier: 'aaaaaa4l5izydl4rfdxv5vmxrbdtkqka5htfq55d2d6pchqcplqv5gjbma'
};

let wallet = null;
let connected = false;

// ===== Initialization =====
document.addEventListener('DOMContentLoaded', () => {
    console.log('Dashboard initialized');
    setupNavigation();
    setupEventListeners();
    loadDemoData();
});

// ===== Navigation =====
function setupNavigation() {
    document.querySelectorAll('.nav-item').forEach(item => {
        item.addEventListener('click', () => {
            const panel = item.dataset.panel;
            document.querySelectorAll('.nav-item').forEach(n => n.classList.remove('active'));
            item.classList.add('active');
            document.querySelectorAll('.panel').forEach(p => p.classList.remove('active'));
            document.getElementById('panel-' + panel).classList.add('active');
        });
    });
}

// ===== Event Listeners =====
function setupEventListeners() {
    document.getElementById('connectWallet').addEventListener('click', connectWallet);
    document.getElementById('refreshAlerts')?.addEventListener('click', loadAlerts);
    document.getElementById('refreshCases')?.addEventListener('click', loadCases);
    document.getElementById('searchEntity')?.addEventListener('click', searchEntity);
    document.getElementById('searchTrades')?.addEventListener('click', searchTrades);
    document.getElementById('analyzeVolume')?.addEventListener('click', analyzeVolume);
}

// ===== Wallet Connection =====
async function connectWallet() {
    console.log('Connecting wallet...');

    if (typeof window.WeilWallet === 'undefined') {
        alert('WeilWallet extension not found. Please install it.');
        return;
    }

    try {
        const accounts = await window.WeilWallet.request({ method: 'weil_requestAccounts' });
        console.log('Connected accounts:', accounts);

        wallet = window.WeilWallet;
        connected = true;

        document.querySelector('.status-dot').className = 'status-dot online';
        document.querySelector('.status-text').textContent = 'Connected';
        document.getElementById('connectWallet').textContent = '✓ Connected';
        document.getElementById('connectWallet').disabled = true;

        await loadAllData();
    } catch (error) {
        console.error('Wallet connection failed:', error);
        alert('Connection failed: ' + error.message);
    }
}

// ===== Contract Calls =====

async function callContract(contractKey, method, args = {}) {
    if (!wallet) {
        console.warn('Wallet not connected');
        return null;
    }

    const address = CONTRACTS[contractKey];
    if (!address) {
        console.error('Unknown contract:', contractKey);
        return null;
    }

    try {
        console.log(`[${new Date().toISOString()}] Calling ${contractKey}.${method} at ${address}`, args);

        const result = await wallet.request({
            method: 'weil_sendTransaction',
            params: {
                to: address,
                method: method,
                args: JSON.stringify(args)
            }
        });

        console.log(`[${new Date().toISOString()}] Raw result from ${method}:`, result);

        let parsed = result;
        if (typeof result === 'string') {
            try {
                parsed = JSON.parse(result);
            } catch (e) {
            }
        }

        if (parsed && parsed.Ok && typeof parsed.Ok === 'string') {
            try {
                parsed.Ok = JSON.parse(parsed.Ok);
            } catch (e) {
            }
        }

        console.log(`[${new Date().toISOString()}] Parsed result from ${method}:`, parsed);
        return parsed;
    } catch (error) {
        console.error(`[${new Date().toISOString()}] Error calling ${contractKey}.${method}:`, error.message);
        console.error('Full error:', error);
        return null;
    }
}

// ===== Data Loading =====
async function loadAllData() {
    await Promise.all([
        loadStats(),
        loadAlerts(),
        loadCases()
    ]);
}

async function loadStats() {
    const stats = await callContract('dashboard', 'get_stats');
    if (stats?.Ok) {
        document.getElementById('totalAlerts').textContent = stats.Ok.total_alerts_today || 0;
        document.getElementById('openCases').textContent = stats.Ok.open_cases || 0;
        document.getElementById('riskEntities').textContent = stats.Ok.high_risk_entities || 0;
        document.getElementById('completedWorkflows').textContent = stats.Ok.total_workflows_today || 0;
    }
}

async function loadAlerts() {
    const severity = document.getElementById('alertSeverityFilter')?.value || 'ALL';
    const result = await callContract('dashboard', 'get_live_alerts', {
        severity_filter: severity === 'ALL' ? null : severity,
        limit: 20
    });

    if (result?.Ok) {
        renderAlerts(result.Ok, 'alertsList');
        renderAlerts(result.Ok.slice(0, 5), 'recentAlerts');
    }
}

function renderAlerts(alerts, containerId) {
    const container = document.getElementById(containerId);
    if (!container) return;

    if (!alerts || alerts.length === 0) {
        container.innerHTML = '<div class="empty-state">No alerts found</div>';
        return;
    }

    container.innerHTML = alerts.map(a => `
        <div class="data-item">
            <div class="data-item-main">
                <div class="data-item-title">${a.alert_type}: ${a.description}</div>
                <div class="data-item-meta">
                    <span>📈 ${a.symbol}</span>
                    <span>👤 ${a.entity_id}</span>
                    <span>Score: ${a.risk_score}</span>
                </div>
            </div>
            <span class="badge ${a.severity.toLowerCase()}">${a.severity}</span>
        </div>
    `).join('');
}

async function loadCases() {
    const status = document.getElementById('caseStatusFilter')?.value || 'ALL';
    const result = await callContract('dashboard', 'get_cases_by_status', {
        status: status === 'ALL' ? null : status,
        limit: 20
    });

    if (result?.Ok) {
        renderCases(result.Ok);
    }
}

function renderCases(cases) {
    const container = document.getElementById('casesList');
    if (!container) return;

    if (!cases || cases.length === 0) {
        container.innerHTML = '<div class="empty-state">No cases found</div>';
        return;
    }

    container.innerHTML = cases.map(c => `
        <div class="data-item">
            <div class="data-item-main">
                <div class="data-item-title">${c.case_id} - ${c.subject_entity}</div>
                <div class="data-item-subtitle">${c.summary || c.case_type}</div>
                <div class="data-item-meta">
                    <span>📈 ${c.symbol}</span>
                    <span>👤 ${c.assigned_to || 'Unassigned'}</span>
                </div>
            </div>
            <span class="badge ${c.status.toLowerCase()}">${c.status}</span>
        </div>
    `).join('');
}

async function searchEntity() {
    const entityId = document.getElementById('entitySearch')?.value?.trim();
    if (!entityId) {
        alert('Please enter an entity ID');
        return;
    }

    const entity = await callContract('dashboard', 'search_entities_proxy', { search_query: entityId });
    const container = document.getElementById('entityDetails');

    if (entity?.Ok) {
        const e = entity.Ok;
        container.innerHTML = `
            <div style="display:flex;align-items:center;gap:16px;margin-bottom:16px;">
                <div style="width:48px;height:48px;background:var(--accent-purple);border-radius:50%;display:flex;align-items:center;justify-content:center;font-size:20px;">👤</div>
                <div>
                    <div style="font-size:18px;font-weight:600;">${e.name}</div>
                    <div style="font-size:13px;color:var(--text-secondary);">${e.entity_type} | ${e.entity_id}</div>
                </div>
            </div>
            <div style="display:grid;grid-template-columns:repeat(3,1fr);gap:16px;">
                <div><div style="font-size:12px;color:var(--text-muted);">PAN</div><div>${e.pan_number || 'N/A'}</div></div>
                <div><div style="font-size:12px;color:var(--text-muted);">Registration</div><div>${e.registration_id || 'N/A'}</div></div>
                <div><div style="font-size:12px;color:var(--text-muted);">Type</div><div>${e.entity_type}</div></div>
            </div>
        `;
    } else {
        container.innerHTML = '<div class="empty-state">Entity not found</div>';
    }

    const rels = await callContract('dashboard', 'get_relationships_proxy', { entity_id: entityId });
    const relContainer = document.getElementById('entityRelationships');

    if (rels?.Ok && rels.Ok.length > 0) {
        relContainer.innerHTML = rels.Ok.map(r => `
            <div class="data-item">
                <div class="data-item-main">
                    <div class="data-item-title">${r.target_entity_id}</div>
                    <div class="data-item-subtitle">${r.relationship_detail || ''}</div>
                </div>
                <span class="badge medium">${r.relationship_type}</span>
            </div>
        `).join('');
    } else {
        relContainer.innerHTML = '<div class="empty-state">No relationships found</div>';
    }
}

async function searchTrades() {
    const symbol = document.getElementById('symbolSearch')?.value?.toUpperCase()?.trim();
    if (!symbol) {
        alert('Please enter a symbol');
        return;
    }

    const result = await callContract('dashboard', 'get_trades_proxy', { symbol, limit: 20 });
    const container = document.getElementById('tradesList');

    if (result?.Ok && result.Ok.length > 0) {
        container.innerHTML = result.Ok.map(t => `
            <div class="data-item">
                <div class="data-item-main">
                    <div class="data-item-title">${t.symbol} - ${t.trade_type}</div>
                    <div class="data-item-meta">
                        <span>Qty: ${t.quantity}</span>
                        <span>Price: ${t.price}</span>
                        <span>Account: ${t.account_id}</span>
                    </div>
                </div>
                <span class="badge ${t.trade_type === 'BUY' ? 'open' : 'closed'}">${t.trade_type}</span>
            </div>
        `).join('');
    } else {
        container.innerHTML = '<div class="empty-state">No trades found</div>';
    }
}

async function analyzeVolume() {
    const symbol = document.getElementById('symbolSearch')?.value?.toUpperCase()?.trim();
    if (!symbol) return;

    const result = await callContract('dashboard', 'analyze_volume_proxy', { symbol });
    const container = document.getElementById('tradeAnalysis');

    if (result?.Ok) {
        container.classList.add('visible');
        const a = result.Ok;
        container.innerHTML = `
            <h3 style="margin-bottom:16px;">Volume Analysis: ${symbol}</h3>
            <div class="analysis-grid">
                <div><div class="analysis-value">${a.total_volume || 0}</div><div class="analysis-label">Total Volume</div></div>
                <div><div class="analysis-value">${a.avg_price || 'N/A'}</div><div class="analysis-label">Avg Price</div></div>
                <div><div class="analysis-value">${a.trade_count || 0}</div><div class="analysis-label">Trade Count</div></div>
                <div><div class="analysis-value">${a.concentration_ratio || 'N/A'}</div><div class="analysis-label">Concentration</div></div>
            </div>
        `;
    }
}

// ===== Demo Data (shown before wallet connection) =====
function loadDemoData() {
    document.getElementById('totalAlerts').textContent = '12';
    document.getElementById('openCases').textContent = '5';
    document.getElementById('riskEntities').textContent = '3';
    document.getElementById('completedWorkflows').textContent = '8';

    const demoAlerts = [
        { alert_type: 'INSIDER', severity: 'CRITICAL', symbol: 'RELIANCE', entity_id: 'ENT-0001', risk_score: 92, description: 'Pre-announcement heavy buying' },
        { alert_type: 'WASH_TRADE', severity: 'HIGH', symbol: 'TCS', entity_id: 'ENT-0002', risk_score: 78, description: 'Circular trading pattern' },
        { alert_type: 'SPOOFING', severity: 'MEDIUM', symbol: 'INFY', entity_id: 'ENT-0003', risk_score: 65, description: 'Order cancellation pattern' }
    ];
    renderAlerts(demoAlerts, 'recentAlerts');
    renderAlerts(demoAlerts, 'alertsList');

    const demoCases = [
        { case_id: 'CASE-2026-0001', subject_entity: 'Rajesh Kumar', symbol: 'RELIANCE', status: 'INVESTIGATING', summary: 'Insider trading investigation', assigned_to: 'Anil Verma' },
        { case_id: 'CASE-2026-0002', subject_entity: 'Priya Sharma', symbol: 'TCS', status: 'OPEN', summary: 'Wash trading suspicion', assigned_to: 'Sunita Rao' }
    ];
    renderCases(demoCases);
}
