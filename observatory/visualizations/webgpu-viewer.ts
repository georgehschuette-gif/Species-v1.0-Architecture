// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

export interface WebGPUViewerOptions {
  width: number;
  height: number;
  powerPreference: 'high-performance' | 'low-power';
}

export interface GPUDeviceStub {
  adapter: GPUAdapter;
  device: GPUDevice;
  format: GPUTextureFormat;
}

export class WebGPUViewer {
  private options: WebGPUViewerOptions;
  private device: GPUDeviceStub | null;

  constructor(options: WebGPUViewerOptions) {
    this.options = options;
    this.device = null;
  }

  public async initialize(): Promise<GPUDeviceStub> {
    if (typeof navigator === 'undefined' || !navigator.gpu) {
      throw new Error('WebGPU is not available in this environment');
    }
    const adapter = await navigator.gpu.requestAdapter({
      powerPreference: this.options.powerPreference,
    });
    if (!adapter) {
      throw new Error('Failed to request GPU adapter');
    }
    const device = await adapter.requestDevice();
    this.device = {
      adapter,
      device,
      format: navigator.gpu.getPreferredCanvasFormat(),
    };
    return this.device;
  }

  public getDevice(): GPUDeviceStub | null {
    return this.device;
  }
}

export function createWebGPUViewer(options: WebGPUViewerOptions): WebGPUViewer {
  return new WebGPUViewer(options);
}
