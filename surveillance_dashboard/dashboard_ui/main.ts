
import { DashboardWebserver } from './bindings';
import { WeilWalletConnection } from '@weilliptic/weil-sdk';

const DASHBOARD_CONTRACT_ID = 'aaaaaayrd3bsxvttmltgyzjr34uzesbbqqzris3dlvskwh3asoj7s4xnsu';

let wallet: WeilWalletConnection | null = null;
let dashboard: ReturnType<typeof DashboardWebserver> | null = null;
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
            const panel = (item as HTMLElement).dataset.panel;
            document.querySelectorAll('.nav-item').forEach(n => n.classList.remove('active'));
            item.classList.add('active');
            document.querySelectorAll('.panel').forEach(p => p.classList.remove('active'));
            document.getElementById('panel-' + panel)?.classList.add('active');
        });
    });
}

// ===== Event Listeners =====
function setupEventListeners() {
    document.getElementById('connectWallet')?.addEventListener('click', connectWallet);
    document.getElementById('refreshAlerts')?.addEventListener('click', loadAlerts);
    document.getElementById('refreshCases')?.addEventListener('click', loadCases);
    document.getElementById('searchEntity')?.addEventListener('click', searchEntity);
    document.getElementById('searchTrades')?.addEventListener('click', searchTrades);
    document.getElementById('analyzeVolume')?.addEventListener('click', analyzeVolume);
}

