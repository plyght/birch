'use client'

import { useEffect, useState } from 'react'
import { useParams } from 'next/navigation'
import DashboardLayout from '@/components/layout/DashboardLayout'
import Card, { CardHeader } from '@/components/ui/Card'
import Badge from '@/components/ui/Badge'
import Button from '@/components/ui/Button'
import { api } from '@/lib/api'
import Link from 'next/link'

export default function WorkspaceDetailPage() {
  const params = useParams()
  const workspaceId = params.id as string
  const [workspace, setWorkspace] = useState<any>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    loadWorkspace()
  }, [workspaceId])

  const loadWorkspace = async () => {
    try {
      setLoading(true)
      const data = await api.workspaces.get(workspaceId)
      setWorkspace(data)
    } catch (err: any) {
      setError(err.message || 'Failed to load workspace')
    } finally {
      setLoading(false)
    }
  }

  if (loading) {
    return (
      <DashboardLayout>
        <div className="flex items-center justify-center h-64">
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-accent-blue mx-auto mb-4"></div>
            <p className="text-text-secondary">Loading workspace...</p>
          </div>
        </div>
      </DashboardLayout>
    )
  }

  if (error || !workspace) {
    return (
      <DashboardLayout>
        <div className="bg-red-50 border border-red-200 rounded-lg p-4">
          <p className="text-red-800">{error || 'Workspace not found'}</p>
        </div>
      </DashboardLayout>
    )
  }

  return (
    <DashboardLayout>
      <div className="space-y-6">
        {/* Page Header */}
        <div className="flex items-center justify-between">
          <div>
            <div className="flex items-center gap-3 mb-2">
              <h1 className="text-4xl font-serif font-semibold text-text-primary">
                {workspace.name}
              </h1>
              <Badge variant="info">{workspace.plan_tier}</Badge>
            </div>
            <p className="text-text-secondary">
              Created {new Date(workspace.created_at).toLocaleDateString()}
            </p>
          </div>
        </div>

        {/* Overview Cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <Card>
            <h3 className="font-semibold text-text-primary mb-2">Credentials</h3>
            <p className="text-3xl font-serif font-semibold text-accent-blue">0</p>
            <p className="text-sm text-text-secondary mt-2">
              Stored credentials
            </p>
          </Card>

          <Card>
            <h3 className="font-semibold text-text-primary mb-2">Policies</h3>
            <p className="text-3xl font-serif font-semibold text-accent-blue">0</p>
            <p className="text-sm text-text-secondary mt-2">
              Active rotation policies
            </p>
          </Card>

          <Card>
            <h3 className="font-semibold text-text-primary mb-2">Members</h3>
            <p className="text-3xl font-serif font-semibold text-accent-blue">1</p>
            <p className="text-sm text-text-secondary mt-2">
              Team members
            </p>
          </Card>
        </div>

        {/* Quick Actions */}
        <Card>
          <CardHeader
            title="Quick Actions"
            description="Manage this workspace"
          />
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <Link href={`/credentials?workspace=${workspaceId}`}>
              <div className="p-4 border border-border rounded-lg hover:border-accent-blue hover:bg-accent-blue-light transition-colors cursor-pointer">
                <h4 className="font-semibold text-text-primary mb-2">
                  Manage Credentials
                </h4>
                <p className="text-sm text-text-secondary">
                  Add or configure provider credentials
                </p>
              </div>
            </Link>

            <Link href={`/policies?workspace=${workspaceId}`}>
              <div className="p-4 border border-border rounded-lg hover:border-accent-blue hover:bg-accent-blue-light transition-colors cursor-pointer">
                <h4 className="font-semibold text-text-primary mb-2">
                  Create Policy
                </h4>
                <p className="text-sm text-text-secondary">
                  Define rotation policies
                </p>
              </div>
            </Link>

            <Link href={`/team?workspace=${workspaceId}`}>
              <div className="p-4 border border-border rounded-lg hover:border-accent-blue hover:bg-accent-blue-light transition-colors cursor-pointer">
                <h4 className="font-semibold text-text-primary mb-2">
                  Manage Team
                </h4>
                <p className="text-sm text-text-secondary">
                  Invite and manage members
                </p>
              </div>
            </Link>
          </div>
        </Card>
      </div>
    </DashboardLayout>
  )
}

