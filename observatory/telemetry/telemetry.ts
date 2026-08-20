// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

export interface TelemetryDataPoint {
  timestamp: number;
  value: number;
  source: string;
}

export interface TelemetryStreamOptions {
  maxHistorySize: number;
  flushIntervalMs: number;
}

export class TelemetryStream {
  private dataPoints: TelemetryDataPoint[];
  private options: TelemetryStreamOptions;

  constructor(options: TelemetryStreamOptions) {
    this.options = options;
    this.dataPoints = [];
  }

  public record(dataPoint: TelemetryDataPoint): void {
    this.dataPoints.push(dataPoint);
    if (this.dataPoints.length > this.options.maxHistorySize) {
      this.dataPoints.shift();
    }
  }

  public snapshot(): TelemetryDataPoint[] {
    return [...this.dataPoints];
  }
}

export function createTelemetryStream(options: TelemetryStreamOptions): TelemetryStream {
  return new TelemetryStream(options);
}