// ===== Wallet Connection =====
async function connectWallet() {
    console.log('Connecting wallet...');

    if (typeof (window as any).WeilWallet === 'undefined') {
        alert('WeilWallet extension not found. Please install it.');
        return;
    }

    try {
        wallet = new WeilWalletConnection({
            walletProvider: (window as any).WeilWallet,
        });
        console.log('Wallet wrapper created');

        dashboard = DashboardWebserver(wallet, DASHBOARD_CONTRACT_ID);
        connected = true;

        const statusDot = document.querySelector('.status-dot');
        const statusText = document.querySelector('.status-text');
        const connectBtn = document.getElementById('connectWallet') as HTMLButtonElement;

        if (statusDot) statusDot.className = 'status-dot online';
        if (statusText) statusText.textContent = 'Connected';
        if (connectBtn) {
            connectBtn.textContent = '✓ Connected';
            connectBtn.disabled = true;
        }

        await loadAllData();
    } catch (error: any) {
        console.error('Wallet connection failed:', error);
        alert('Connection failed: ' + error.message);
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

function unwrap(data: any): any {
    if (data && typeof data === 'object' && 'Ok' in data) {
        return data.Ok;
    }
    return data;
}

async function loadStats() {
    if (!dashboard) return;

    try {
        console.log('Loading stats...');
        let stats = await dashboard.get_stats();
        console.log('Stats result (raw):', stats);
        stats = unwrap(stats);
        console.log('Stats result (unwrapped):', stats);

        if (stats) {
            const totalAlerts = document.getElementById('totalAlerts');
            const openCases = document.getElementById('openCases');
            const riskEntities = document.getElementById('riskEntities');
            const completedWorkflows = document.getElementById('completedWorkflows');

            if (totalAlerts) totalAlerts.textContent = String(stats.total_alerts_today || 0);
            if (openCases) openCases.textContent = String(stats.open_cases || 0);
            if (riskEntities) riskEntities.textContent = String(stats.high_risk_entities || 0);
            if (completedWorkflows) completedWorkflows.textContent = String(stats.total_workflows_today || 0);
        }
    } catch (error) {
        console.error('Error loading stats:', error);
    }
}

async function loadAlerts() {
    if (!dashboard) return;

    try {
        const severityFilter = (document.getElementById('alertSeverityFilter') as HTMLSelectElement)?.value || 'ALL';
        console.log('Loading alerts with filter:', severityFilter);

        let alerts = await dashboard.get_live_alerts(
            severityFilter === 'ALL' ? undefined : severityFilter,
            20
        );
        alerts = unwrap(alerts);
        console.log('Alerts result:', alerts);

        if (alerts && alerts.length > 0) {
            renderAlerts(alerts, 'alertsList');
            renderAlerts(alerts.slice(0, 5), 'recentAlerts');
        } else {
            renderAlerts([], 'alertsList');
        }
    } catch (error) {
        console.error('Error loading alerts:', error);
    }
}

function renderAlerts(alerts: any[], containerId: string) {
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
            <span class="badge ${String(a.severity).toLowerCase()}">${a.severity}</span>
        </div>
    `).join('');
}

async function loadCases() {
    if (!dashboard) return;

    try {
        const statusFilter = (document.getElementById('caseStatusFilter') as HTMLSelectElement)?.value || 'ALL';
        console.log('Loading cases with filter:', statusFilter);

        let cases = await dashboard.get_cases_by_status(
            statusFilter === 'ALL' ? undefined : statusFilter,
            20
        );
        cases = unwrap(cases);
        console.log('Cases result:', cases);

        if (cases) {
            renderCases(cases);
        }
    } catch (error) {
        console.error('Error loading cases:', error);
    }
}

function renderCases(cases: any[]) {
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
            <span class="badge ${String(c.status).toLowerCase()}">${c.status}</span>
        </div>
    `).join('');
}

async function searchEntity() {
    if (!dashboard) return;

    const entityId = (document.getElementById('entitySearch') as HTMLInputElement)?.value?.trim();
    if (!entityId) {
        alert('Please enter an entity ID');
        return;
    }

    try {
        const entities = await dashboard.search_entities_proxy(entityId);
        const container = document.getElementById('entityDetails');

        if (entities && entities.length > 0) {
            const e = entities[0];
            if (container) {
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
            }

            const rels = await dashboard.get_relationships_proxy(e.entity_id);
            const relContainer = document.getElementById('entityRelationships');

            if (rels && rels.length > 0 && relContainer) {
                relContainer.innerHTML = rels.map(r => `
                    <div class="data-item">
                        <div class="data-item-main">
                            <div class="data-item-title">${r.target_entity_id}</div>
                            <div class="data-item-subtitle">${r.relationship_detail || ''}</div>
                        </div>
                        <span class="badge medium">${r.relationship_type}</span>
                    </div>
                `).join('');
            } else if (relContainer) {
                relContainer.innerHTML = '<div class="empty-state">No relationships found</div>';
            }
        } else if (container) {
            container.innerHTML = '<div class="empty-state">Entity not found</div>';
        }
    } catch (error) {
        console.error('Error searching entity:', error);
    }
}

async function searchTrades() {
    if (!dashboard) return;

    const symbol = (document.getElementById('symbolSearch') as HTMLInputElement)?.value?.toUpperCase()?.trim();
    if (!symbol) {
        alert('Please enter a symbol');
        return;
    }

    try {
        const trades = await dashboard.get_trades_proxy(symbol, 20);
        const container = document.getElementById('tradesList');

        if (trades && trades.length > 0 && container) {
            container.innerHTML = trades.map(t => `
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
        } else if (container) {
            container.innerHTML = '<div class="empty-state">No trades found</div>';
        }
    } catch (error) {
        console.error('Error searching trades:', error);
    }
}

async function analyzeVolume() {
    if (!dashboard) return;

    const symbol = (document.getElementById('symbolSearch') as HTMLInputElement)?.value?.toUpperCase()?.trim();
    if (!symbol) return;

    try {
        const analysis = await dashboard.analyze_volume_proxy(symbol);
        const container = document.getElementById('tradeAnalysis');

        if (analysis && container) {
            container.classList.add('visible');
            container.innerHTML = `
                <h3 style="margin-bottom:16px;">Volume Analysis: ${symbol}</h3>
                <div class="analysis-grid">
                    <div><div class="analysis-value">${analysis.total_volume || 0}</div><div class="analysis-label">Total Volume</div></div>
                    <div><div class="analysis-value">${analysis.avg_price || 'N/A'}</div><div class="analysis-label">Avg Price</div></div>
                    <div><div class="analysis-value">${analysis.trade_count || 0}</div><div class="analysis-label">Trade Count</div></div>
                    <div><div class="analysis-value">${analysis.concentration_ratio || 'N/A'}</div><div class="analysis-label">Concentration</div></div>
                </div>
            `;
        }
    } catch (error) {
        console.error('Error analyzing volume:', error);
    }
}

function loadDemoData() {
    const totalAlerts = document.getElementById('totalAlerts');
    const openCases = document.getElementById('openCases');
    const riskEntities = document.getElementById('riskEntities');
    const completedWorkflows = document.getElementById('completedWorkflows');

    if (totalAlerts) totalAlerts.textContent = '12';
    if (openCases) openCases.textContent = '5';
    if (riskEntities) riskEntities.textContent = '3';
    if (completedWorkflows) completedWorkflows.textContent = '8';

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
