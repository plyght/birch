export interface RotateResult {
  success: boolean;
  new_value?: string;
  pool_status?: PoolStatus;
  message?: string;
}

export interface PoolStatus {
  total_keys: number;
  available_keys: number;
  exhausted_keys: number;
  current_index: number;
}

export interface RotationInfo {
  secretName: string;
  timestamp: Date;
  success: boolean;
}

