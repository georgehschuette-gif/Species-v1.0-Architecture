// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

export interface EcosystemMetricValue {
  metricName: string;
  value: number;
  unit?: string;
}

export interface EcosystemMetricsCollectorOptions {
  collectionIntervalMs: number;
}

export class EcosystemMetricsCollector {
  private options: EcosystemMetricsCollectorOptions;
  private metrics: Map<string, EcosystemMetricValue[]>;

  constructor(options: EcosystemMetricsCollectorOptions) {
    this.options = options;
    this.metrics = new Map();
  }

  public record(metric: EcosystemMetricValue): void {
    const bucket = this.metrics.get(metric.metricName) || [];
    bucket.push(metric);
    this.metrics.set(metric.metricName, bucket);
  }

  public summarize(metricName: string): { sum: number; count: number } | undefined {
    const bucket = this.metrics.get(metricName);
    if (!bucket || bucket.length === 0) return undefined;
    const sum = bucket.reduce((acc, m) => acc + m.value, 0);
    return { sum, count: bucket.length };
  }
}

export function createEcosystemMetricsCollector(options: EcosystemMetricsCollectorOptions): EcosystemMetricsCollector {
  return new EcosystemMetricsCollector(options);
}
