import request from '@/utils/request';
import type {
  LoginStepOneResponse,
  TotpVerifyRequest,
  TotpSetupResponse,
  TotpStatusResponse,
  TotpEnableRequest,
} from '@/types/totp';

export const authApi = {
  // 第一步登录：验证用户名密码
  login: (username: string, password: string) =>
    request.post<LoginStepOneResponse>('/auth/login', { username, password }),

  // 第二步登录：验证TOTP码
  verifyTotp: (data: TotpVerifyRequest) =>
    request.post<{ token: string; expires_in: number }>('/auth/totp/verify', data),

  // 获取TOTP状态
  getTotpStatus: () =>
    request.get<TotpStatusResponse>('/auth/totp/status'),

  // 初始化TOTP设置
  setupTotp: () =>
    request.post<TotpSetupResponse>('/auth/totp/setup'),

  // 启用TOTP
  enableTotp: (data: TotpEnableRequest) =>
    request.post<{ success: boolean }>('/auth/totp/enable', data),

  // 禁用TOTP
  disableTotp: () =>
    request.post<{ success: boolean }>('/auth/totp/disable'),

  // 重新生成备用码
  regenerateBackupCodes: () =>
    request.post<{ backup_codes: string[] }>('/auth/totp/regenerate-backup-codes'),
};
