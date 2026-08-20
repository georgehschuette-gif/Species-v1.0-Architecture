export interface IRobot {
  id: string;
  name: string;
  jointCount: number;
}

export interface JointState {
  index: number;
  angle: number;
  velocity: number;
  torque: number;
}

export interface Pose {
  x: number;
  y: number;
  z: number;
  roll: number;
  pitch: number;
  yaw: number;
}

export interface IRoboticsService {
  registerRobot(robot: IRobot): void;
  getJointState(robotId: string, jointIndex: number): Promise<JointState>;
  setPose(robotId: string, pose: Pose): Promise<void>;
}

export class RoboticsService implements IRoboticsService {
  private robots: Map<string, IRobot>;

  constructor() {
    this.robots = new Map();
  }

  public registerRobot(robot: IRobot): void {
    this.roboticsCheck(robot);
    this.robots.set(robot.id, robot);
  }

  public async getJointState(robotId: string, jointIndex: number): Promise<JointState> {
    this.ensureRobotExists(robotId);
    if (jointIndex < 0) {
      throw new Error('Joint index must be non-negative');
    }
    return {
      index: jointIndex,
      angle: 0,
      velocity: 0,
      torque: 0,
    };
  }

  public async setPose(robotId: string, pose: Pose): Promise<void> {
    this.ensureRobotExists(robotId);
    if (Math.abs(pose.roll) > Math.PI || Math.abs(pose.pitch) > Math.PI) {
      throw new Error('Pose exceeds joint limits');
    }
  }

  private roboticsCheck(robot: IRobot): void {
    if (!robot.id.trim()) {
      throw new Error('Robot id is required');
    }
    if (robot.jointCount < 1) {
      throw new Error('Robot must have at least one joint');
    }
  }

  private ensureRobotExists(robotId: string): void {
    if (!this.robots.has(robotId)) {
      throw new Error(`Unknown robot: ${robotId}`);
    }
  }
}

export const createRoboticsService = (): IRoboticsService => {
  return new RoboticsService();
};
