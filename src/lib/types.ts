export interface SliderConfig {
  id: string;
  label: string;
  min?: number;
  max?: number;
  default: number | boolean;
  default2?: number;
  default3?: number;
  default4?: number;
  default5?: number;
  value: number | boolean;
  value2?: number;
  value3?: number;
  value4?: number;
  value5?: number;
  step?: number;
  type: string;
  help: string;
}

export interface DriverParams {
  target_speed: number;
  steer_gain: number;
  centering_gain: number;
  brake_threshold: number;
  gear_thresholds: number[];
  traction_control: boolean;
  team_name: string;
}


export type RaceTeam = {
  logo_path: string;
  name: string;
  script_path: string;
}