'use client'

import { useEffect, useState } from 'react'
import DashboardLayout from '@/components/layout/DashboardLayout'
import Card, { CardHeader } from '@/components/ui/Card'
import Button from '@/components/ui/Button'
import Input from '@/components/ui/Input'
import Modal, { ModalFooter } from '@/components/ui/Modal'
import Table, { TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/Table'
import Badge from '@/components/ui/Badge'
import { api } from '@/lib/api'
import { supabase } from '@/lib/supabase'

export default function SettingsPage() {
  const [user, setUser] = useState<any>(null)
  const [workspaces, setWorkspaces] = useState<any[]>([])
  const [selectedWorkspace, setSelectedWorkspace] = useState<string | null>(null)
  const [apiKeys, setApiKeys] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [showCreateKeyModal, setShowCreateKeyModal] = useState(false)
  const [showKeyModal, setShowKeyModal] = useState(false)
  const [newKeyName, setNewKeyName] = useState('')
  const [creating, setCreating] = useState(false)
  const [newlyCreatedKey, setNewlyCreatedKey] = useState<string | null>(null)

  useEffect(() => {
    loadData()
  }, [])

  useEffect(() => {
    if (selectedWorkspace) {
      loadApiKeys()
    }
  }, [selectedWorkspace])

  const loadData = async () => {
    try {
      setLoading(true)
      const { data: { session } } = await supabase.auth.getSession()
      setUser(session?.user)

      const workspacesData = await api.workspaces.list()
      setWorkspaces(workspacesData)
      
      if (workspacesData.length > 0) {
        setSelectedWorkspace(workspacesData[0].id)
      }
    } catch (err: any) {
      console.error('Failed to load data:', err)
    } finally {
      setLoading(false)
    }
  }

  const loadApiKeys = async () => {
    if (!selectedWorkspace) return

    try {
      const keys = await api.apiKeys.list(selectedWorkspace)
      setApiKeys(keys)
    } catch (err: any) {
      console.error('Failed to load API keys:', err)
    }
  }

  const handleCreateApiKey = async (e: React.FormEvent) => {
    e.preventDefault()

    if (!newKeyName.trim() || !selectedWorkspace) {
      return
    }

    try {
      setCreating(true)
      const response = await api.apiKeys.create(selectedWorkspace, {
        name: newKeyName,
      })
      
      setNewlyCreatedKey(response.api_key.key)
      setShowCreateKeyModal(false)
      setShowKeyModal(true)
      setNewKeyName('')
      loadApiKeys()
    } catch (err: any) {
      alert(err.message || 'Failed to create API key')
    } finally {
      setCreating(false)
    }
  }

  const handleRevokeKey = async (keyId: string) => {
    if (!selectedWorkspace) return

    if (!confirm('Are you sure you want to revoke this API key? This action cannot be undone.')) {
      return
    }

    try {
      await api.apiKeys.revoke(selectedWorkspace, keyId)
      loadApiKeys()
    } catch (err: any) {
      alert(err.message || 'Failed to revoke API key')
    }
  }

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text)
    alert('Copied to clipboard!')
  }

  if (loading) {
    return (
      <DashboardLayout>
        <div className="flex items-center justify-center h-64">
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent-blue mx-auto mb-4"></div>
            <p className="text-text-secondary">Loading settings...</p>
          </div>
        </div>
      </DashboardLayout>
    )
  }

  return (
    <DashboardLayout>
      <div className="space-y-8">
        {/* Page Header */}
        <div>
          <h1 className="text-4xl font-serif font-semibold text-text-primary mb-2">
            Settings
          </h1>
          <p className="text-text-secondary">
            Manage your account and API keys
          </p>
        </div>

        {/* User Profile */}
        <Card>
          <CardHeader
            title="Profile"
            description="Your account information"
          />
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-text-secondary mb-1">
                Email
              </label>
              <p className="text-text-primary">{user?.email}</p>
            </div>
            <div>
              <label className="block text-sm font-medium text-text-secondary mb-1">
                User ID
              </label>
              <p className="text-sm font-mono text-text-primary">{user?.id}</p>
            </div>
          </div>
        </Card>

        {/* API Keys */}
        <Card>
          <CardHeader
            title="API Keys"
            description="Manage API keys for CLI access and programmatic access"
            action={
              <Button
                onClick={() => setShowCreateKeyModal(true)}
                disabled={!selectedWorkspace}
              >
                Create API Key
              </Button>
            }
          />

          {/* Workspace Selector */}
          {workspaces.length > 1 && (
            <div className="mb-4">
              <label className="block text-sm font-medium text-text-primary mb-2">
                Workspace
              </label>
              <select
                value={selectedWorkspace || ''}
                onChange={(e) => setSelectedWorkspace(e.target.value)}
                className="w-full px-3 py-2 border border-border rounded-md bg-white text-text-primary focus:outline-none focus:ring-2 focus:ring-accent-blue"
              >
                {workspaces.map((workspace) => (
                  <option key={workspace.id} value={workspace.id}>
                    {workspace.name}
                  </option>
                ))}
              </select>
            </div>
          )}

          {apiKeys.length === 0 ? (
            <div className="text-center py-12 border border-border border-dashed rounded-lg">
              <p className="text-text-secondary mb-4">No API keys yet</p>
              <Button
                onClick={() => setShowCreateKeyModal(true)}
                disabled={!selectedWorkspace}
              >
                Create Your First API Key
              </Button>
            </div>
          ) : (
            <Table>
              <TableHeader>
                <tr>
                  <TableHead>Name</TableHead>
                  <TableHead>Prefix</TableHead>
                  <TableHead>Created</TableHead>
                  <TableHead>Last Used</TableHead>
                  <TableHead>Status</TableHead>
                  <TableHead>Actions</TableHead>
                </tr>
              </TableHeader>
              <TableBody>
                {apiKeys.map((key) => (
                  <TableRow key={key.id}>
                    <TableCell className="font-medium">{key.name}</TableCell>
                    <TableCell>
                      <code className="text-sm">{key.prefix}...</code>
                    </TableCell>
                    <TableCell>
                      {new Date(key.created_at).toLocaleDateString()}
                    </TableCell>
                    <TableCell>
                      {key.last_used_at
                        ? new Date(key.last_used_at).toLocaleDateString()
                        : 'Never'}
                    </TableCell>
                    <TableCell>
                      {key.revoked_at ? (
                        <Badge variant="danger">Revoked</Badge>
                      ) : (
                        <Badge variant="success">Active</Badge>
                      )}
                    </TableCell>
                    <TableCell>
                      {!key.revoked_at && (
                        <Button
                          variant="danger"
                          size="sm"
                          onClick={() => handleRevokeKey(key.id)}
                        >
                          Revoke
                        </Button>
                      )}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}

          <div className="mt-6 p-4 bg-accent-blue-light rounded-lg border border-accent-blue">
            <h4 className="font-semibold text-text-primary mb-2">
              Using API Keys with CLI
            </h4>
            <ol className="text-sm text-text-secondary space-y-1 list-decimal list-inside">
              <li>Create an API key above</li>
              <li>Run <code className="bg-white px-1 py-0.5 rounded text-text-primary">birch login http://localhost:3000</code></li>
              <li>Paste your API key when prompted</li>
              <li>Use commands like <code className="bg-white px-1 py-0.5 rounded text-text-primary">birch workspace list</code></li>
            </ol>
          </div>
        </Card>
      </div>

      {/* Create API Key Modal */}
      <Modal
        isOpen={showCreateKeyModal}
        onClose={() => setShowCreateKeyModal(false)}
        title="Create API Key"
      >
        <form onSubmit={handleCreateApiKey}>
          <Input
            label="Key Name"
            value={newKeyName}
            onChange={(e) => setNewKeyName(e.target.value)}
            placeholder="e.g., Production CLI, CI/CD Pipeline"
            required
            autoFocus
            helperText="Choose a descriptive name to identify this key"
          />

          <div className="mt-4 p-3 bg-orange-50 border border-orange-200 rounded-md">
            <p className="text-sm text-orange-800">
              <strong>Important:</strong> The API key will only be shown once. Make sure to copy it immediately.
            </p>
          </div>

          <ModalFooter>
            <Button
              type="button"
              variant="secondary"
              onClick={() => setShowCreateKeyModal(false)}
            >
              Cancel
            </Button>
            <Button type="submit" loading={creating}>
              Create Key
            </Button>
          </ModalFooter>
        </form>
      </Modal>

      {/* Show New API Key Modal */}
      <Modal
        isOpen={showKeyModal}
        onClose={() => {
          setShowKeyModal(false)
          setNewlyCreatedKey(null)
        }}
        title="API Key Created"
      >
        <div className="space-y-4">
          <div className="p-4 bg-green-50 border border-green-200 rounded-md">
            <p className="text-sm text-green-800 mb-3">
              <strong>Success!</strong> Your API key has been created. Copy it now - you won't be able to see it again.
            </p>
            <div className="flex items-center gap-2">
              <code className="flex-1 p-3 bg-white rounded border border-green-300 text-sm font-mono break-all">
                {newlyCreatedKey}
              </code>
              <Button
                variant="secondary"
                size="sm"
                onClick={() => newlyCreatedKey && copyToClipboard(newlyCreatedKey)}
              >
                Copy
              </Button>
            </div>
          </div>

          <div className="p-4 bg-accent-blue-light border border-accent-blue rounded-md">
            <h4 className="font-semibold text-text-primary mb-2">Next Steps:</h4>
            <ol className="text-sm text-text-secondary space-y-1 list-decimal list-inside">
              <li>Copy the API key above</li>
              <li>Open your terminal</li>
              <li>Run <code className="bg-white px-1 py-0.5 rounded text-text-primary">birch login http://localhost:3000</code></li>
              <li>Paste your API key when prompted</li>
            </ol>
          </div>
        </div>

        <ModalFooter>
          <Button
            onClick={() => {
              setShowKeyModal(false)
              setNewlyCreatedKey(null)
            }}
          >
            Done
          </Button>
        </ModalFooter>
      </Modal>
    </DashboardLayout>
  )
}

