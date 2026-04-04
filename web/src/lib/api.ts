import type {
  StatusResponse,
  ToolSpec,
  CronJob,
  Integration,
  IntegrationSettingsPayload,
  DiagResult,
  MemoryEntry,
  PairedDevice,
  CostSummary,
  CliTool,
  HealthSnapshot,
} from '../types/api';
import { clearToken, getToken, setToken } from './auth';

// ---------------------------------------------------------------------------
// Base fetch wrapper
// ---------------------------------------------------------------------------

export class UnauthorizedError extends Error {
  constructor() {
    super('Unauthorized');
    this.name = 'UnauthorizedError';
  }
}

export async function apiFetch<T = unknown>(
  path: string,
  options: RequestInit = {},
): Promise<T> {
  const token = getToken();
  const headers = new Headers(options.headers);

  if (token) {
    headers.set('Authorization', `Bearer ${token}`);
  }

  if (
    options.body &&
    typeof options.body === 'string' &&
    !headers.has('Content-Type')
  ) {
    headers.set('Content-Type', 'application/json');
  }

  const response = await fetch(path, { ...options, headers });

  if (response.status === 401) {
    clearToken();
    window.dispatchEvent(new Event('zeroclaw-unauthorized'));
    throw new UnauthorizedError();
  }

  if (!response.ok) {
    const text = await response.text().catch(() => '');
    throw new Error(`API ${response.status}: ${text || response.statusText}`);
  }

  // Some endpoints may return 204 No Content
  if (response.status === 204) {
    return undefined as unknown as T;
  }

  return response.json() as Promise<T>;
}

function unwrapField<T>(value: T | Record<string, T>, key: string): T {
  if (value !== null && typeof value === 'object' && !Array.isArray(value) && key in value) {
    const unwrapped = (value as Record<string, T | undefined>)[key];
    if (unwrapped !== undefined) {
      return unwrapped;
    }
  }
  return value as T;
}

function normalizeMemoryCategory(raw: unknown): string {
  if (typeof raw === 'string') {
    return raw;
  }
  if (raw && typeof raw === 'object') {
    const custom = (raw as Record<string, unknown>).custom;
    if (typeof custom === 'string' && custom.trim().length > 0) {
      return custom;
    }
  }
  return 'unknown';
}

function normalizeMemoryEntry(raw: unknown): MemoryEntry {
  const entry = (raw ?? {}) as Record<string, unknown>;
  const key = typeof entry.key === 'string' ? entry.key : '';
  const category = normalizeMemoryCategory(entry.category);
  const fallbackId = `${key || 'memory'}:${category}`;
  return {
    id: typeof entry.id === 'string' && entry.id.length > 0 ? entry.id : fallbackId,
    key,
    content: typeof entry.content === 'string' ? entry.content : String(entry.content ?? ''),
    category,
    timestamp: typeof entry.timestamp === 'string' ? entry.timestamp : '',
    session_id: typeof entry.session_id === 'string' ? entry.session_id : null,
    score: typeof entry.score === 'number' ? entry.score : null,
  };
}

function normalizeMemoryEntries(raw: unknown): MemoryEntry[] {
  if (!Array.isArray(raw)) {
    return [];
  }
  return raw.map((entry) => normalizeMemoryEntry(entry));
}

// ---------------------------------------------------------------------------
// Pairing
// ---------------------------------------------------------------------------

export async function pair(code: string): Promise<{ token: string }> {
  const response = await fetch('/pair', {
    method: 'POST',
    headers: { 'X-Pairing-Code': code },
  });

  if (!response.ok) {
    const text = await response.text().catch(() => '');
    throw new Error(`Pairing failed (${response.status}): ${text || response.statusText}`);
  }

  const data = (await response.json()) as { token: string };
  setToken(data.token);
  return data;
}

// ---------------------------------------------------------------------------
// Public health (no auth required)
// ---------------------------------------------------------------------------

export async function getPublicHealth(): Promise<{ require_pairing: boolean; paired: boolean }> {
  const response = await fetch('/health');
  if (!response.ok) {
    throw new Error(`Health check failed (${response.status})`);
  }
  return response.json() as Promise<{ require_pairing: boolean; paired: boolean }>;
}

// ---------------------------------------------------------------------------
// Status / Health
// ---------------------------------------------------------------------------

export function getStatus(): Promise<StatusResponse> {
  return apiFetch<StatusResponse>('/api/status');
}

export function getHealth(): Promise<HealthSnapshot> {
  return apiFetch<HealthSnapshot | { health: HealthSnapshot }>('/api/health').then((data) =>
    unwrapField(data, 'health'),
  );
}

// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

export function getConfig(): Promise<string> {
  return apiFetch<string | { format?: string; content: string }>('/api/config').then((data) =>
    typeof data === 'string' ? data : data.content,
  );
}

export function putConfig(toml: string): Promise<void> {
  return apiFetch<void>('/api/config', {
    method: 'PUT',
    headers: { 'Content-Type': 'application/toml' },
    body: toml,
  });
}

