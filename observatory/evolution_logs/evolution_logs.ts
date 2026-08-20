// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

export interface EvolutionLogEntry {
  generation: number;
  timestamp: number;
  description: string;
  delta: Record<string, unknown>;
}

export interface EvolutionLogOptions {
  maxEntries: number;
}

export class EvolutionLog {
  private entries: EvolutionLogEntry[];
  private options: EvolutionLogOptions;

  constructor(options: EvolutionLogOptions) {
    this.options = options;
    this.entries = [];
  }

  public append(entry: EvolutionLogEntry): void {
    this.entries.push(entry);
    while (this.entries.length > this.options.maxEntries) {
      this.entries.shift();
    }
  }

  public tail(count: number): EvolutionLogEntry[] {
    return this.entries.slice(-count);
  }
}

export function createEvolutionLog(options: EvolutionLogOptions): EvolutionLog {
  return new EvolutionLog(options);
}
