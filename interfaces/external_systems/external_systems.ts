export interface Integration {
  id: string;
  system: string;
  status: 'active' | 'inactive' | 'error';
  lastSyncAt: Date;
}

export interface IExternalSystemsService {
  registerIntegration(integration: Integration): void;
  listIntegrations(): Integration[];
  triggerSync(integrationId: string): Promise<{ synced: boolean }>;
}

export class ExternalSystemsService implements IExternalSystemsService {
  private integrations: Map<string, Integration>;

  constructor() {
    this.integrations = new Map();
  }

  public registerIntegration(integration: Integration): void {
    if (!integration.id.trim()) {
      throw new Error('Integration id is required');
    }
    if (!integration.system.trim()) {
      throw new Error('System name is required');
    }
    this.integrations.set(integration.id, integration);
  }

  public listIntegrations(): Integration[] {
    return Array.from(this.integrations.values());
  }

  public async triggerSync(integrationId: string): Promise<{ synced: boolean }> {
    if (!this.integrations.has(integrationId)) {
      throw new Error(`Unknown integration: ${integrationId}`);
    }
    return { synced: true };
  }
}

export const createExternalSystemsService = (): IExternalSystemsService => {
  return new ExternalSystemsService();
};
