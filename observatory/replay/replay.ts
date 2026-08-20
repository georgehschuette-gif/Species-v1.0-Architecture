// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

export interface ReplayFrame {
  frameIndex: number;
  timestamp: number;
  payload: unknown;
}

export interface ReplaySessionOptions {
  playbackSpeed: number;
  loop: boolean;
}

export class ReplaySession {
  private frames: ReplayFrame[];
  private options: ReplaySessionOptions;
  private currentFrameIndex: number;

  constructor(frames: ReplayFrame[], options: ReplaySessionOptions) {
    this.frames = frames;
    this.options = options;
    this.currentFrameIndex = 0;
  }

  public play(): void {
    if (this.frames.length === 0) return;
    while (this.currentFrameIndex < this.frames.length) {
      const frame = this.frames[this.currentFrameIndex];
      this.currentFrameIndex++;
      // process frame
    }
    if (this.options.loop) {
      this.currentFrameIndex = 0;
      this.play();
    }
  }

  public seek(frameIndex: number): void {
    if (frameIndex < 0 || frameIndex >= this.frames.length) {
      throw new Error('Frame index out of bounds');
    }
    this.currentFrameIndex = frameIndex;
  }
}

export function createReplaySession(frames: ReplayFrame[], options: ReplaySessionOptions): ReplaySession {
  return new ReplaySession(frames, options);
}
