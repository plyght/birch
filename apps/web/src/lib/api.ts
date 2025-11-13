import { supabase } from './supabase'

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000'

export class ApiError extends Error {
  constructor(
    message: string,
    public status: number,
    public data?: any
  ) {
    super(message)
    this.name = 'ApiError'
  }
}

async function getAuthToken(): Promise<string | null> {
  const {
    data: { session },
    error,
  } = await supabase.auth.getSession()

  if (error) {
    console.error('Error getting session:', error)
    return null
  }

  return session?.access_token ?? null
}

async function apiRequest<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  const token = await getAuthToken()

  if (!token) {
    throw new ApiError('Not authenticated', 401)
  }

  const url = `${API_URL}${endpoint}`
  const headers = {
    'Content-Type': 'application/json',
    Authorization: `Bearer ${token}`,
    ...options.headers,
  }

  try {
    const response = await fetch(url, {
      ...options,
      headers,
    })

    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}))
      throw new ApiError(
        errorData.message || `HTTP ${response.status}: ${response.statusText}`,
        response.status,
        errorData
      )
    }

    // Handle 204 No Content
    if (response.status === 204) {
      return null as T
    }

    return await response.json()
  } catch (error) {
    if (error instanceof ApiError) {
      throw error
    }
    throw new ApiError(
      error instanceof Error ? error.message : 'Network request failed',
      0
    )
  }
}

// Workspace API
export const workspaceApi = {
  async list() {
    return apiRequest<Array<{
      id: string
      name: string
      plan_tier: string
      created_at: string
    }>>('/api/v1/workspaces')
  },

  async get(id: string) {
    return apiRequest<{
      id: string
      name: string
      plan_tier: string
      created_at: string
    }>(`/api/v1/workspaces/${id}`)
  },

  async create(data: { name: string }) {
    return apiRequest<{
      workspace: {
        id: string
        name: string
        plan_tier: string
        created_at: string
      }
    }>('/api/v1/workspaces', {
      method: 'POST',
      body: JSON.stringify(data),
    })
  },

  async update(id: string, data: { name?: string; plan_tier?: string }) {
    return apiRequest<{
      workspace: {
        id: string
        name: string
        plan_tier: string
        created_at: string
      }
    }>(`/api/v1/workspaces/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    })
  },

  async delete(id: string) {
    return apiRequest(`/api/v1/workspaces/${id}`, {
      method: 'DELETE',
    })
  },
}

// Provider API
export const providerApi = {
  async list(workspaceId: string) {
    return apiRequest<Array<{
      id: string
      workspace_id: string
      provider: string
      mode: string
      created_at: string
    }>>(`/api/v1/workspaces/${workspaceId}/providers`)
  },

  async get(workspaceId: string, provider: string) {
    return apiRequest<{
      id: string
      workspace_id: string
      provider: string
      mode: string
      config: Record<string, any>
      created_at: string
    }>(`/api/v1/workspaces/${workspaceId}/providers/${provider}`)
  },

  async create(
    workspaceId: string,
    data: {
      provider: string
      mode: string
      config: Record<string, any>
    }
  ) {
    return apiRequest<{
      config: {
        id: string
        workspace_id: string
        provider: string
        mode: string
        created_at: string
      }
    }>(`/api/v1/workspaces/${workspaceId}/providers`, {
      method: 'POST',
      body: JSON.stringify(data),
    })
  },

  async update(
    workspaceId: string,
    provider: string,
    data: {
      mode?: string
      config?: Record<string, any>
    }
  ) {
    return apiRequest<{
      config: {
        id: string
        workspace_id: string
        provider: string
        mode: string
        created_at: string
      }
    }>(`/api/v1/workspaces/${workspaceId}/providers/${provider}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    })
  },

  async delete(workspaceId: string, provider: string) {
    return apiRequest(`/api/v1/workspaces/${workspaceId}/providers/${provider}`, {
      method: 'DELETE',
    })
  },
}

// Credentials API
export const credentialsApi = {
  async store(
    workspaceId: string,
    data: {
      provider: string
      secret_name: string
      value: string
    }
  ) {
    return apiRequest(`/api/v1/workspaces/${workspaceId}/credentials`, {
      method: 'POST',
      body: JSON.stringify(data),
    })
  },

  async get(workspaceId: string, provider: string, secretName: string) {
    return apiRequest<{ value: string }>(
      `/api/v1/workspaces/${workspaceId}/credentials/${provider}/${secretName}`
    )
  },
}

// Members API
export const membersApi = {
  async list(workspaceId: string) {
    return apiRequest<Array<{
      user_id: string
      role: string
      email?: string
      joined_at: string
    }>>(`/api/v1/workspaces/${workspaceId}/members`)
  },

  async add(workspaceId: string, data: { email: string; role: string }) {
    return apiRequest(`/api/v1/workspaces/${workspaceId}/members`, {
      method: 'POST',
      body: JSON.stringify(data),
    })
  },

  async updateRole(workspaceId: string, userId: string, role: string) {
    return apiRequest(`/api/v1/workspaces/${workspaceId}/members/${userId}`, {
      method: 'PUT',
      body: JSON.stringify({ role }),
    })
  },

  async remove(workspaceId: string, userId: string) {
    return apiRequest(`/api/v1/workspaces/${workspaceId}/members/${userId}`, {
      method: 'DELETE',
    })
  },
}

// API Keys API
export const apiKeysApi = {
  async list(workspaceId: string) {
    return apiRequest<Array<{
      id: string
      name: string
      prefix: string
      created_at: string
      last_used_at?: string
      revoked_at?: string
    }>>(`/api/v1/workspaces/${workspaceId}/api-keys`)
  },

  async create(workspaceId: string, data: { name: string }) {
    return apiRequest<{
      api_key: {
        id: string
        key: string
        name: string
        prefix: string
        created_at: string
      }
    }>(`/api/v1/workspaces/${workspaceId}/api-keys`, {
      method: 'POST',
      body: JSON.stringify(data),
    })
  },

  async revoke(workspaceId: string, keyId: string) {
    return apiRequest(`/api/v1/workspaces/${workspaceId}/api-keys/${keyId}`, {
      method: 'DELETE',
    })
  },
}

export const api = {
  workspaces: workspaceApi,
  providers: providerApi,
  credentials: credentialsApi,
  members: membersApi,
  apiKeys: apiKeysApi,
}

export default api

