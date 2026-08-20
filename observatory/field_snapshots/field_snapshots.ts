// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

export interface FieldSnapshot {
  id: string;
  capturedAt: number;
  fieldState: Record<string, unknown>;
  checksum: string;
}

export interface FieldSnapshotStoreOptions {
  maxSnapshots: number;
}

export class FieldSnapshotStore {
  private snapshots: FieldSnapshot[];
  private options: FieldSnapshotStoreOptions;

  constructor(options: FieldSnapshotStoreOptions) {
    this.options = options;
    this.snapshots = [];
  }

  public store(snapshot: FieldSnapshot): void {
    this.snapshots.push(snapshot);
    while (this.snapshots.length > this.options.maxSnapshots) {
      this.snapshots.shift();
    }
  }

  public retrieveLatest(): FieldSnapshot | undefined {
    return this.snapshots[this.snapshots.length - 1];
  }
}

export function createFieldSnapshotStore(options: FieldSnapshotStoreOptions): FieldSnapshotStore {
  return new FieldSnapshotStore(options);
}
