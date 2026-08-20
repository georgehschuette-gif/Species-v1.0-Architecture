export interface ISimulationService {
  createScenario(name: string): Scenario;
  step(scenario: Scenario, deltaMs: number): Promise<ScenarioState>;
}

export interface Scenario {
  id: string;
  name: string;
  timeStepMs: number;
}

export interface ScenarioState {
  simulationTimeMs: number;
  tick: number;
  running: boolean;
  entities: Record<string, unknown>;
}

export class SimulationService implements ISimulationService {
  private scenarios: Map<string, Scenario>;
  private counter: number;

  constructor() {
    this.scenarios = new Map();
    this.counter = 0;
  }

  public createScenario(name: string): Scenario {
    if (!name.trim()) {
      throw new Error('Scenario name is required');
    }
    this.counter += 1;
    const scenario: Scenario = {
      id: `scenario-${this.counter}`,
      name,
      timeStepMs: 16,
    };
    this.scenarios.set(scenario.id, scenario);
    return scenario;
  }

  public async step(scenario: Scenario, deltaMs: number): Promise<ScenarioState> {
    if (!this.scenarios.has(scenario.id)) {
      throw new Error(`Unknown scenario: ${scenario.id}`);
    }
    if (deltaMs <= 0) {
      throw new Error('Delta time must be positive');
    }
    return {
      simulationTimeMs: Date.now(),
      tick: this.counter,
      running: true,
      entities: {},
    };
  }
}

export const createSimulationService = (): ISimulationService => {
  return new SimulationService();
};