// ---------------------------------------------------------------------------
// Tools
// ---------------------------------------------------------------------------

export function getTools(): Promise<ToolSpec[]> {
  return apiFetch<ToolSpec[] | { tools: ToolSpec[] }>('/api/tools').then((data) =>
    unwrapField(data, 'tools'),
  );
}

// ---------------------------------------------------------------------------
// Cron
// ---------------------------------------------------------------------------

export function getCronJobs(): Promise<CronJob[]> {
  return apiFetch<CronJob[] | { jobs: CronJob[] }>('/api/cron').then((data) =>
    unwrapField(data, 'jobs'),
  );
}

export function addCronJob(body: {
  name?: string;
  command: string;
  schedule: string;
  enabled?: boolean;
}): Promise<CronJob> {
  return apiFetch<CronJob | { status: string; job: CronJob }>('/api/cron', {
    method: 'POST',
    body: JSON.stringify(body),
  }).then((data) => (typeof (data as { job?: CronJob }).job === 'object' ? (data as { job: CronJob }).job : (data as CronJob)));
}

export function deleteCronJob(id: string): Promise<void> {
  return apiFetch<void>(`/api/cron/${encodeURIComponent(id)}`, {
    method: 'DELETE',
  });
}

// ---------------------------------------------------------------------------
// Integrations
// ---------------------------------------------------------------------------

export function getIntegrations(): Promise<Integration[]> {
  return apiFetch<Integration[] | { integrations: Integration[] }>('/api/integrations').then(
    (data) => unwrapField(data, 'integrations'),
  );
}

export function getIntegrationSettings(): Promise<IntegrationSettingsPayload> {
  return apiFetch<IntegrationSettingsPayload>('/api/integrations/settings');
}

export function putIntegrationCredentials(
  integrationId: string,
  body: { revision?: string; fields: Record<string, string> },
): Promise<{ status: string; revision: string; unchanged?: boolean }> {
  return apiFetch<{ status: string; revision: string; unchanged?: boolean }>(
    `/api/integrations/${encodeURIComponent(integrationId)}/credentials`,
    {
      method: 'PUT',
      body: JSON.stringify(body),
    },
  );
}

// ---------------------------------------------------------------------------
// Doctor / Diagnostics
// ---------------------------------------------------------------------------

export function runDoctor(): Promise<DiagResult[]> {
  return apiFetch<DiagResult[] | { results: DiagResult[]; summary?: unknown }>('/api/doctor', {
    method: 'POST',
    body: JSON.stringify({}),
  }).then((data) => (Array.isArray(data) ? data : data.results));
}

// ---------------------------------------------------------------------------
// Memory
// ---------------------------------------------------------------------------

export function getMemory(
  query?: string,
  category?: string,
  limit = 200,
): Promise<MemoryEntry[]> {
  const params = new URLSearchParams();
  if (query) params.set('query', query);
  if (category) params.set('category', category);
  if (Number.isFinite(limit) && limit > 0) {
    params.set('limit', String(Math.min(Math.floor(limit), 1000)));
  }
  const qs = params.toString();
  return apiFetch<MemoryEntry[] | { entries: MemoryEntry[] }>(
    `/api/memory${qs ? `?${qs}` : ''}`,
  ).then((data) => normalizeMemoryEntries(unwrapField(data, 'entries')));
}

export function storeMemory(
  key: string,
  content: string,
  category?: string,
): Promise<void> {
  return apiFetch<unknown>('/api/memory', {
    method: 'POST',
    body: JSON.stringify({ key, content, category }),
  }).then(() => undefined);
}

export function deleteMemory(key: string): Promise<void> {
  return apiFetch<void>(`/api/memory/${encodeURIComponent(key)}`, {
    method: 'DELETE',
  });
}

// ---------------------------------------------------------------------------
// Paired Devices
// ---------------------------------------------------------------------------

export function getPairedDevices(): Promise<PairedDevice[]> {
  return apiFetch<PairedDevice[] | { devices: PairedDevice[] }>('/api/pairing/devices').then(
    (data) => unwrapField(data, 'devices'),
  );
}

export function revokePairedDevice(id: string): Promise<void> {
  return apiFetch<void>(`/api/pairing/devices/${encodeURIComponent(id)}`, {
    method: 'DELETE',
  });
}

// ---------------------------------------------------------------------------
// Cost
// ---------------------------------------------------------------------------

export function getCost(): Promise<CostSummary> {
  return apiFetch<CostSummary | { cost: CostSummary }>('/api/cost').then((data) =>
    unwrapField(data, 'cost'),
  );
}

// ---------------------------------------------------------------------------
// CLI Tools
// ---------------------------------------------------------------------------

export function getCliTools(): Promise<CliTool[]> {
  return apiFetch<CliTool[] | { cli_tools: CliTool[] }>('/api/cli-tools').then((data) =>
    unwrapField(data, 'cli_tools'),
  );
}
