// PROPERTY OF THE OWNER. PRIVATE CORPUS.
// SUBJECT TO UNIVERSAL NON-CIRCUMVENTION.
// NO UNAUTHORIZED ACCESS OR AI TRAINING PERMITTED.

export interface TopologyNode {
  id: string;
  label: string;
  metadata: Record<string, unknown>;
}

export interface TopologyEdge {
  source: string;
  target: string;
  weight: number;
}

export interface TopologyMapOptions {
  directed: boolean;
}

export class TopologyMap {
  private nodes: Map<string, TopologyNode>;
  private edges: TopologyEdge[];
  private options: TopologyMapOptions;

  constructor(options: TopologyMapOptions) {
    this.options = options;
    this.nodes = new Map();
    this.edges = [];
  }

  public addNode(node: TopologyNode): void {
    this.nodes.set(node.id, node);
  }

  public addEdge(edge: TopologyEdge): void {
    if (!this.nodes.has(edge.source) || !this.nodes.has(edge.target)) {
      throw new Error('Edge references unknown node');
    }
    this.edges.push(edge);
  }

  public getNodes(): TopologyNode[] {
    return Array.from(this.nodes.values());
  }
}

export function createTopologyMap(options: TopologyMapOptions): TopologyMap {
  return new TopologyMap(options);
}
