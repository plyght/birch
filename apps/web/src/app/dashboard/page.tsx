'use client'

import { useEffect, useState } from 'react'
import { motion } from 'framer-motion'
import DashboardLayout from '@/components/layout/DashboardLayout'
import Card, { CardHeader } from '@/components/ui/Card'
import Badge from '@/components/ui/Badge'
import Button from '@/components/ui/Button'
import { api } from '@/lib/api'
import Link from 'next/link'
import {
  IconFolder,
  IconKey,
  IconShieldCheck,
  IconRotate,
  IconSparkles,
  IconArrowRight,
} from '@tabler/icons-react'

export default function DashboardPage() {
  const [workspaces, setWorkspaces] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    const loadData = async () => {
      try {
        const data = await api.workspaces.list()
        setWorkspaces(data)
      } catch (err: any) {
        setError(err.message || 'Failed to load dashboard data')
      } finally {
        setLoading(false)
      }
    }

    loadData()
  }, [])

  if (loading) {
    return (
      <DashboardLayout>
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
            <p className="text-text-secondary font-medium">Loading dashboard...</p>
          </motion.div>
        </div>
      </DashboardLayout>
    )
  }

  return (
    <DashboardLayout>
      <div className="space-y-8">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
        >
          <h1 className="text-4xl font-serif font-semibold bg-gradient-to-r from-accent-blue to-accent-purple bg-clip-text text-transparent mb-2">
            Dashboard
          </h1>
          <p className="text-text-secondary text-lg">
            Welcome to Birch. Manage your credentials, policies, and secret rotations.
          </p>
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

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          {[
            {
              label: 'Workspaces',
              value: workspaces.length,
              icon: IconFolder,
              gradient: 'from-blue-500 to-blue-600',
              bg: 'bg-blue-100',
              textColor: 'text-blue-600',
            },
            {
              label: 'Credentials',
              value: 0,
              icon: IconKey,
              gradient: 'from-green-500 to-green-600',
              bg: 'bg-green-100',
              textColor: 'text-green-600',
            },
            {
              label: 'Policies',
              value: 0,
              icon: IconShieldCheck,
              gradient: 'from-purple-500 to-purple-600',
              bg: 'bg-purple-100',
              textColor: 'text-purple-600',
            },
            {
              label: 'Rotations',
              value: 0,
              icon: IconRotate,
              gradient: 'from-orange-500 to-orange-600',
              bg: 'bg-orange-100',
              textColor: 'text-orange-600',
            },
          ].map((metric, index) => (
            <motion.div
              key={metric.label}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: index * 0.1 }}
            >
              <Card hover gradient>
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-sm text-text-secondary mb-2">{metric.label}</p>
                    <p className="text-4xl font-serif font-bold text-text-primary">
                      {metric.value}
                    </p>
                  </div>
                  <div
                    className={`w-14 h-14 rounded-2xl bg-gradient-to-br ${metric.gradient} flex items-center justify-center text-white shadow-modern-lg`}
                  >
                    <metric.icon className="w-7 h-7" />
                  </div>
                </div>
              </Card>
            </motion.div>
          ))}
        </div>

        <Card>
          <CardHeader
            title="Quick Actions"
            description="Get started with common tasks"
            icon={<IconSparkles className="w-5 h-5" />}
          />
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {[
              {
                title: 'Create Workspace',
                description: 'Set up a new workspace for your team',
                href: '/workspaces',
                icon: IconFolder,
              },
              {
                title: 'Add Credentials',
                description: 'Store and manage provider credentials',
                href: '/credentials',
                icon: IconKey,
              },
              {
                title: 'Create Policy',
                description: 'Define rotation policies and schedules',
                href: '/policies',
                icon: IconShieldCheck,
              },
            ].map((action, index) => (
              <motion.div
                key={action.title}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.4 + index * 0.1 }}
              >
                <Link href={action.href}>
                  <div className="group p-5 border border-border-light rounded-xl hover:border-accent-blue hover:shadow-modern-lg transition-all duration-200 cursor-pointer bg-gradient-to-br from-white to-paper-dark">
                    <div className="flex items-start gap-4">
                      <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-accent-blue to-accent-purple flex items-center justify-center text-white group-hover:scale-110 transition-transform duration-200">
                        <action.icon className="w-5 h-5" />
                      </div>
                      <div className="flex-1">
                        <h4 className="font-semibold text-text-primary mb-1 group-hover:text-accent-blue transition-colors">
                          {action.title}
                        </h4>
                        <p className="text-sm text-text-secondary">{action.description}</p>
                      </div>
                      <IconArrowRight className="w-5 h-5 text-text-tertiary group-hover:text-accent-blue group-hover:translate-x-1 transition-all" />
                    </div>
                  </div>
                </Link>
              </motion.div>
            ))}
          </div>
        </Card>

        {workspaces.length > 0 && (
          <Card>
            <CardHeader
              title="Your Workspaces"
              description="Manage your workspaces"
              action={
                <Link href="/workspaces">
                  <Button variant="secondary" size="sm">
                    View All
                  </Button>
                </Link>
              }
            />
            <div className="space-y-3">
              {workspaces.slice(0, 5).map((workspace, index) => (
                <motion.div
                  key={workspace.id}
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: 0.6 + index * 0.05 }}
                >
                  <Link
                    href={`/workspaces/${workspace.id}`}
                    className="block p-4 border border-border-light rounded-xl hover:border-accent-blue hover:shadow-modern-md transition-all duration-200 bg-gradient-to-br from-white to-paper-dark"
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center gap-3">
                        <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-accent-blue to-accent-purple flex items-center justify-center text-white">
                          <IconFolder className="w-5 h-5" />
                        </div>
                        <div>
                          <h4 className="font-semibold text-text-primary">
                            {workspace.name}
                          </h4>
                          <p className="text-sm text-text-secondary mt-0.5">
                            Created {new Date(workspace.created_at).toLocaleDateString()}
                          </p>
                        </div>
                      </div>
                      <Badge variant="info">{workspace.plan_tier}</Badge>
                    </div>
                  </Link>
                </motion.div>
              ))}
            </div>
          </Card>
        )}

        {workspaces.length === 0 && (
          <Card>
            <div className="text-center py-16">
              <motion.div
                initial={{ scale: 0.9, opacity: 0 }}
                animate={{ scale: 1, opacity: 1 }}
                transition={{ delay: 0.3 }}
              >
                <div className="w-20 h-20 bg-gradient-to-br from-accent-blue to-accent-purple rounded-2xl mx-auto mb-6 flex items-center justify-center">
                  <IconSparkles className="w-10 h-10 text-white" />
                </div>
                <h3 className="text-2xl font-serif font-semibold text-text-primary mb-2">
                  Welcome to Birch
                </h3>
                <p className="text-text-secondary mb-6 max-w-md mx-auto">
                  Get started by creating your first workspace. Workspaces help you organize credentials and policies for different teams or projects.
                </p>
                <Link href="/workspaces">
                  <Button variant="gradient" size="lg">
                    Create Your First Workspace
                  </Button>
                </Link>
              </motion.div>
            </div>
          </Card>
        )}
      </div>
    </DashboardLayout>
  )
}
