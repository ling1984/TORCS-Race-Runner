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