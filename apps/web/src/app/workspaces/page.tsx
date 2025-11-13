'use client'

import { useEffect, useState } from 'react'
import { motion } from 'framer-motion'
import DashboardLayout from '@/components/layout/DashboardLayout'
import Card from '@/components/ui/Card'
import Badge from '@/components/ui/Badge'
import Button from '@/components/ui/Button'
import Modal, { ModalFooter } from '@/components/ui/Modal'
import Input from '@/components/ui/Input'
import { api } from '@/lib/api'
import Link from 'next/link'
import { IconFolder, IconSparkles, IconPlus } from '@tabler/icons-react'

export default function WorkspacesPage() {
  const [workspaces, setWorkspaces] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [showCreateModal, setShowCreateModal] = useState(false)
  const [creating, setCreating] = useState(false)
  const [newWorkspaceName, setNewWorkspaceName] = useState('')

  useEffect(() => {
    loadWorkspaces()
  }, [])

  const loadWorkspaces = async () => {
    try {
      setLoading(true)
      const data = await api.workspaces.list()
      setWorkspaces(data)
    } catch (err: any) {
      setError(err.message || 'Failed to load workspaces')
    } finally {
      setLoading(false)
    }
  }

  const handleCreateWorkspace = async (e: React.FormEvent) => {
    e.preventDefault()
    
    if (!newWorkspaceName.trim()) {
      return
    }

    try {
      setCreating(true)
      await api.workspaces.create({ name: newWorkspaceName })
      setShowCreateModal(false)
      setNewWorkspaceName('')
      loadWorkspaces()
    } catch (err: any) {
      alert(err.message || 'Failed to create workspace')
    } finally {
      setCreating(false)
    }
  }

  return (
    <DashboardLayout>
      <div className="space-y-6">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="flex items-center justify-between"
        >
          <div>
            <h1 className="text-4xl font-serif font-semibold bg-gradient-to-r from-accent-blue to-accent-purple bg-clip-text text-transparent mb-2">
              Workspaces
            </h1>
            <p className="text-text-secondary text-lg">
              Manage your workspaces and organize your credentials
            </p>
          </div>
          <Button variant="gradient" onClick={() => setShowCreateModal(true)}>
            <IconPlus className="w-5 h-5" />
            Create Workspace
          </Button>
        </motion.div>

        {error && (
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            className="bg-red-50 border border-red-200 rounded-xl p-4 flex items-center gap-3"
          >
            <div className="w-10 h-10 rounded-lg bg-red-100 flex items-center justify-center shrink-0">
              <svg className="w-5 h-5 text-red-600" fill="currentColor" viewBox="0 0 20 20">
                <path
                  fillRule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                  clipRule="evenodd"
                />
              </svg>
            </div>
            <p className="text-red-800 font-medium">{error}</p>
          </motion.div>
        )}

        {loading && (
          <div className="flex items-center justify-center h-64">
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="text-center"
            >
              <div className="relative">
                <div className="animate-spin rounded-full h-16 w-16 border-4 border-paper-darker mx-auto mb-4"></div>
                <div className="animate-spin rounded-full h-16 w-16 border-4 border-t-accent-blue border-r-accent-purple absolute inset-0 mx-auto"></div>
              </div>
              <p className="text-text-secondary font-medium">Loading workspaces...</p>
            </motion.div>
          </div>
        )}

        {!loading && workspaces.length > 0 && (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {workspaces.map((workspace, index) => (
              <motion.div
                key={workspace.id}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: index * 0.05 }}
              >
                <Link href={`/workspaces/${workspace.id}`}>
                  <Card hover>
                    <div className="flex items-start justify-between mb-4">
                      <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-accent-blue to-accent-purple flex items-center justify-center shadow-modern-md">
                        <IconFolder className="w-6 h-6 text-white" />
                      </div>
                      <Badge variant="info">{workspace.plan_tier}</Badge>
                    </div>
                    
                    <h3 className="text-xl font-serif font-semibold text-text-primary mb-2">
                      {workspace.name}
                    </h3>
                    
                    <p className="text-sm text-text-secondary">
                      Created {new Date(workspace.created_at).toLocaleDateString()}
                    </p>
                  </Card>
                </Link>
              </motion.div>
            ))}
          </div>
        )}

        {!loading && workspaces.length === 0 && (
          <Card>
            <div className="text-center py-16">
              <motion.div
                initial={{ scale: 0.9, opacity: 0 }}
                animate={{ scale: 1, opacity: 1 }}
              >
                <div className="w-20 h-20 bg-gradient-to-br from-accent-blue to-accent-purple rounded-2xl mx-auto mb-6 flex items-center justify-center">
                  <IconSparkles className="w-10 h-10 text-white" />
                </div>
                <h3 className="text-2xl font-serif font-semibold text-text-primary mb-2">
                  No workspaces yet
                </h3>
                <p className="text-text-secondary mb-6 max-w-md mx-auto">
                  Create your first workspace to start organizing your credentials and policies.
                </p>
                <Button variant="gradient" size="lg" onClick={() => setShowCreateModal(true)}>
                  <IconPlus className="w-5 h-5" />
                  Create Your First Workspace
                </Button>
              </motion.div>
            </div>
          </Card>
        )}
      </div>

      <Modal
        isOpen={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        title="Create Workspace"
      >
        <form onSubmit={handleCreateWorkspace}>
          <Input
            label="Workspace Name"
            value={newWorkspaceName}
            onChange={(e) => setNewWorkspaceName(e.target.value)}
            placeholder="e.g., Production, Staging, Development"
            required
            autoFocus
          />

          <ModalFooter>
            <Button
              type="button"
              variant="secondary"
              onClick={() => setShowCreateModal(false)}
            >
              Cancel
            </Button>
            <Button type="submit" variant="gradient" loading={creating}>
              <IconPlus className="w-4 h-4" />
              Create Workspace
            </Button>
          </ModalFooter>
        </form>
      </Modal>
    </DashboardLayout>
  )
}
